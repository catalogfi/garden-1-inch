import { AxiosProviderConnector } from '@1inch/fusion-sdk';
import { QuoterApi } from './quoter';
import { RelayerApi } from './relayer';
import { ActiveOrdersRequest, OrdersApi } from './orders';
export class FusionApi {
    constructor(config) {
        const httpProvider = config.httpProvider || new AxiosProviderConnector(config.authKey);
        this.quoterApi = new QuoterApi({
            url: `${config.url}/quoter`,
            authKey: config.authKey
        }, httpProvider);
        this.relayerApi = new RelayerApi({
            url: `${config.url}/relayer`,
            authKey: config.authKey
        }, httpProvider);
        this.ordersApi = new OrdersApi({
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
    getActiveOrders(params = new ActiveOrdersRequest()) {
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
//# sourceMappingURL=fusion-api.js.map