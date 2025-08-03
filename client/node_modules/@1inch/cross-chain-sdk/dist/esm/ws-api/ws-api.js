import { WebsocketClient } from '@1inch/fusion-sdk';
import { ActiveOrdersWebSocketApi } from './active-websocket-orders-api';
import { RpcWebsocketApi } from './rpc-websocket-api';
import { castUrl } from './url';
export class WebSocketApi {
    constructor(configOrProvider) {
        if (instanceOfWsApiConfig(configOrProvider)) {
            const url = castUrl(configOrProvider.url);
            const configWithUrl = {
                ...configOrProvider,
                url: `${url}/${WebSocketApi.Version}`
            };
            const provider = new WebsocketClient(configWithUrl);
            this.provider = provider;
            this.rpc = new RpcWebsocketApi(provider);
            this.order = new ActiveOrdersWebSocketApi(provider);
            return;
        }
        this.provider = configOrProvider;
        this.rpc = new RpcWebsocketApi(configOrProvider);
        this.order = new ActiveOrdersWebSocketApi(configOrProvider);
    }
    static new(configOrProvider) {
        return new WebSocketApi(configOrProvider);
    }
    init() {
        this.provider.init();
    }
    on(event, cb) {
        this.provider.on(event, cb);
    }
    off(event, cb) {
        this.provider.off(event, cb);
    }
    onOpen(cb) {
        this.provider.onOpen(cb);
    }
    send(message) {
        this.provider.send(message);
    }
    close() {
        this.provider.close();
    }
    onMessage(cb) {
        this.provider.onMessage(cb);
    }
    onClose(cb) {
        this.provider.onClose(cb);
    }
    onError(cb) {
        this.provider.onError(cb);
    }
}
WebSocketApi.Version = 'v1.0';
function instanceOfWsApiConfig(val) {
    return !('send' in val);
}
//# sourceMappingURL=ws-api.js.map