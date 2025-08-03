import { Address, Extension, FusionExtension, Interaction, SettlementPostInteractionData, AuctionDetails, NetworkEnum } from '@1inch/fusion-sdk';
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
export declare class EscrowExtension extends FusionExtension {
    readonly hashLockInfo: HashLock;
    readonly dstChainId: NetworkEnum;
    readonly dstToken: Address;
    readonly srcSafetyDeposit: bigint;
    readonly dstSafetyDeposit: bigint;
    readonly timeLocks: TimeLocks;
    private static EXTRA_DATA_TYPES;
    private static EXTRA_DATA_LENGTH;
    constructor(address: Address, auctionDetails: AuctionDetails, postInteractionData: SettlementPostInteractionData, makerPermit: Interaction | undefined, hashLockInfo: HashLock, dstChainId: NetworkEnum, dstToken: Address, srcSafetyDeposit: bigint, dstSafetyDeposit: bigint, timeLocks: TimeLocks);
    /**
     * Create EscrowExtension from bytes
     * @param bytes 0x prefixed bytes
     */
    static decode(bytes: string): EscrowExtension;
    static fromExtension(extension: Extension): EscrowExtension;
    /**
     * Decode escrow data not related to fusion
     *
     * @param bytes 0x prefixed bytes
     */
    private static decodeExtraData;
    build(): Extension;
    private encodeExtraData;
}
