"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.WebSocketApi = void 0;
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const active_websocket_orders_api_1 = require("./active-websocket-orders-api");
const rpc_websocket_api_1 = require("./rpc-websocket-api");
const url_1 = require("./url");
class WebSocketApi {
    constructor(configOrProvider) {
        if (instanceOfWsApiConfig(configOrProvider)) {
            const url = (0, url_1.castUrl)(configOrProvider.url);
            const configWithUrl = {
                ...configOrProvider,
                url: `${url}/${WebSocketApi.Version}`
            };
            const provider = new fusion_sdk_1.WebsocketClient(configWithUrl);
            this.provider = provider;
            this.rpc = new rpc_websocket_api_1.RpcWebsocketApi(provider);
            this.order = new active_websocket_orders_api_1.ActiveOrdersWebSocketApi(provider);
            return;
        }
        this.provider = configOrProvider;
        this.rpc = new rpc_websocket_api_1.RpcWebsocketApi(configOrProvider);
        this.order = new active_websocket_orders_api_1.ActiveOrdersWebSocketApi(configOrProvider);
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
exports.WebSocketApi = WebSocketApi;
WebSocketApi.Version = 'v1.0';
function instanceOfWsApiConfig(val) {
    return !('send' in val);
}
//# sourceMappingURL=ws-api.js.map