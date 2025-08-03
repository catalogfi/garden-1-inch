"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Quote = void 0;
const byte_utils_1 = require("@1inch/byte-utils");
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const types_1 = require("../types");
const preset_1 = require("../preset");
const cross_chain_order_1 = require("../../../cross-chain-order");
class Quote {
    constructor(params, response) {
        this.params = params;
        this.srcTokenAmount = BigInt(response.srcTokenAmount);
        this.dstTokenAmount = BigInt(response.dstTokenAmount);
        this.presets = {
            [types_1.PresetEnum.fast]: new preset_1.Preset(response.presets.fast),
            [types_1.PresetEnum.medium]: new preset_1.Preset(response.presets.medium),
            [types_1.PresetEnum.slow]: new preset_1.Preset(response.presets.slow),
            [types_1.PresetEnum.custom]: response.presets.custom
                ? new preset_1.Preset(response.presets.custom)
                : undefined
        };
        this.timeLocks = response.timeLocks;
        this.srcSafetyDeposit = BigInt(response.srcSafetyDeposit);
        this.dstSafetyDeposit = BigInt(response.dstSafetyDeposit);
        this.prices = response.prices;
        this.volume = response.volume;
        this.quoteId = response.quoteId;
        this.whitelist = response.whitelist.map((a) => new fusion_sdk_1.Address(a));
        this.recommendedPreset = response.recommendedPreset;
        this.slippage = response.autoK;
        this.srcEscrowFactory = new fusion_sdk_1.Address(response.srcEscrowFactory);
        this.dstEscrowFactory = new fusion_sdk_1.Address(response.dstEscrowFactory);
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
            ? (params.nonce ?? (0, fusion_sdk_1.randBigInt)(byte_utils_1.UINT_40_MAX))
            : params.nonce;
        const takerAsset = this.params.dstTokenAddress.isNative()
            ? fusion_sdk_1.Address.NATIVE_CURRENCY
            : this.params.dstTokenAddress;
        return cross_chain_order_1.CrossChainOrder.new(this.srcEscrowFactory, {
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
            timeLocks: cross_chain_order_1.TimeLocks.new({
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
                    ratio: (0, fusion_sdk_1.bpsToRatioFormat)(this.params.fee) || 0n,
                    receiver: params?.takingFeeReceiver
                        ? new fusion_sdk_1.Address(params?.takingFeeReceiver)
                        : fusion_sdk_1.Address.ZERO_ADDRESS
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
exports.Quote = Quote;
//# sourceMappingURL=quote.js.map