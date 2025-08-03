"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.QuoterRequest = void 0;
const fusion_sdk_1 = require("@1inch/fusion-sdk");
class QuoterRequest {
    constructor(params) {
        if (params.srcChain === params.dstChain) {
            throw new Error('srcChain and dstChain should be different');
        }
        if (!(0, fusion_sdk_1.isValidAmount)(params.amount)) {
            throw new Error(`${params.amount} is invalid amount`);
        }
        this.srcChain = params.srcChain;
        this.dstChain = params.dstChain;
        this.srcTokenAddress = new fusion_sdk_1.Address(params.srcTokenAddress);
        this.dstTokenAddress = new fusion_sdk_1.Address(params.dstTokenAddress);
        this.walletAddress = new fusion_sdk_1.Address(params.walletAddress);
        this.enableEstimate = params.enableEstimate || false;
        this.permit = params.permit;
        this.fee = params.fee;
        this.source = params.source || 'sdk';
        this.isPermit2 = params.isPermit2 ?? false;
        if (this.srcTokenAddress.isNative()) {
            throw new Error(`cannot swap ${fusion_sdk_1.Address.NATIVE_CURRENCY}: wrap native currency to it's wrapper fist`);
        }
        if (this.dstTokenAddress.isZero()) {
            throw new Error(`replace ${fusion_sdk_1.Address.ZERO_ADDRESS} with ${fusion_sdk_1.Address.NATIVE_CURRENCY}`);
        }
        this.amount = BigInt(params.amount);
        if (this.fee && this.source === 'sdk') {
            throw new Error('cannot use fee without source');
        }
    }
    static new(params) {
        return new QuoterRequest(params);
    }
    build() {
        return {
            srcChain: this.srcChain,
            dstChain: this.dstChain,
            srcTokenAddress: this.srcTokenAddress.toString(),
            dstTokenAddress: this.dstTokenAddress.toString(),
            amount: this.amount.toString(),
            walletAddress: this.walletAddress.toString(),
            enableEstimate: this.enableEstimate,
            permit: this.permit,
            fee: this.fee,
            source: this.source,
            isPermit2: this.isPermit2
        };
    }
}
exports.QuoterRequest = QuoterRequest;
//# sourceMappingURL=quoter.request.js.map