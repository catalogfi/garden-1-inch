import { ActiveOrdersRequestParams, OrdersByMakerParams, OrderStatusParams } from './types';
import { PaginationRequest } from '../pagination';
import { SupportedChain } from '../../chains';
export declare class ActiveOrdersRequest {
    readonly pagination: PaginationRequest;
    constructor(params?: ActiveOrdersRequestParams);
    build(): ActiveOrdersRequestParams;
}
export declare class OrderStatusRequest {
    readonly orderHash: string;
    constructor(params: OrderStatusParams);
    build(): OrderStatusParams;
}
export declare class OrdersByMakerRequest {
    readonly address: string;
    readonly pagination: PaginationRequest;
    readonly srcChain?: SupportedChain;
    readonly dstChain?: SupportedChain;
    readonly srcToken?: string;
    readonly dstToken?: string;
    readonly withToken?: string;
    readonly timestampFrom?: number;
    readonly timestampTo?: number;
    constructor(params: OrdersByMakerParams);
    buildQueryParams(): Omit<OrdersByMakerParams, 'address'>;
}
