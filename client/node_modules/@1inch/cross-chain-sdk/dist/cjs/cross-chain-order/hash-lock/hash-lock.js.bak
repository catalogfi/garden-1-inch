"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HashLock = void 0;
const tslib_1 = require("tslib");
const ethers_1 = require("ethers");
const merkle_tree_1 = require("@openzeppelin/merkle-tree");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
class HashLock {
    constructor(val) {
        this.value = val;
    }
    static hashSecret(secret) {
        (0, assert_1.default)((0, byte_utils_1.isHexBytes)(secret) && (0, byte_utils_1.getBytesCount)(secret) === 32n, 'secret length must be 32 bytes hex encoded');
        return (0, ethers_1.keccak256)(secret);
    }
    static getMerkleLeaves(secrets) {
        return HashLock.getMerkleLeavesFromSecretHashes(secrets.map(HashLock.hashSecret));
    }
    static getMerkleLeavesFromSecretHashes(secretHashes) {
        return secretHashes.map((s, idx) => (0, ethers_1.solidityPackedKeccak256)(['uint64', 'bytes32'], [idx, s]));
    }
    static getProof(leaves, idx) {
        return merkle_tree_1.SimpleMerkleTree.of(leaves).getProof(idx);
    }
    static fromString(value) {
        (0, assert_1.default)((0, byte_utils_1.isHexBytes)(value) && (0, byte_utils_1.getBytesCount)(value) === 32n, 'HashLock value must be bytes32 hex encoded');
        return new HashLock(value);
    }
    /**
     * Create HashLock from keccak256 hash of secret
     */
    static forSingleFill(secret) {
        return new HashLock(HashLock.hashSecret(secret));
    }
    static forMultipleFills(leaves) {
        (0, assert_1.default)(leaves.length > 2, 'leaves array must be greater than 2. Or use HashLock.forSingleFill');
        const root = merkle_tree_1.SimpleMerkleTree.of(leaves).root;
        const rootWithCount = byte_utils_1.BN.fromHex(root).setMask(new byte_utils_1.BitMask(240n, 256n), BigInt(leaves.length - 1));
        return new HashLock(rootWithCount.toHex(64));
    }
    /**
     * Only use if HashLockInfo is for multiple fill order
     * Otherwise garbage will be returned
     *
     */
    getPartsCount() {
        return new byte_utils_1.BN(BigInt(this.value)).getMask(new byte_utils_1.BitMask(240n, 256n)).value;
    }
    toString() {
        return this.value;
    }
    eq(other) {
        return this.value === other.value;
    }
}
exports.HashLock = HashLock;
HashLock.Web3Type = 'bytes32';
//# sourceMappingURL=hash-lock.js.map