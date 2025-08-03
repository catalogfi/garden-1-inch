import { EIP712TypedData } from '@1inch/limit-order-sdk';
import { BlockchainProviderConnector } from './blockchain-provider.connector';
interface TransactionConfig {
    data?: string;
    to?: string;
}
export interface Web3Like {
    eth: {
        call(transactionConfig: TransactionConfig): Promise<string>;
    };
    extend(extension: unknown): any;
}
export declare class Web3ProviderConnector implements BlockchainProviderConnector {
    protected readonly web3Provider: Web3Like;
    constructor(web3Provider: Web3Like);
    signTypedData(walletAddress: string, typedData: EIP712TypedData): Promise<string>;
    ethCall(contractAddress: string, callData: string): Promise<string>;
}
export {};
