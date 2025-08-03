import { Address, AuctionCalculator, Extension, LimitOrderV4Struct, EIP712TypedData, NetworkEnum } from '@1inch/fusion-sdk';
import { CrossChainOrderInfo, Details, EscrowParams, Extra } from './types';
import { EscrowExtension } from './escrow-extension';
import { SupportedChain } from '../chains';
import { Immutables } from '../immutables';
export declare class CrossChainOrder {
    private inner;
    private constructor();
    get dstChainId(): NetworkEnum;
    get escrowExtension(): EscrowExtension;
    get extension(): Extension;
    get maker(): Address;
    get takerAsset(): Address;
    get makerAsset(): Address;
    get takingAmount(): bigint;
    get makingAmount(): bigint;
    get salt(): bigint;
    /**
     * If zero address, then maker will receive funds
     */
    get receiver(): Address;
    /**
     * Timestamp in sec
     */
    get deadline(): bigint;
    /**
     * Timestamp in sec
     */
    get auctionStartTime(): bigint;
    /**
     * Timestamp in sec
     */
    get auctionEndTime(): bigint;
    get nonce(): bigint;
    get partialFillAllowed(): boolean;
    get multipleFillsAllowed(): boolean;
    /**
     * Create new CrossChainOrder
     */
    static new(escrowFactory: Address, orderInfo: CrossChainOrderInfo, escrowParams: EscrowParams, details: Details, extra?: Extra): CrossChainOrder;
    /**
     * Create CrossChainOrder from order data and extension
     *
     */
    static fromDataAndExtension(order: LimitOrderV4Struct, extension: Extension): CrossChainOrder;
    build(): LimitOrderV4Struct;
    getOrderHash(srcChainId: number): string;
    getTypedData(srcChainId: number): EIP712TypedData;
    getCalculator(): AuctionCalculator;
    /**
     * Calculates required taking amount for passed `makingAmount` at block time `time`
     *
     * @param makingAmount maker swap amount
     * @param time execution time in sec
     * @param blockBaseFee block fee in wei.
     * */
    calcTakingAmount(makingAmount: bigint, time: bigint, blockBaseFee?: bigint): bigint;
    /**
     * Check whether address allowed to execute order at the given time
     *
     * @param executor address of executor
     * @param executionTime timestamp in sec at which order planning to execute
     */
    canExecuteAt(executor: Address, executionTime: bigint): boolean;
    /**
     * Check is order expired at a given time
     *
     * @param time timestamp in seconds
     */
    isExpiredAt(time: bigint): boolean;
    /**
     * Returns how much fee will be credited from a resolver deposit account
     * Token of fee set in Settlement extension constructor
     * Actual deployments can be found at https://github.com/1inch/limit-order-settlement/tree/master/deployments
     *
     * @param filledMakingAmount which resolver fills
     * @see https://github.com/1inch/limit-order-settlement/blob/0e3cae3653092ebb4ea5d2a338c87a54351ad883/contracts/extensions/ResolverFeeExtension.sol#L29
     */
    getResolverFee(filledMakingAmount: bigint): bigint;
    /**
     * Check if `wallet` can fill order before other
     */
    isExclusiveResolver(wallet: Address): boolean;
    /**
     * Check if the auction has exclusive resolver, and it is in the exclusivity period
     *
     * @param time timestamp to check, `now()` by default
     */
    isExclusivityPeriod(time?: bigint): boolean;
    /**
     * @param srcChainId
     * @param taker executor of fillOrder* transaction
     * @param amount making amount (make sure same amount passed to contact fillOrder method)
     * @param hashLock leaf of a merkle tree for multiple fill
     */
    toSrcImmutables(srcChainId: SupportedChain, taker: Address, amount: bigint, hashLock?: import("./hash-lock").HashLock): Immutables;
    getMultipleFillIdx(fillAmount: bigint, remainingAmount?: bigint): number;
}
