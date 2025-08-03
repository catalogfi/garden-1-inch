"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.OrdersByMakerRequest = exports.OrderStatusRequest = exports.ActiveOrdersRequest = void 0;
const byte_utils_1 = require("@1inch/byte-utils");
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const pagination_1 = require("../pagination");
class ActiveOrdersRequest {
    constructor(params = {}) {
        this.pagination = new pagination_1.PaginationRequest(params.page, params.limit);
    }
    build() {
        return {
            page: this.pagination.page,
            limit: this.pagination.limit
        };
    }
}
exports.ActiveOrdersRequest = ActiveOrdersRequest;
class OrderStatusRequest {
    constructor(params) {
        this.orderHash = params.orderHash;
        if (this.orderHash.length !== 66) {
            throw Error(`orderHash length should be equals 66`);
        }
        if (!(0, byte_utils_1.isHexString)(this.orderHash)) {
            throw Error(`orderHash have to be hex`);
        }
    }
    build() {
        return {
            orderHash: this.orderHash
        };
    }
}
exports.OrderStatusRequest = OrderStatusRequest;
class OrdersByMakerRequest {
    constructor(params) {
        this.address = params.address;
        this.pagination = new pagination_1.PaginationRequest(params.page, params.limit);
        this.srcChain = params.srcChain;
        this.dstChain = params.dstChain;
        this.srcToken = params.srcToken;
        this.dstToken = params.dstToken;
        this.withToken = params.withToken;
        this.timestampFrom = params.timestampFrom;
        this.timestampTo = params.timestampTo;
        if (!(0, fusion_sdk_1.isValidAddress)(this.address)) {
            throw Error(`${this.address} is invalid address`);
        }
        if (this.srcToken && !(0, fusion_sdk_1.isValidAddress)(this.srcToken)) {
            throw Error(`${this.srcToken} is invalid address`);
        }
        if (this.dstToken && !(0, fusion_sdk_1.isValidAddress)(this.dstToken)) {
            throw Error(`${this.dstToken} is invalid address`);
        }
        if (this.withToken && !(0, fusion_sdk_1.isValidAddress)(this.withToken)) {
            throw Error(`${this.withToken} is invalid address`);
        }
    }
    buildQueryParams() {
        return {
            limit: this.pagination.limit,
            page: this.pagination.page,
            srcChain: this.srcChain,
            dstChain: this.dstChain,
            srcToken: this.srcToken,
            dstToken: this.dstToken,
            withToken: this.withToken,
            timestampFrom: this.timestampFrom,
            timestampTo: this.timestampTo
        };
    }
}
exports.OrdersByMakerRequest = OrdersByMakerRequest;
//# sourceMappingURL=orders.request.js.map