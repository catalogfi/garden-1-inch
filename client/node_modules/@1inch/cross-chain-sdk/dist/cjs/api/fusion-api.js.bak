"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FusionApi = void 0;
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const quoter_1 = require("./quoter");
const relayer_1 = require("./relayer");
const orders_1 = require("./orders");
class FusionApi {
    constructor(config) {
        const httpProvider = config.httpProvider || new fusion_sdk_1.AxiosProviderConnector(config.authKey);
        this.quoterApi = new quoter_1.QuoterApi({
            url: `${config.url}/quoter`,
            authKey: config.authKey
        }, httpProvider);
        this.relayerApi = new relayer_1.RelayerApi({
            url: `${config.url}/relayer`,
            authKey: config.authKey
        }, httpProvider);
        this.ordersApi = new orders_1.OrdersApi({
            url: `${config.url}/orders`,
            authKey: config.authKey
        }, httpProvider);
    }
    getQuote(params) {
        return this.quoterApi.getQuote(params);
    }
    getQuoteWithCustomPreset(params, body) {
        return this.quoterApi.getQuoteWithCustomPreset(params, body);
    }
    getActiveOrders(params = new orders_1.ActiveOrdersRequest()) {
        return this.ordersApi.getActiveOrders(params);
    }
    getOrderStatus(params) {
        return this.ordersApi.getOrderStatus(params);
    }
    getOrdersByMaker(params) {
        return this.ordersApi.getOrdersByMaker(params);
    }
    getReadyToAcceptSecretFills(orderHash) {
        return this.ordersApi.getReadyToAcceptSecretFills(orderHash);
    }
    getReadyToExecutePublicActions() {
        return this.ordersApi.getReadyToExecutePublicActions();
    }
    getPublishedSecrets(orderHash) {
        return this.ordersApi.getPublishedSecrets(orderHash);
    }
    submitOrder(params) {
        return this.relayerApi.submit(params);
    }
    submitOrderBatch(params) {
        return this.relayerApi.submitBatch(params);
    }
    submitSecret(orderHash, secret) {
        return this.relayerApi.submitSecret(orderHash, secret);
    }
}
exports.FusionApi = FusionApi;
//# sourceMappingURL=fusion-api.js.map