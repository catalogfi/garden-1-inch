import { Address, Interaction, MakerTraits, ZX, SettlementPostInteractionData, now } from '@1inch/fusion-sdk';
import assert from 'assert';
import { InnerOrder } from './inner-order';
import { EscrowExtension } from './escrow-extension';
import { TRUE_ERC20 } from '../deployments';
import { isSupportedChain } from '../chains';
import { Immutables } from '../immutables';
export class CrossChainOrder {
    constructor(extension, orderInfo, extra) {
        this.inner = new InnerOrder(extension, orderInfo, extra);
    }
    get dstChainId() {
        return this.inner.escrowExtension.dstChainId;
    }
    get escrowExtension() {
        return this.inner.escrowExtension;
    }
    get extension() {
        return this.inner.extension;
    }
    get maker() {
        return this.inner.maker;
    }
    get takerAsset() {
        return this.inner.escrowExtension.dstToken;
    }
    get makerAsset() {
        return this.inner.makerAsset;
    }
    get takingAmount() {
        return this.inner.takingAmount;
    }
    get makingAmount() {
        return this.inner.makingAmount;
    }
    get salt() {
        return this.inner.salt;
    }
    /**
     * If zero address, then maker will receive funds
     */
    get receiver() {
        return this.inner.receiver;
    }
    /**
     * Timestamp in sec
     */
    get deadline() {
        return this.inner.deadline;
    }
    /**
     * Timestamp in sec
     */
    get auctionStartTime() {
        return this.inner.auctionStartTime;
    }
    /**
     * Timestamp in sec
     */
    get auctionEndTime() {
        return this.inner.auctionEndTime;
    }
    get nonce() {
        return this.inner.nonce;
    }
    get partialFillAllowed() {
        return this.inner.partialFillAllowed;
    }
    get multipleFillsAllowed() {
        return this.inner.multipleFillsAllowed;
    }
    /**
     * Create new CrossChainOrder
     */
    static new(escrowFactory, orderInfo, escrowParams, details, extra) {
        const postInteractionData = SettlementPostInteractionData.new({
            bankFee: details.fees?.bankFee || 0n,
            integratorFee: details.fees?.integratorFee,
            whitelist: details.whitelist,
            resolvingStartTime: details.resolvingStartTime ?? now(),
            customReceiver: orderInfo.receiver
        });
        const ext = new EscrowExtension(escrowFactory, details.auction, postInteractionData, extra?.permit
            ? new Interaction(orderInfo.makerAsset, extra.permit)
            : undefined, escrowParams.hashLock, escrowParams.dstChainId, orderInfo.takerAsset, escrowParams.srcSafetyDeposit, escrowParams.dstSafetyDeposit, escrowParams.timeLocks);
        assert(isSupportedChain(escrowParams.srcChainId), `Not supported chain ${escrowParams.srcChainId}`);
        assert(isSupportedChain(escrowParams.dstChainId), `Not supported chain ${escrowParams.dstChainId}`);
        assert(escrowParams.srcChainId !== escrowParams.dstChainId, 'Chains must be different');
        return new CrossChainOrder(ext, {
            ...orderInfo,
            takerAsset: TRUE_ERC20[escrowParams.srcChainId]
        }, extra);
    }
    /**
     * Create CrossChainOrder from order data and extension
     *
     */
    static fromDataAndExtension(order, extension) {
        const ext = EscrowExtension.fromExtension(extension);
        const makerTraits = new MakerTraits(BigInt(order.makerTraits));
        const deadline = makerTraits.expiration();
        const orderExpirationDelay = deadline === null
            ? undefined
            : deadline -
                ext.auctionDetails.startTime -
                ext.auctionDetails.duration;
        return new CrossChainOrder(ext, {
            makerAsset: new Address(order.makerAsset),
            takerAsset: new Address(order.takerAsset),
            makingAmount: BigInt(order.makingAmount),
            takingAmount: BigInt(order.takingAmount),
            receiver: new Address(order.receiver),
            maker: new Address(order.maker),
            salt: BigInt(order.salt) >> 160n
        }, {
            enablePermit2: makerTraits.isPermit2(),
            nonce: makerTraits.nonceOrEpoch(),
            permit: extension.makerPermit === ZX
                ? undefined
                : Interaction.decode(extension.makerPermit).data,
            orderExpirationDelay,
            allowMultipleFills: makerTraits.isMultipleFillsAllowed(),
            allowPartialFills: makerTraits.isPartialFillAllowed()
        });
    }
    build() {
        return this.inner.build();
    }
    getOrderHash(srcChainId) {
        return this.inner.getOrderHash(srcChainId);
    }
    getTypedData(srcChainId) {
        return this.inner.getTypedData(srcChainId);
    }
    getCalculator() {
        return this.inner.getCalculator();
    }
    /**
     * Calculates required taking amount for passed `makingAmount` at block time `time`
     *
     * @param makingAmount maker swap amount
     * @param time execution time in sec
     * @param blockBaseFee block fee in wei.
     * */
    calcTakingAmount(makingAmount, time, blockBaseFee) {
        return this.inner.calcTakingAmount(makingAmount, time, blockBaseFee);
    }
    /**
     * Check whether address allowed to execute order at the given time
     *
     * @param executor address of executor
     * @param executionTime timestamp in sec at which order planning to execute
     */
    canExecuteAt(executor, executionTime) {
        return this.inner.canExecuteAt(executor, executionTime);
    }
    /**
     * Check is order expired at a given time
     *
     * @param time timestamp in seconds
     */
    isExpiredAt(time) {
        return this.inner.isExpiredAt(time);
    }
    /**
     * Returns how much fee will be credited from a resolver deposit account
     * Token of fee set in Settlement extension constructor
     * Actual deployments can be found at https://github.com/1inch/limit-order-settlement/tree/master/deployments
     *
     * @param filledMakingAmount which resolver fills
     * @see https://github.com/1inch/limit-order-settlement/blob/0e3cae3653092ebb4ea5d2a338c87a54351ad883/contracts/extensions/ResolverFeeExtension.sol#L29
     */
    getResolverFee(filledMakingAmount) {
        return this.inner.getResolverFee(filledMakingAmount);
    }
    /**
     * Check if `wallet` can fill order before other
     */
    isExclusiveResolver(wallet) {
        return this.inner.isExclusiveResolver(wallet);
    }
    /**
     * Check if the auction has exclusive resolver, and it is in the exclusivity period
     *
     * @param time timestamp to check, `now()` by default
     */
    isExclusivityPeriod(time = now()) {
        return this.inner.isExclusivityPeriod(time);
    }
    /**
     * @param srcChainId
     * @param taker executor of fillOrder* transaction
     * @param amount making amount (make sure same amount passed to contact fillOrder method)
     * @param hashLock leaf of a merkle tree for multiple fill
     */
    toSrcImmutables(srcChainId, taker, amount, hashLock = this.escrowExtension.hashLockInfo) {
        const isPartialFill = amount !== this.makingAmount;
        const isLeafHashLock = hashLock !== this.escrowExtension.hashLockInfo;
        if (isPartialFill && !isLeafHashLock) {
            throw new Error('Provide leaf of merkle tree as HashLock for partial fell');
        }
        return Immutables.new({
            hashLock,
            safetyDeposit: this.escrowExtension.srcSafetyDeposit,
            taker,
            maker: this.maker,
            orderHash: this.getOrderHash(srcChainId),
            amount,
            timeLocks: this.escrowExtension.timeLocks,
            token: this.makerAsset
        });
    }
    getMultipleFillIdx(fillAmount, remainingAmount = this.makingAmount) {
        assert(this.inner.multipleFillsAllowed, 'Multiple fills disabled for order');
        const partsCount = this.escrowExtension.hashLockInfo.getPartsCount();
        const calculatedIndex = ((this.makingAmount - remainingAmount + fillAmount - 1n) *
            partsCount) /
            this.makingAmount;
        if (remainingAmount === fillAmount) {
            return Number(calculatedIndex + 1n);
        }
        return Number(calculatedIndex);
    }
}
//# sourceMappingURL=cross-chain-order.js.map