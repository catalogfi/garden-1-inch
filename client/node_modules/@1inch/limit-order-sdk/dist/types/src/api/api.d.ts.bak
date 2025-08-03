import { ApiConfig, LimitOrderApiItem, SortKey, StatusKey } from './types';
import { Pager } from './pager';
import { LimitOrder } from '../limit-order';
import { Address } from '../address';
export declare class Api {
    private readonly baseUrl;
    private readonly networkId;
    private readonly httpClient;
    private readonly authHeader;
    constructor(config: ApiConfig);
    submitOrder(order: LimitOrder, signature: string): Promise<void>;
    getOrdersByMaker(maker: Address, filters?: {
        pager?: Pager;
        statuses?: StatusKey[];
        takerAsset?: Address;
        makerAsset?: Address;
    }, sort?: SortKey): Promise<LimitOrderApiItem[]>;
    getOrderByHash(hash: string): Promise<LimitOrderApiItem>;
    private url;
    private headers;
}
