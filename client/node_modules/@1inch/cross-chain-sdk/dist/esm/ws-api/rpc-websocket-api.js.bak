import { RpcMethod } from './types';
import { PaginationRequest } from '../api/pagination';
export class RpcWebsocketApi {
    constructor(provider) {
        this.provider = provider;
    }
    onPong(cb) {
        this.provider.onMessage((data) => {
            if (data.method === RpcMethod.Ping) {
                cb(data.result);
            }
        });
    }
    ping() {
        this.provider.send({ method: RpcMethod.Ping });
    }
    getActiveOrders({ limit, page } = {}) {
        const paginationRequest = new PaginationRequest(page, limit);
        this.provider.send({
            method: RpcMethod.GetActiveOrders,
            param: paginationRequest
        });
    }
    onGetActiveOrders(cb) {
        this.provider.onMessage((data) => {
            if (data.method === RpcMethod.GetActiveOrders) {
                cb(data.result);
            }
        });
    }
    getSecrets({ limit, page } = {}) {
        const paginationRequest = new PaginationRequest(page, limit);
        this.provider.send({
            method: RpcMethod.GetSecrets,
            param: paginationRequest
        });
    }
    onGetSecrets(cb) {
        this.provider.onMessage((data) => {
            if (data.method === RpcMethod.GetSecrets) {
                cb(data.result);
            }
        });
    }
    getAllowedMethods() {
        this.provider.send({ method: RpcMethod.GetAllowedMethods });
    }
    onGetAllowedMethods(cb) {
        this.provider.onMessage((data) => {
            if (data.method === RpcMethod.GetAllowedMethods) {
                cb(data.result);
            }
        });
    }
}
//# sourceMappingURL=rpc-websocket-api.js.map