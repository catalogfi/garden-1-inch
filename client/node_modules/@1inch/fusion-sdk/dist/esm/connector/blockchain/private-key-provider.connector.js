import { Wallet } from 'ethers';
import { add0x } from '../../utils';
export class PrivateKeyProviderConnector {
    constructor(privateKey, web3Provider) {
        this.privateKey = privateKey;
        this.web3Provider = web3Provider;
        this.wallet = new Wallet(add0x(privateKey));
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
//# sourceMappingURL=private-key-provider.connector.js.map