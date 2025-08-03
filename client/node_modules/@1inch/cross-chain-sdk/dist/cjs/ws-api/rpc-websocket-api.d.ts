import { OnGetActiveOrdersCb, OnGetAllowedMethodsCb, OnPongCb, WsProviderConnector } from '@1inch/fusion-sdk';
import { OnGetSecretsCb } from './types';
import { PaginationParams } from '../api/pagination';
export declare class RpcWebsocketApi {
    readonly provider: WsProviderConnector;
    constructor(provider: WsProviderConnector);
    onPong(cb: OnPongCb): void;
    ping(): void;
    getActiveOrders({ limit, page }?: PaginationParams): void;
    onGetActiveOrders(cb: OnGetActiveOrdersCb): void;
    getSecrets({ limit, page }?: PaginationParams): void;
    onGetSecrets(cb: OnGetSecretsCb): void;
    getAllowedMethods(): void;
    onGetAllowedMethods(cb: OnGetAllowedMethodsCb): void;
}
