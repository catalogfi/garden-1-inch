"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Preset = void 0;
const limit_order_sdk_1 = require("@1inch/limit-order-sdk");
const fusion_order_1 = require("../../fusion-order");
class Preset {
    constructor(preset) {
        this.auctionDuration = BigInt(preset.auctionDuration);
        this.startAuctionIn = BigInt(preset.startAuctionIn);
        this.bankFee = BigInt(preset.bankFee);
        this.initialRateBump = preset.initialRateBump;
        this.auctionStartAmount = BigInt(preset.auctionStartAmount);
        this.auctionEndAmount = BigInt(preset.auctionEndAmount);
        this.tokenFee = BigInt(preset.tokenFee);
        this.points = preset.points;
        this.gasCostInfo = {
            gasPriceEstimate: BigInt(preset.gasCost?.gasPriceEstimate || 0n),
            gasBumpEstimate: BigInt(preset.gasCost?.gasBumpEstimate || 0n)
        };
        this.exclusiveResolver = preset.exclusiveResolver
            ? new limit_order_sdk_1.Address(preset.exclusiveResolver)
            : undefined;
        this.allowPartialFills = preset.allowPartialFills;
        this.allowMultipleFills = preset.allowMultipleFills;
    }
    createAuctionDetails(additionalWaitPeriod = 0n) {
        return new fusion_order_1.AuctionDetails({
            duration: this.auctionDuration,
            startTime: this.calcAuctionStartTime(additionalWaitPeriod),
            initialRateBump: this.initialRateBump,
            points: this.points,
            gasCost: this.gasCostInfo
        });
    }
    calcAuctionStartTime(additionalWaitPeriod = 0n) {
        return (BigInt(Math.floor(Date.now() / 1000)) +
            additionalWaitPeriod +
            this.startAuctionIn);
    }
}
exports.Preset = Preset;
//# sourceMappingURL=preset.js.map