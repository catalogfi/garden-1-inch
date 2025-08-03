"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.OrdersApi = void 0;
const orders_request_1 = require("./orders.request");
const params_1 = require("../params");
class OrdersApi {
    constructor(config, httpClient) {
        this.config = config;
        this.httpClient = httpClient;
    }
    async getActiveOrders(params = new orders_request_1.ActiveOrdersRequest()) {
        const queryParams = (0, params_1.concatQueryParams)(params.build());
        const url = `${this.config.url}/${OrdersApi.Version}/order/active/${queryParams}`;
        return this.httpClient.get(url);
    }
    async getOrderStatus(params) {
        const url = `${this.config.url}/${OrdersApi.Version}/order/status/${params.orderHash}`;
        return this.httpClient.get(url);
    }
    async getOrdersByMaker(params) {
        const qp = (0, params_1.concatQueryParams)(params.buildQueryParams());
        const url = `${this.config.url}/${OrdersApi.Version}/order/maker/${params.address}/${qp}`;
        return this.httpClient.get(url);
    }
    async getReadyToAcceptSecretFills(orderHash) {
        const url = `${this.config.url}/${OrdersApi.Version}/order/ready-to-accept-secret-fills/${orderHash}`;
        return this.httpClient.get(url);
    }
    async getReadyToExecutePublicActions() {
        const url = `${this.config.url}/${OrdersApi.Version}/order/ready-to-execute-public-actions`;
        return this.httpClient.get(url);
    }
    async getPublishedSecrets(orderHash) {
        const url = `${this.config.url}/${OrdersApi.Version}/order/secrets/${orderHash}`;
        return this.httpClient.get(url);
    }
}
exports.OrdersApi = OrdersApi;
OrdersApi.Version = 'v1.0';
//# sourceMappingURL=orders.api.js.map