export class RelayerRequest {
    constructor(params) {
        this.order = params.order;
        this.signature = params.signature;
        this.quoteId = params.quoteId;
        this.extension = params.extension;
        this.srcChainId = params.srcChainId;
        this.secretHashes = params.secretHashes;
    }
    static new(params) {
        return new RelayerRequest(params);
    }
    build() {
        return {
            order: this.order,
            signature: this.signature,
            quoteId: this.quoteId,
            extension: this.extension,
            srcChainId: this.srcChainId,
            secretHashes: this.secretHashes
        };
    }
}
//# sourceMappingURL=relayer.request.js.map