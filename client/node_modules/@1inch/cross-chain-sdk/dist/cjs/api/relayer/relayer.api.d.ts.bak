import { HttpProviderConnector } from '@1inch/fusion-sdk';
import { RelayerRequest } from './relayer.request';
import { RelayerApiConfig } from './types';
export declare class RelayerApi {
    private readonly config;
    private readonly httpClient;
    private static Version;
    constructor(config: RelayerApiConfig, httpClient: HttpProviderConnector);
    submit(params: RelayerRequest): Promise<void>;
    submitBatch(params: RelayerRequest[]): Promise<void>;
    submitSecret(orderHash: string, secret: string): Promise<void>;
}
