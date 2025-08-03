"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.EscrowExtension = void 0;
const tslib_1 = require("tslib");
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const ethers_1 = require("ethers");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const time_locks_1 = require("./time-locks/time-locks");
const hash_lock_1 = require("./hash-lock/hash-lock");
/**
 * Same as FusionExtension, but with extra data at the end
 * Extra data contains next fields:
 * - hashLock
 * - dstChainId
 * - dstToken
 * - srcSafetyDeposit
 * - dstSafetyDeposit
 * - timeLocks
 */
class EscrowExtension extends fusion_sdk_1.FusionExtension {
    constructor(address, auctionDetails, postInteractionData, makerPermit, hashLockInfo, dstChainId, dstToken, srcSafetyDeposit, dstSafetyDeposit, timeLocks) {
        (0, assert_1.default)(srcSafetyDeposit <= byte_utils_1.UINT_128_MAX);
        (0, assert_1.default)(srcSafetyDeposit <= byte_utils_1.UINT_128_MAX);
        super(address, auctionDetails, postInteractionData, makerPermit);
        this.hashLockInfo = hashLockInfo;
        this.dstChainId = dstChainId;
        this.dstToken = dstToken;
        this.srcSafetyDeposit = srcSafetyDeposit;
        this.dstSafetyDeposit = dstSafetyDeposit;
        this.timeLocks = timeLocks;
        if (this.dstToken.isZero()) {
            this.dstToken = fusion_sdk_1.Address.NATIVE_CURRENCY;
        }
    }
    /**
     * Create EscrowExtension from bytes
     * @param bytes 0x prefixed bytes
     */
    static decode(bytes) {
        const extension = fusion_sdk_1.Extension.decode(bytes);
        return EscrowExtension.fromExtension(extension);
    }
    static fromExtension(extension) {
        const fusionExt = fusion_sdk_1.FusionExtension.fromExtension(new fusion_sdk_1.Extension({
            ...extension,
            postInteraction: extension.postInteraction.slice(0, -EscrowExtension.EXTRA_DATA_LENGTH)
        }));
        const extra = EscrowExtension.decodeExtraData('0x' +
            extension.postInteraction.slice(-EscrowExtension.EXTRA_DATA_LENGTH));
        return new EscrowExtension(fusionExt.address, fusionExt.auctionDetails, fusionExt.postInteractionData, fusionExt.makerPermit, extra.hashLock, extra.dstChainId, extra.dstToken, extra.srcSafetyDeposit, extra.dstSafetyDeposit, extra.timeLocks);
    }
    /**
     * Decode escrow data not related to fusion
     *
     * @param bytes 0x prefixed bytes
     */
    static decodeExtraData(bytes) {
        const [hashLock, dstChainId, dstToken, safetyDeposit, timeLocks] = ethers_1.AbiCoder.defaultAbiCoder().decode(EscrowExtension.EXTRA_DATA_TYPES, bytes);
        const safetyDepositBN = new byte_utils_1.BN(safetyDeposit);
        return {
            hashLock: hash_lock_1.HashLock.fromString(hashLock),
            dstChainId: Number(dstChainId),
            dstToken: new fusion_sdk_1.Address(dstToken),
            dstSafetyDeposit: safetyDepositBN.getMask(new byte_utils_1.BitMask(0n, 128n))
                .value,
            srcSafetyDeposit: safetyDepositBN.getMask(new byte_utils_1.BitMask(128n, 256n))
                .value,
            timeLocks: time_locks_1.TimeLocks.fromBigInt(timeLocks)
        };
    }
    build() {
        const baseExt = super.build();
        return new fusion_sdk_1.Extension({
            ...baseExt,
            postInteraction: baseExt.postInteraction + (0, byte_utils_1.trim0x)(this.encodeExtraData())
        });
    }
    encodeExtraData() {
        const dstToken = this.dstToken.isNative()
            ? fusion_sdk_1.Address.ZERO_ADDRESS
            : this.dstToken;
        return ethers_1.AbiCoder.defaultAbiCoder().encode(EscrowExtension.EXTRA_DATA_TYPES, [
            this.hashLockInfo.toString(),
            this.dstChainId,
            dstToken.toString(),
            (this.srcSafetyDeposit << 128n) | this.dstSafetyDeposit,
            this.timeLocks.build()
        ]);
    }
}
exports.EscrowExtension = EscrowExtension;
EscrowExtension.EXTRA_DATA_TYPES = [
    hash_lock_1.HashLock.Web3Type,
    'uint256', // dst chain id
    'address', // dst token
    'uint256', // src/dst safety deposit
    time_locks_1.TimeLocks.Web3Type
];
EscrowExtension.EXTRA_DATA_LENGTH = 160 * 2; // 160 bytes, so 320 hex chars
//# sourceMappingURL=escrow-extension.js.map