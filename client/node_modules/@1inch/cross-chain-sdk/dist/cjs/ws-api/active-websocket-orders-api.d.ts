import { OnOrderFilledCb, OnOrderFilledPartiallyCb, OnOrderInvalidCb, WsProviderConnector } from '@1inch/fusion-sdk';
import { OnOrderCancelledCb, OnOrderCb, OnOrderCreatedCb, OnOrderNotEnoughAllowanceCb, OnOrderNotEnoughBalanceCb, OnOrderSecretSharedCb } from './types';
export declare class ActiveOrdersWebSocketApi {
    readonly provider: WsProviderConnector;
    constructor(provider: WsProviderConnector);
    onOrder(cb: OnOrderCb): void;
    onOrderCreated(cb: OnOrderCreatedCb): void;
    onOrderInvalid(cb: OnOrderInvalidCb): void;
    onOrderBalanceChange(cb: OnOrderNotEnoughBalanceCb): void;
    onOrderAllowanceChange(cb: OnOrderNotEnoughAllowanceCb): void;
    onOrderFilled(cb: OnOrderFilledCb): void;
    onOrderCancelled(cb: OnOrderCancelledCb): void;
    onOrderFilledPartially(cb: OnOrderFilledPartiallyCb): void;
    onOrderSecretShared(cb: OnOrderSecretSharedCb): void;
}
