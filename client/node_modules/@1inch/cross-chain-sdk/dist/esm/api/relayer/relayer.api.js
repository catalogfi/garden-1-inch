export class RelayerApi {
    constructor(config, httpClient) {
        this.config = config;
        this.httpClient = httpClient;
    }
    submit(params) {
        const url = `${this.config.url}/${RelayerApi.Version}/submit`;
        return this.httpClient.post(url, params);
    }
    submitBatch(params) {
        const url = `${this.config.url}/${RelayerApi.Version}/submit/many`;
        return this.httpClient.post(url, params);
    }
    submitSecret(orderHash, secret) {
        const url = `${this.config.url}/${RelayerApi.Version}/submit/secret`;
        return this.httpClient.post(url, {
            orderHash,
            secret
        });
    }
}
RelayerApi.Version = 'v1.0';
//# sourceMappingURL=relayer.api.js.map