import { Address, Interaction } from '@1inch/fusion-sdk';
import { AbiCoder, getCreate2Address, keccak256 } from 'ethers';
import { add0x, getBytesCount, isHexBytes, trim0x } from '@1inch/byte-utils';
import assert from 'assert';
export class EscrowFactory {
    constructor(address) {
        this.address = address;
    }
    /**
     * See https://github.com/1inch/cross-chain-swap/blob/03d99b9604d8f7a5a396720fbe1059f7d94db762/contracts/libraries/ProxyHashLib.sol#L14
     */
    static calcProxyBytecodeHash(impl) {
        return keccak256(`0x3d602d80600a3d3981f3363d3d373d3d3d363d73${trim0x(impl.toString())}5af43d82803e903d91602b57fd5bf3`);
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
        assert(isHexBytes(immutablesHash) && getBytesCount(immutablesHash) === 32n, 'invalid hash');
        return new Address(getCreate2Address(this.address.toString(), immutablesHash, EscrowFactory.calcProxyBytecodeHash(implementationAddress)));
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
        const data = AbiCoder.defaultAbiCoder().encode([
            `(
                        bytes32[] proof,
                        uint256 idx,
                        bytes32 secretHash,
                    )`
        ], [{ proof, idx, secretHash }]);
        const dataNoOffset = add0x(data.slice(66));
        return new Interaction(this.address, dataNoOffset);
    }
}
//# sourceMappingURL=escrow-factory.js.map