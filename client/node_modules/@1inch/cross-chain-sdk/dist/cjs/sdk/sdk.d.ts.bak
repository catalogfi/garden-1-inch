import { OrderInfo, OrderParams, PreparedOrder, QuoteParams, QuoteCustomPresetParams, CrossChainSDKConfigParams } from './types';
import { FusionApi, Quote, ActiveOrdersRequestParams, ActiveOrdersResponse, OrdersByMakerParams, OrdersByMakerResponse, OrderStatusResponse, ReadyToAcceptSecretFills, PublishedSecretsResponse, ReadyToExecutePublicActions } from '../api';
import { CrossChainOrder } from '../cross-chain-order';
import { SupportedChain } from '../chains';
export declare class SDK {
    private readonly config;
    readonly api: FusionApi;
    constructor(config: CrossChainSDKConfigParams);
    getActiveOrders(params?: ActiveOrdersRequestParams): Promise<ActiveOrdersResponse>;
    getOrderStatus(orderHash: string): Promise<OrderStatusResponse>;
    getOrdersByMaker(params: OrdersByMakerParams): Promise<OrdersByMakerResponse>;
    getReadyToAcceptSecretFills(orderHash: string): Promise<ReadyToAcceptSecretFills>;
    getReadyToExecutePublicActions(): Promise<ReadyToExecutePublicActions>;
    getPublishedSecrets(orderHash: string): Promise<PublishedSecretsResponse>;
    submitSecret(orderHash: string, secret: string): Promise<void>;
    getQuote(params: QuoteParams): Promise<Quote>;
    getQuoteWithCustomPreset(params: QuoteParams, body: QuoteCustomPresetParams): Promise<Quote>;
    createOrder(quote: Quote, params: OrderParams): Promise<PreparedOrder>;
    submitOrder(srcChainId: SupportedChain, order: CrossChainOrder, quoteId: string, secretHashes: string[]): Promise<OrderInfo>;
    placeOrder(quote: Quote, params: OrderParams): Promise<OrderInfo>;
    buildCancelOrderCallData(orderHash: string): Promise<string>;
}
