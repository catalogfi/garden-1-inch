import { HttpProviderConnector } from '@1inch/fusion-sdk';
import { QuoterRequest } from './quoter.request';
import { QuoterApiConfig } from './types';
import { Quote } from './quote';
import { QuoterCustomPresetRequest } from './quoter-custom-preset.request';
export declare class QuoterApi {
    private readonly config;
    private readonly httpClient;
    private static Version;
    constructor(config: QuoterApiConfig, httpClient: HttpProviderConnector);
    getQuote(params: QuoterRequest): Promise<Quote>;
    getQuoteWithCustomPreset(params: QuoterRequest, body: QuoterCustomPresetRequest): Promise<Quote>;
}
