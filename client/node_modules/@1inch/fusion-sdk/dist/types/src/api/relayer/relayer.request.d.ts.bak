import { LimitOrderV4Struct } from '@1inch/limit-order-sdk';
import { RelayerRequestParams } from './types';
export declare class RelayerRequest {
    readonly order: LimitOrderV4Struct;
    readonly signature: string;
    readonly quoteId: string;
    readonly extension: string;
    constructor(params: RelayerRequestParams);
    static new(params: RelayerRequestParams): RelayerRequest;
    build(): RelayerRequestParams;
}
