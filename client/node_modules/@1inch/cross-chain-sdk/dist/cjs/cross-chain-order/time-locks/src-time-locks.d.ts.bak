import { BaseTimeLock } from './base-time-lock';
export declare enum SrcStage {
    FinalityLock = 0,
    PrivateWithdrawal = 1,
    PublicWithdrawal = 2,
    PrivateCancellation = 3,
    PublicCancellation = 4
}
/**
 * Class to access source chain related time-locks
 *
 * Intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation | public cancellation |
 * ^deployedAt
 */
export declare class SrcTimeLocks extends BaseTimeLock {
    private readonly _withdrawal;
    private readonly _publicWithdrawal;
    private readonly _cancellation;
    private readonly _publicCancellation;
    private constructor();
    /** Timestamp at which ends `finality lock` and starts `private withdrawal` */
    get privateWithdrawal(): bigint;
    /** Timestamp at which ends `private withdrawal` and starts `public withdrawal` */
    get publicWithdrawal(): bigint;
    /** Timestamp at which ends `public withdrawal` and starts `private cancellation` */
    get privateCancellation(): bigint;
    /** Timestamp at which ends `private cancellation` and starts `public cancellation` */
    get publicCancellation(): bigint;
    static new(params: {
        /** Escrow deploy timestamp */
        deployedAt: bigint;
        /** Delay from `deployedAt` at which ends `finality lock` and starts `private withdrawal` */
        withdrawal: bigint;
        /** Delay from `deployedAt` at which ends `private withdrawal` and starts `public withdrawal` */
        publicWithdrawal: bigint;
        /** Delay from `deployedAt` at which ends `public withdrawal` and starts `private cancellation` */
        cancellation: bigint;
        /** Delay from `deployedAt` at which ends `private cancellation` and starts `public cancellation` */
        publicCancellation: bigint;
    }): SrcTimeLocks;
    /**
     * Return true, when `time` in `finality lock` interval
     *
     * @param time default is `now()`
     */
    isFinalityLock(time?: bigint): boolean;
    /**
     * Return true, when `time` in `private withdrawal` interval
     *
     * @param time default is `now()`
     */
    isPrivateWithdrawal(time?: bigint): boolean;
    /**
     * Return true, when `time` in `public withdrawal` interval
     *
     * @param time default is `now()`
     */
    isPublicWithdrawal(time?: bigint): boolean;
    /**
     * Return true, when `time` in `private cancellation` interval
     *
     * @param time default is `now()`
     */
    isPrivateCancellation(time?: bigint): boolean;
    /**
     * Return true, when `time` in `public cancellation` interval
     *
     * @param time default is `now()`
     */
    isPublicCancellation(time?: bigint): boolean;
    getStage(time?: bigint): SrcStage;
}
