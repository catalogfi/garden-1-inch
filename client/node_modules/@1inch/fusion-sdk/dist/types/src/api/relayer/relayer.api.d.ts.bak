import { RelayerRequest } from './relayer.request';
import { RelayerApiConfig } from './types';
import { HttpProviderConnector } from '../../connector';
export declare class RelayerApi {
    private readonly config;
    private readonly httpClient;
    private static Version;
    constructor(config: RelayerApiConfig, httpClient: HttpProviderConnector);
    static new(config: RelayerApiConfig, httpClient?: HttpProviderConnector): RelayerApi;
    submit(params: RelayerRequest): Promise<void>;
    submitBatch(params: RelayerRequest[]): Promise<void>;
}
