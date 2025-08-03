import { BaseTimeLock } from './base-time-lock';
export declare enum DstStage {
    FinalityLock = 0,
    PrivateWithdrawal = 1,
    PublicWithdrawal = 2,
    PrivateCancellation = 3
}
/**
 * Class to access destination chain related time-locks
 *
 * Intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation |
 * ^deployedAt
 */
export declare class DstTimeLocks extends BaseTimeLock {
    private readonly _withdrawal;
    private readonly _publicWithdrawal;
    private readonly _cancellation;
    private constructor();
    /** Timestamp at which ends `finality lock` and starts `private withdrawal` */
    get privateWithdrawal(): bigint;
    /** Timestamp at which ends `private withdrawal` and starts `public withdrawal` */
    get publicWithdrawal(): bigint;
    /** Timestamp at which ends `public withdrawal` and starts `private cancellation` */
    get privateCancellation(): bigint;
    static new(params: {
        /** Escrow deploy timestamp */
        deployedAt: bigint;
        /** Delay from `deployedAt` at which ends `finality lock` and starts `private withdrawal` */
        withdrawal: bigint;
        /** Delay from `deployedAt` at which ends `private withdrawal` and starts `public withdrawal` */
        publicWithdrawal: bigint;
        /** Delay from `deployedAt` at which ends `public withdrawal` and starts `private cancellation` */
        cancellation: bigint;
    }): DstTimeLocks;
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
    getStage(time?: bigint): DstStage;
}
