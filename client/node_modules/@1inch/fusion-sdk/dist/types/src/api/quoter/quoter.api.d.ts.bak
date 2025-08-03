import { QuoterRequest } from './quoter.request';
import { QuoterApiConfig } from './types';
import { Quote } from './quote';
import { QuoterCustomPresetRequest } from './quoter-custom-preset.request';
import { HttpProviderConnector } from '../../connector';
export declare class QuoterApi {
    private readonly config;
    private readonly httpClient;
    private static Version;
    constructor(config: QuoterApiConfig, httpClient: HttpProviderConnector);
    static new(config: QuoterApiConfig, httpClient?: HttpProviderConnector): QuoterApi;
    getQuote(params: QuoterRequest): Promise<Quote>;
    getQuoteWithCustomPreset(params: QuoterRequest, body: QuoterCustomPresetRequest): Promise<Quote>;
}
