import { Address } from '@1inch/limit-order-sdk';
import { PresetEnum } from '../types';
export class FusionOrderParams {
    constructor(params) {
        this.preset = PresetEnum.fast;
        this.receiver = Address.ZERO_ADDRESS;
        if (params.preset) {
            this.preset = params.preset;
        }
        if (params.receiver) {
            this.receiver = params.receiver;
        }
        this.isPermit2 = params.isPermit2;
        this.nonce = params.nonce;
        this.permit = params.permit;
        this.delayAuctionStartTimeBy = params.delayAuctionStartTimeBy || 0n;
    }
    static new(params) {
        return new FusionOrderParams(params);
    }
}
//# sourceMappingURL=order-params.js.map