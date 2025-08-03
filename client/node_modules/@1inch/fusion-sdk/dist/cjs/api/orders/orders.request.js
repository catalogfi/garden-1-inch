"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.OrdersByMakerRequest = exports.OrderStatusRequest = exports.ActiveOrdersRequest = void 0;
const validations_1 = require("../../validations");
const pagination_1 = require("../pagination");
class ActiveOrdersRequest {
    constructor(params = {}) {
        this.pagination = new pagination_1.PaginationRequest(params.page, params.limit);
    }
    static new(params) {
        return new ActiveOrdersRequest(params);
    }
    validate() {
        const res = this.pagination.validate();
        if (res) {
            return res;
        }
        return null;
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
    }
    static new(params) {
        return new OrderStatusRequest(params);
    }
    validate() {
        if (this.orderHash.length !== 66) {
            return `orderHash length should be equals 66`;
        }
        if (!(0, validations_1.isHexString)(this.orderHash)) {
            return `orderHash have to be hex`;
        }
        return null;
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
    }
    static new(params) {
        return new OrdersByMakerRequest(params);
    }
    validate() {
        const res = this.pagination.validate();
        if (res) {
            return res;
        }
        if (!(0, validations_1.isValidAddress)(this.address)) {
            return `${this.address} is invalid address`;
        }
        return null;
    }
    buildQueryParams() {
        return {
            limit: this.pagination.limit,
            page: this.pagination.page
        };
    }
}
exports.OrdersByMakerRequest = OrdersByMakerRequest;
//# sourceMappingURL=orders.request.js.map