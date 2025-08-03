"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FusionOrderParams = void 0;
const limit_order_sdk_1 = require("@1inch/limit-order-sdk");
const types_1 = require("../types");
class FusionOrderParams {
    constructor(params) {
        this.preset = types_1.PresetEnum.fast;
        this.receiver = limit_order_sdk_1.Address.ZERO_ADDRESS;
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
exports.FusionOrderParams = FusionOrderParams;
//# sourceMappingURL=order-params.js.map