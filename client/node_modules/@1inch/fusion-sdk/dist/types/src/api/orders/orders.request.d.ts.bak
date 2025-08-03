import { ActiveOrdersRequestParams, OrdersByMakerParams, OrderStatusParams } from './types';
import { PaginationParams, PaginationRequest } from '../pagination';
export declare class ActiveOrdersRequest {
    readonly pagination: PaginationRequest;
    constructor(params?: ActiveOrdersRequestParams);
    static new(params?: ActiveOrdersRequestParams): ActiveOrdersRequest;
    validate(): string | null;
    build(): ActiveOrdersRequestParams;
}
export declare class OrderStatusRequest {
    readonly orderHash: string;
    constructor(params: OrderStatusParams);
    static new(params: OrderStatusParams): OrderStatusRequest;
    validate(): string | null;
    build(): OrderStatusParams;
}
export declare class OrdersByMakerRequest {
    readonly address: string;
    readonly pagination: PaginationRequest;
    constructor(params: OrdersByMakerParams);
    static new(params: OrdersByMakerParams): OrdersByMakerRequest;
    validate(): string | null;
    buildQueryParams(): PaginationParams;
}
