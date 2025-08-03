"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PrivateKeyProviderConnector = void 0;
const ethers_1 = require("ethers");
const utils_1 = require("../../utils");
class PrivateKeyProviderConnector {
    constructor(privateKey, web3Provider) {
        this.privateKey = privateKey;
        this.web3Provider = web3Provider;
        this.wallet = new ethers_1.Wallet((0, utils_1.add0x)(privateKey));
    }
    signTypedData(_walletAddress, typedData) {
        const primaryTypes = { ...typedData.types };
        delete primaryTypes['EIP712Domain'];
        return this.wallet.signTypedData(typedData.domain, primaryTypes, typedData.message);
    }
    ethCall(contractAddress, callData) {
        return this.web3Provider.eth.call({
            to: contractAddress,
            data: callData
        });
    }
}
exports.PrivateKeyProviderConnector = PrivateKeyProviderConnector;
//# sourceMappingURL=private-key-provider.connector.js.map