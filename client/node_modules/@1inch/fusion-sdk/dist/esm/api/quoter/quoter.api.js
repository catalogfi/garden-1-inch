import { Quote } from './quote';
import { concatQueryParams } from '../params';
import { AxiosProviderConnector } from '../../connector';
export class QuoterApi {
    constructor(config, httpClient) {
        this.config = config;
        this.httpClient = httpClient;
    }
    static new(config, httpClient = new AxiosProviderConnector(config.authKey)) {
        return new QuoterApi(config, httpClient);
    }
    async getQuote(params) {
        const queryParams = concatQueryParams(params.build());
        const url = `${this.config.url}/${QuoterApi.Version}/${this.config.network}/quote/receive/${queryParams}`;
        const res = await this.httpClient.get(url);
        return new Quote(params, res);
    }
    async getQuoteWithCustomPreset(params, body) {
        const bodyErr = body.validate();
        if (bodyErr) {
            throw new Error(bodyErr);
        }
        const queryParams = concatQueryParams(params.build());
        const bodyParams = body.build();
        const url = `${this.config.url}/${QuoterApi.Version}/${this.config.network}/quote/receive/${queryParams}`;
        const res = await this.httpClient.post(url, bodyParams);
        return new Quote(params, res);
    }
}
QuoterApi.Version = 'v2.0';
//# sourceMappingURL=quoter.api.js.map