"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.EscrowFactory = void 0;
const tslib_1 = require("tslib");
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const ethers_1 = require("ethers");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
class EscrowFactory {
    constructor(address) {
        this.address = address;
    }
    /**
     * See https://github.com/1inch/cross-chain-swap/blob/03d99b9604d8f7a5a396720fbe1059f7d94db762/contracts/libraries/ProxyHashLib.sol#L14
     */
    static calcProxyBytecodeHash(impl) {
        return (0, ethers_1.keccak256)(`0x3d602d80600a3d3981f3363d3d373d3d3d363d73${(0, byte_utils_1.trim0x)(impl.toString())}5af43d82803e903d91602b57fd5bf3`);
    }
    /**
     * Calculate address of escrow contract
     *
     * @return escrow address at same the chain as `this.address`
     */
    getEscrowAddress(
    /**
     * @see Immutables.hash
     */
    immutablesHash, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress) {
        (0, assert_1.default)((0, byte_utils_1.isHexBytes)(immutablesHash) && (0, byte_utils_1.getBytesCount)(immutablesHash) === 32n, 'invalid hash');
        return new fusion_sdk_1.Address((0, ethers_1.getCreate2Address)(this.address.toString(), immutablesHash, EscrowFactory.calcProxyBytecodeHash(implementationAddress)));
    }
    /**
     * Calculates source escrow address for given params
     *
     * Make sure you call it on source chain escrow factory
     */
    getSrcEscrowAddress(
    /**
     * From `SrcEscrowCreated` event (with correct timeLock.deployedAt)
     */
    srcImmutables, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress) {
        return this.getEscrowAddress(srcImmutables.hash(), implementationAddress);
    }
    /**
     * Calculates destination escrow address for given params
     *
     * Make sure you call it on destination chain escrow factory
     */
    getDstEscrowAddress(
    /**
     * From `SrcEscrowCreated` event
     */
    srcImmutables, 
    /**
     * From `SrcEscrowCreated` event
     */
    complement, 
    /**
     * Block time when event `DstEscrowCreated` produced
     */
    blockTime, 
    /**
     * Taker from `DstEscrowCreated` event
     */
    taker, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress) {
        return this.getEscrowAddress(srcImmutables
            .withComplement(complement)
            .withTaker(taker)
            .withDeployedAt(blockTime)
            .hash(), implementationAddress);
    }
    getMultipleFillInteraction(proof, idx, secretHash) {
        const data = ethers_1.AbiCoder.defaultAbiCoder().encode([
            `(
                        bytes32[] proof,
                        uint256 idx,
                        bytes32 secretHash,
                    )`
        ], [{ proof, idx, secretHash }]);
        const dataNoOffset = (0, byte_utils_1.add0x)(data.slice(66));
        return new fusion_sdk_1.Interaction(this.address, dataNoOffset);
    }
}
exports.EscrowFactory = EscrowFactory;
//# sourceMappingURL=escrow-factory.js.map