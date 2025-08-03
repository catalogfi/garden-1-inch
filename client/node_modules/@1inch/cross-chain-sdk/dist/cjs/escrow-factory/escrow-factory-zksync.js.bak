"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.EscrowFactoryZksync = void 0;
const tslib_1 = require("tslib");
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const ethers_1 = require("ethers");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const escrow_factory_1 = require("./escrow-factory");
class EscrowFactoryZksync extends escrow_factory_1.EscrowFactory {
    /**
     * Calculate address of escrow contract in ZkSync Era
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
        const inputHash = (0, ethers_1.keccak256)(ethers_1.AbiCoder.defaultAbiCoder().encode(['address'], [implementationAddress.toString()]));
        const concatenatedData = (0, ethers_1.concat)([
            EscrowFactoryZksync.create2Prefix,
            (0, byte_utils_1.add0x)((0, byte_utils_1.trim0x)(this.address.toString()).padStart(64, '0')),
            immutablesHash,
            EscrowFactoryZksync.minimalProxyBytecodeHash,
            inputHash
        ]);
        return new fusion_sdk_1.Address((0, byte_utils_1.add0x)((0, ethers_1.keccak256)(concatenatedData).slice(-40)));
    }
}
exports.EscrowFactoryZksync = EscrowFactoryZksync;
EscrowFactoryZksync.create2Prefix = '0x2020dba91b30cc0006188af794c2fb30dd8520db7e2c088b7fc7c103c00ca494';
/**
 * ZkSync proxy bytecode do not depends on implementation address
 *
 * @see proxy example - https://explorer.zksync.io/address/0xd5317Ded4FBb98526AdD35A15d63cFBFB929efc7
 */
EscrowFactoryZksync.minimalProxyBytecodeHash = '0x01000035492ceb24a47d861a8fd7e65b117f2eb5bf6453e191ba770c70ca7f43';
//# sourceMappingURL=escrow-factory-zksync.js.map