import { LimitOrderV4Struct } from '@1inch/fusion-sdk';
import { RelayerRequestParams } from './types';
import { SupportedChain } from '../../chains';
export declare class RelayerRequest {
    readonly order: LimitOrderV4Struct;
    readonly signature: string;
    readonly quoteId: string;
    readonly extension: string;
    readonly srcChainId: SupportedChain;
    readonly secretHashes?: string[];
    constructor(params: RelayerRequestParams);
    static new(params: RelayerRequestParams): RelayerRequest;
    build(): RelayerRequestParams;
}
