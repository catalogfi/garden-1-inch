import { Address, Extension, FusionExtension } from '@1inch/fusion-sdk';
import { AbiCoder } from 'ethers';
import { BitMask, BN, trim0x, UINT_128_MAX } from '@1inch/byte-utils';
import assert from 'assert';
import { TimeLocks } from './time-locks/time-locks';
import { HashLock } from './hash-lock/hash-lock';
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
export class EscrowExtension extends FusionExtension {
    constructor(address, auctionDetails, postInteractionData, makerPermit, hashLockInfo, dstChainId, dstToken, srcSafetyDeposit, dstSafetyDeposit, timeLocks) {
        assert(srcSafetyDeposit <= UINT_128_MAX);
        assert(srcSafetyDeposit <= UINT_128_MAX);
        super(address, auctionDetails, postInteractionData, makerPermit);
        this.hashLockInfo = hashLockInfo;
        this.dstChainId = dstChainId;
        this.dstToken = dstToken;
        this.srcSafetyDeposit = srcSafetyDeposit;
        this.dstSafetyDeposit = dstSafetyDeposit;
        this.timeLocks = timeLocks;
        if (this.dstToken.isZero()) {
            this.dstToken = Address.NATIVE_CURRENCY;
        }
    }
    /**
     * Create EscrowExtension from bytes
     * @param bytes 0x prefixed bytes
     */
    static decode(bytes) {
        const extension = Extension.decode(bytes);
        return EscrowExtension.fromExtension(extension);
    }
    static fromExtension(extension) {
        const fusionExt = FusionExtension.fromExtension(new Extension({
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
        const [hashLock, dstChainId, dstToken, safetyDeposit, timeLocks] = AbiCoder.defaultAbiCoder().decode(EscrowExtension.EXTRA_DATA_TYPES, bytes);
        const safetyDepositBN = new BN(safetyDeposit);
        return {
            hashLock: HashLock.fromString(hashLock),
            dstChainId: Number(dstChainId),
            dstToken: new Address(dstToken),
            dstSafetyDeposit: safetyDepositBN.getMask(new BitMask(0n, 128n))
                .value,
            srcSafetyDeposit: safetyDepositBN.getMask(new BitMask(128n, 256n))
                .value,
            timeLocks: TimeLocks.fromBigInt(timeLocks)
        };
    }
    build() {
        const baseExt = super.build();
        return new Extension({
            ...baseExt,
            postInteraction: baseExt.postInteraction + trim0x(this.encodeExtraData())
        });
    }
    encodeExtraData() {
        const dstToken = this.dstToken.isNative()
            ? Address.ZERO_ADDRESS
            : this.dstToken;
        return AbiCoder.defaultAbiCoder().encode(EscrowExtension.EXTRA_DATA_TYPES, [
            this.hashLockInfo.toString(),
            this.dstChainId,
            dstToken.toString(),
            (this.srcSafetyDeposit << 128n) | this.dstSafetyDeposit,
            this.timeLocks.build()
        ]);
    }
}
EscrowExtension.EXTRA_DATA_TYPES = [
    HashLock.Web3Type,
    'uint256', // dst chain id
    'address', // dst token
    'uint256', // src/dst safety deposit
    TimeLocks.Web3Type
];
EscrowExtension.EXTRA_DATA_LENGTH = 160 * 2; // 160 bytes, so 320 hex chars
//# sourceMappingURL=escrow-extension.js.map