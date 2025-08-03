import { UINT_40_MAX } from '@1inch/byte-utils';
import { Address, bpsToRatioFormat, randBigInt } from '@1inch/fusion-sdk';
import { PresetEnum } from '../types';
import { Preset } from '../preset';
import { CrossChainOrder, TimeLocks } from '../../../cross-chain-order';
export class Quote {
    constructor(params, response) {
        this.params = params;
        this.srcTokenAmount = BigInt(response.srcTokenAmount);
        this.dstTokenAmount = BigInt(response.dstTokenAmount);
        this.presets = {
            [PresetEnum.fast]: new Preset(response.presets.fast),
            [PresetEnum.medium]: new Preset(response.presets.medium),
            [PresetEnum.slow]: new Preset(response.presets.slow),
            [PresetEnum.custom]: response.presets.custom
                ? new Preset(response.presets.custom)
                : undefined
        };
        this.timeLocks = response.timeLocks;
        this.srcSafetyDeposit = BigInt(response.srcSafetyDeposit);
        this.dstSafetyDeposit = BigInt(response.dstSafetyDeposit);
        this.prices = response.prices;
        this.volume = response.volume;
        this.quoteId = response.quoteId;
        this.whitelist = response.whitelist.map((a) => new Address(a));
        this.recommendedPreset = response.recommendedPreset;
        this.slippage = response.autoK;
        this.srcEscrowFactory = new Address(response.srcEscrowFactory);
        this.dstEscrowFactory = new Address(response.dstEscrowFactory);
    }
    get srcChainId() {
        return this.params.srcChain;
    }
    get dstChainId() {
        return this.params.dstChain;
    }
    createOrder(params) {
        const preset = this.getPreset(params?.preset || this.recommendedPreset);
        const auctionDetails = preset.createAuctionDetails(params.delayAuctionStartTimeBy);
        const allowPartialFills = preset.allowPartialFills;
        const allowMultipleFills = preset.allowMultipleFills;
        const isNonceRequired = !allowPartialFills || !allowMultipleFills;
        const nonce = isNonceRequired
            ? (params.nonce ?? randBigInt(UINT_40_MAX))
            : params.nonce;
        const takerAsset = this.params.dstTokenAddress.isNative()
            ? Address.NATIVE_CURRENCY
            : this.params.dstTokenAddress;
        return CrossChainOrder.new(this.srcEscrowFactory, {
            makerAsset: this.params.srcTokenAddress,
            takerAsset: takerAsset,
            makingAmount: this.srcTokenAmount,
            takingAmount: preset.auctionEndAmount,
            maker: this.params.walletAddress,
            receiver: params.receiver
        }, {
            hashLock: params.hashLock,
            srcChainId: this.params.srcChain,
            dstChainId: this.params.dstChain,
            srcSafetyDeposit: this.srcSafetyDeposit,
            dstSafetyDeposit: this.dstSafetyDeposit,
            timeLocks: TimeLocks.new({
                srcWithdrawal: BigInt(this.timeLocks.srcWithdrawal),
                srcPublicWithdrawal: BigInt(this.timeLocks.srcPublicWithdrawal),
                srcCancellation: BigInt(this.timeLocks.srcCancellation),
                srcPublicCancellation: BigInt(this.timeLocks.srcPublicCancellation),
                dstWithdrawal: BigInt(this.timeLocks.dstWithdrawal),
                dstPublicWithdrawal: BigInt(this.timeLocks.dstPublicWithdrawal),
                dstCancellation: BigInt(this.timeLocks.dstCancellation)
            })
        }, {
            auction: auctionDetails,
            fees: {
                integratorFee: {
                    ratio: bpsToRatioFormat(this.params.fee) || 0n,
                    receiver: params?.takingFeeReceiver
                        ? new Address(params?.takingFeeReceiver)
                        : Address.ZERO_ADDRESS
                },
                bankFee: 0n
            },
            whitelist: this.getWhitelist(auctionDetails.startTime, preset.exclusiveResolver)
        }, {
            nonce,
            permit: params.permit,
            allowPartialFills,
            allowMultipleFills,
            orderExpirationDelay: params?.orderExpirationDelay,
            source: this.params.source,
            enablePermit2: params.isPermit2
        });
    }
    getPreset(type = this.recommendedPreset) {
        return this.presets[type];
    }
    getWhitelist(auctionStartTime, exclusiveResolver) {
        if (exclusiveResolver) {
            return this.whitelist.map((resolver) => {
                const isExclusive = resolver.equal(exclusiveResolver);
                return {
                    address: resolver,
                    allowFrom: isExclusive ? 0n : auctionStartTime
                };
            });
        }
        return this.whitelist.map((resolver) => ({
            address: resolver,
            allowFrom: 0n
        }));
    }
}
//# sourceMappingURL=quote.js.map