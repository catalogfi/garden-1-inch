import { Address } from '@1inch/fusion-sdk';
import { QuoterRequestParams } from './types';
import { SupportedChain } from '../../chains';
export declare class QuoterRequest {
    readonly srcChain: SupportedChain;
    readonly dstChain: SupportedChain;
    readonly srcTokenAddress: Address;
    readonly dstTokenAddress: Address;
    readonly amount: bigint;
    readonly walletAddress: Address;
    readonly enableEstimate: boolean;
    readonly permit: string | undefined;
    readonly fee: number | undefined;
    readonly source: string;
    readonly isPermit2: boolean;
    constructor(params: QuoterRequestParams);
    static new(params: QuoterRequestParams): QuoterRequest;
    build(): QuoterRequestParams;
}
