import { Address } from '@1inch/limit-order-sdk';
import { AuctionDetails } from '../../fusion-order';
export class Preset {
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
            ? new Address(preset.exclusiveResolver)
            : undefined;
        this.allowPartialFills = preset.allowPartialFills;
        this.allowMultipleFills = preset.allowMultipleFills;
    }
    createAuctionDetails(additionalWaitPeriod = 0n) {
        return new AuctionDetails({
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
//# sourceMappingURL=preset.js.map