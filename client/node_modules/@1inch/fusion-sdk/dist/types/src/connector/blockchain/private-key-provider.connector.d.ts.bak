import { EIP712TypedData } from '@1inch/limit-order-sdk';
import { BlockchainProviderConnector } from './blockchain-provider.connector';
import { Web3Like } from './web3-provider-connector';
export declare class PrivateKeyProviderConnector implements BlockchainProviderConnector {
    readonly privateKey: string;
    protected readonly web3Provider: Web3Like;
    private readonly wallet;
    constructor(privateKey: string, web3Provider: Web3Like);
    signTypedData(_walletAddress: string, typedData: EIP712TypedData): Promise<string>;
    ethCall(contractAddress: string, callData: string): Promise<string>;
}
