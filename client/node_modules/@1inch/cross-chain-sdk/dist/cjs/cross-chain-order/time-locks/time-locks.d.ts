import { SrcTimeLocks } from './src-time-locks';
import { DstTimeLocks } from './dst-time-locks';
/**
 * Contains the duration of each stage of swap for source and destination chain
 *
 * Source chain intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation | public cancellation |
 * ^deployedAt
 *
 * Destination chain intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation |
 * ^deployedAt
 *
 * @see SrcTimeLocks
 * @see DstTimeLocks
 */
export declare class TimeLocks {
    private readonly _srcWithdrawal;
    private readonly _srcPublicWithdrawal;
    private readonly _srcCancellation;
    private readonly _srcPublicCancellation;
    private readonly _dstWithdrawal;
    private readonly _dstPublicWithdrawal;
    private readonly _dstCancellation;
    private _deployedAt;
    static DEFAULT_RESCUE_DELAY: bigint;
    static Web3Type: string;
    protected constructor(_srcWithdrawal: bigint, _srcPublicWithdrawal: bigint, _srcCancellation: bigint, _srcPublicCancellation: bigint, _dstWithdrawal: bigint, _dstPublicWithdrawal: bigint, _dstCancellation: bigint, _deployedAt: bigint);
    get deployedAt(): bigint;
    static new(params: {
        /**
         * Network: Source
         * Delay from `deployedAt` at which ends `finality lock` and starts `private withdrawal` */
        srcWithdrawal: bigint;
        /**
         * Network: Source
         * Delay from `deployedAt` at which ends `private withdrawal` and starts `public withdrawal` */
        srcPublicWithdrawal: bigint;
        /**
         * Network: Source
         * Delay from `deployedAt` at which ends `public withdrawal` and starts `private cancellation` */
        srcCancellation: bigint;
        /**
         * Network: Source
         * Delay from `deployedAt` at which ends `private cancellation` and starts `public cancellation` */
        srcPublicCancellation: bigint;
        /**
         * Network: Destination
         * Delay from `deployedAt` at which ends `finality lock` and starts `private withdrawal` */
        dstWithdrawal: bigint;
        /**
         * Network: Destination
         * Delay from `deployedAt` at which ends `private withdrawal` and starts `public withdrawal` */
        dstPublicWithdrawal: bigint;
        /**
         * Network: Destination
         * Delay from `deployedAt` at which ends `public withdrawal` and starts `private cancellation` */
        dstCancellation: bigint;
    }): TimeLocks;
    static fromDurations(durations: {
        srcFinalityLock: bigint;
        srcPrivateWithdrawal: bigint;
        srcPublicWithdrawal: bigint;
        srcPrivateCancellation: bigint;
        dstFinalityLock: bigint;
        dstPrivateWithdrawal: bigint;
        dstPublicWithdrawal: bigint;
    }): TimeLocks;
    static fromBigInt(val: bigint): TimeLocks;
    toJSON(): string;
    build(): bigint;
    setDeployedAt(time: bigint): this;
    toSrcTimeLocks(deployedAt?: bigint): SrcTimeLocks;
    toDstTimeLocks(deployedAt?: bigint): DstTimeLocks;
}
