import { AxiosProviderConnector } from '../../connector';
export class RelayerApi {
    constructor(config, httpClient) {
        this.config = config;
        this.httpClient = httpClient;
    }
    static new(config, httpClient = new AxiosProviderConnector(config.authKey)) {
        return new RelayerApi(config, httpClient);
    }
    submit(params) {
        const url = `${this.config.url}/${RelayerApi.Version}/${this.config.network}/order/submit`;
        return this.httpClient.post(url, params);
    }
    submitBatch(params) {
        const url = `${this.config.url}/${RelayerApi.Version}/${this.config.network}/order/submit/many`;
        return this.httpClient.post(url, params);
    }
}
RelayerApi.Version = 'v2.0';
//# sourceMappingURL=relayer.api.js.map