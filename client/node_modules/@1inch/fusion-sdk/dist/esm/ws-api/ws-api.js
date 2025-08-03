import { ActiveOrdersWebSocketApi } from './active-websocket-orders-api';
import { RpcWebsocketApi } from './rpc-websocket-api';
import { castUrl } from './url';
import { WebsocketClient } from '../connector/ws';
export class WebSocketApi {
    constructor(configOrProvider) {
        if (instanceOfWsApiConfigWithNetwork(configOrProvider)) {
            const url = castUrl(configOrProvider.url);
            const urlWithNetwork = `${url}/${WebSocketApi.Version}/${configOrProvider.network}`;
            const configWithUrl = { ...configOrProvider, url: urlWithNetwork };
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
WebSocketApi.Version = 'v2.0';
function instanceOfWsApiConfigWithNetwork(val) {
    return 'url' in val && 'network' in val;
}
//# sourceMappingURL=ws-api.js.map