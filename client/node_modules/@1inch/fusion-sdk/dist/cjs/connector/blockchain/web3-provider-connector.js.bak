"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Web3ProviderConnector = void 0;
class Web3ProviderConnector {
    constructor(web3Provider) {
        this.web3Provider = web3Provider;
    }
    signTypedData(walletAddress, typedData) {
        const extendedWeb3 = this.web3Provider.extend({
            methods: [
                {
                    name: 'signTypedDataV4',
                    call: 'eth_signTypedData_v4',
                    params: 2
                }
            ]
        });
        return extendedWeb3.signTypedDataV4(walletAddress, JSON.stringify(typedData));
    }
    ethCall(contractAddress, callData) {
        return this.web3Provider.eth.call({
            to: contractAddress,
            data: callData
        });
    }
}
exports.Web3ProviderConnector = Web3ProviderConnector;
//# sourceMappingURL=web3-provider-connector.js.map