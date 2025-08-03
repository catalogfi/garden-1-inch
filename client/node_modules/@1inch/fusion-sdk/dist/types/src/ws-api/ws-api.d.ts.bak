import { ActiveOrdersWebSocketApi } from './active-websocket-orders-api';
import { RpcWebsocketApi } from './rpc-websocket-api';
import { WsApiConfigWithNetwork } from './types';
import { AnyFunction, AnyFunctionWithThis, OnMessageCb, WsProviderConnector } from '../connector/ws';
export declare class WebSocketApi {
    private static Version;
    readonly rpc: RpcWebsocketApi;
    readonly order: ActiveOrdersWebSocketApi;
    readonly provider: WsProviderConnector;
    constructor(configOrProvider: WsApiConfigWithNetwork | WsProviderConnector);
    static new(configOrProvider: WsApiConfigWithNetwork | WsProviderConnector): WebSocketApi;
    init(): void;
    on(event: string, cb: AnyFunctionWithThis): void;
    off(event: string, cb: AnyFunctionWithThis): void;
    onOpen(cb: AnyFunctionWithThis): void;
    send<T>(message: T): void;
    close(): void;
    onMessage(cb: OnMessageCb): void;
    onClose(cb: AnyFunction): void;
    onError(cb: AnyFunction): void;
}
