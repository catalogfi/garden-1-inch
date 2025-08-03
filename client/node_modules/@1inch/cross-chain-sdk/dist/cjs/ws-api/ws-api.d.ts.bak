import { AnyFunction, AnyFunctionWithThis, OnMessageCb, WsApiConfigWithNetwork, WsProviderConnector, WsApiConfig } from '@1inch/fusion-sdk';
import { ActiveOrdersWebSocketApi } from './active-websocket-orders-api';
import { RpcWebsocketApi } from './rpc-websocket-api';
import { WebSocketEvent } from './types';
export declare class WebSocketApi {
    private static Version;
    readonly rpc: RpcWebsocketApi;
    readonly order: ActiveOrdersWebSocketApi;
    readonly provider: WsProviderConnector;
    constructor(configOrProvider: WsApiConfig | WsProviderConnector);
    static new(configOrProvider: WsApiConfigWithNetwork | WsProviderConnector): WebSocketApi;
    init(): void;
    on(event: WebSocketEvent, cb: AnyFunctionWithThis): void;
    off(event: WebSocketEvent, cb: AnyFunctionWithThis): void;
    onOpen(cb: AnyFunctionWithThis): void;
    send<T>(message: T): void;
    close(): void;
    onMessage(cb: OnMessageCb): void;
    onClose(cb: AnyFunction): void;
    onError(cb: AnyFunction): void;
}
