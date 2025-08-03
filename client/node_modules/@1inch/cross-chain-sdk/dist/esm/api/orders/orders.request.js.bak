import { isHexString } from '@1inch/byte-utils';
import { isValidAddress } from '@1inch/fusion-sdk';
import { PaginationRequest } from '../pagination';
export class ActiveOrdersRequest {
    constructor(params = {}) {
        this.pagination = new PaginationRequest(params.page, params.limit);
    }
    build() {
        return {
            page: this.pagination.page,
            limit: this.pagination.limit
        };
    }
}
export class OrderStatusRequest {
    constructor(params) {
        this.orderHash = params.orderHash;
        if (this.orderHash.length !== 66) {
            throw Error(`orderHash length should be equals 66`);
        }
        if (!isHexString(this.orderHash)) {
            throw Error(`orderHash have to be hex`);
        }
    }
    build() {
        return {
            orderHash: this.orderHash
        };
    }
}
export class OrdersByMakerRequest {
    constructor(params) {
        this.address = params.address;
        this.pagination = new PaginationRequest(params.page, params.limit);
        this.srcChain = params.srcChain;
        this.dstChain = params.dstChain;
        this.srcToken = params.srcToken;
        this.dstToken = params.dstToken;
        this.withToken = params.withToken;
        this.timestampFrom = params.timestampFrom;
        this.timestampTo = params.timestampTo;
        if (!isValidAddress(this.address)) {
            throw Error(`${this.address} is invalid address`);
        }
        if (this.srcToken && !isValidAddress(this.srcToken)) {
            throw Error(`${this.srcToken} is invalid address`);
        }
        if (this.dstToken && !isValidAddress(this.dstToken)) {
            throw Error(`${this.dstToken} is invalid address`);
        }
        if (this.withToken && !isValidAddress(this.withToken)) {
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
//# sourceMappingURL=orders.request.js.map