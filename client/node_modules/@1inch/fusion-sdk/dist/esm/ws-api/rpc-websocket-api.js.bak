import { PaginationRequest } from '../api/pagination';
export class RpcWebsocketApi {
    constructor(provider) {
        this.provider = provider;
    }
    onPong(cb) {
        this.provider.onMessage((data) => {
            if (data.method === 'ping') {
                cb(data.result);
            }
        });
    }
    ping() {
        this.provider.send({ method: 'ping' });
    }
    getActiveOrders({ limit, page } = {}) {
        const paginationRequest = new PaginationRequest(page, limit);
        const err = paginationRequest.validate();
        if (err) {
            throw new Error(err);
        }
        this.provider.send({ method: 'getActiveOrders', param: { limit, page } });
    }
    onGetActiveOrders(cb) {
        this.provider.onMessage((data) => {
            if (data.method === 'getActiveOrders') {
                cb(data.result);
            }
        });
    }
    getAllowedMethods() {
        this.provider.send({ method: 'getAllowedMethods' });
    }
    onGetAllowedMethods(cb) {
        this.provider.onMessage((data) => {
            if (data.method === 'getAllowedMethods') {
                cb(data.result);
            }
        });
    }
}
//# sourceMappingURL=rpc-websocket-api.js.map