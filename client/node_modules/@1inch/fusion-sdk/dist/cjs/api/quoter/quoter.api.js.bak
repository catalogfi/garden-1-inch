"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.QuoterApi = void 0;
const quote_1 = require("./quote");
const params_1 = require("../params");
const connector_1 = require("../../connector");
class QuoterApi {
    constructor(config, httpClient) {
        this.config = config;
        this.httpClient = httpClient;
    }
    static new(config, httpClient = new connector_1.AxiosProviderConnector(config.authKey)) {
        return new QuoterApi(config, httpClient);
    }
    async getQuote(params) {
        const queryParams = (0, params_1.concatQueryParams)(params.build());
        const url = `${this.config.url}/${QuoterApi.Version}/${this.config.network}/quote/receive/${queryParams}`;
        const res = await this.httpClient.get(url);
        return new quote_1.Quote(params, res);
    }
    async getQuoteWithCustomPreset(params, body) {
        const bodyErr = body.validate();
        if (bodyErr) {
            throw new Error(bodyErr);
        }
        const queryParams = (0, params_1.concatQueryParams)(params.build());
        const bodyParams = body.build();
        const url = `${this.config.url}/${QuoterApi.Version}/${this.config.network}/quote/receive/${queryParams}`;
        const res = await this.httpClient.post(url, bodyParams);
        return new quote_1.Quote(params, res);
    }
}
exports.QuoterApi = QuoterApi;
QuoterApi.Version = 'v2.0';
//# sourceMappingURL=quoter.api.js.map