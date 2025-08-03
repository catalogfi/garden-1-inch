import { ActiveOrdersRequest } from './orders.request';
import { concatQueryParams } from '../params';
export class OrdersApi {
    constructor(config, httpClient) {
        this.config = config;
        this.httpClient = httpClient;
    }
    async getActiveOrders(params = new ActiveOrdersRequest()) {
        const queryParams = concatQueryParams(params.build());
        const url = `${this.config.url}/${OrdersApi.Version}/order/active/${queryParams}`;
        return this.httpClient.get(url);
    }
    async getOrderStatus(params) {
        const url = `${this.config.url}/${OrdersApi.Version}/order/status/${params.orderHash}`;
        return this.httpClient.get(url);
    }
    async getOrdersByMaker(params) {
        const qp = concatQueryParams(params.buildQueryParams());
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
OrdersApi.Version = 'v1.0';
//# sourceMappingURL=orders.api.js.map