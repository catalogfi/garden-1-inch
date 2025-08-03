import { isHexString, isValidAddress } from '../../validations';
import { PaginationRequest } from '../pagination';
export class ActiveOrdersRequest {
    constructor(params = {}) {
        this.pagination = new PaginationRequest(params.page, params.limit);
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
export class OrderStatusRequest {
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
        if (!isHexString(this.orderHash)) {
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
export class OrdersByMakerRequest {
    constructor(params) {
        this.address = params.address;
        this.pagination = new PaginationRequest(params.page, params.limit);
    }
    static new(params) {
        return new OrdersByMakerRequest(params);
    }
    validate() {
        const res = this.pagination.validate();
        if (res) {
            return res;
        }
        if (!isValidAddress(this.address)) {
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
//# sourceMappingURL=orders.request.js.map