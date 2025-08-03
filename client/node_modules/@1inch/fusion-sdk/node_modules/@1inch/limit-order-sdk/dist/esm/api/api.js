import { DEV_PORTAL_LIMIT_ORDER_BASE_URL } from './constants';
export class Api {
    constructor(config) {
        this.baseUrl = config.baseUrl || DEV_PORTAL_LIMIT_ORDER_BASE_URL;
        this.networkId = config.networkId;
        this.httpClient = config.httpConnector;
        this.authHeader = `Bearer ${config.authKey}`;
    }
    async submitOrder(order, signature) {
        await this.httpClient.post(this.url('/'), {
            orderHash: order.getOrderHash(this.networkId),
            signature,
            data: {
                ...order.build(),
                extension: order.extension.encode()
            }
        }, this.headers());
    }
    async getOrdersByMaker(maker, filters, sort) {
        const params = {
            limit: filters?.pager?.limit.toString(),
            page: filters?.pager?.page.toString(),
            statuses: filters?.statuses?.join(','),
            makerAsset: filters?.makerAsset?.toString(),
            takerAsset: filters?.takerAsset?.toString(),
            sortBy: sort
        };
        return this.httpClient.get(this.url(`/address/${maker}`, params), this.headers());
    }
    async getOrderByHash(hash) {
        return this.httpClient.get(this.url(`/order/${hash}`), this.headers());
    }
    url(path, params) {
        const query = params
            ? `?${new URLSearchParams(Object.entries(params).filter(([_, val]) => val !== undefined))}`
            : '';
        return `${this.baseUrl}/${this.networkId}${path}${query}`;
    }
    headers(additional) {
        return { Authorization: this.authHeader, ...additional };
    }
}
//# sourceMappingURL=api.js.map