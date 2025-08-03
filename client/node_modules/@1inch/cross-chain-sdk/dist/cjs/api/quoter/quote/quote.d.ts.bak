import { Address } from '@1inch/fusion-sdk';
import { CrossChainOrderParamsData } from './types';
import { Cost, PresetEnum, QuoterResponse, TimeLocksRaw } from '../types';
import { Preset } from '../preset';
import { QuoterRequest } from '../quoter.request';
import { CrossChainOrder } from '../../../cross-chain-order';
import { SupportedChain } from '../../../chains';
export declare class Quote {
    private readonly params;
    readonly quoteId: string | null;
    readonly srcTokenAmount: bigint;
    readonly dstTokenAmount: bigint;
    readonly presets: {
        [PresetEnum.fast]: Preset;
        [PresetEnum.slow]: Preset;
        [PresetEnum.medium]: Preset;
        [PresetEnum.custom]?: Preset;
    };
    readonly srcEscrowFactory: Address;
    readonly dstEscrowFactory: Address;
    readonly timeLocks: TimeLocksRaw;
    readonly srcSafetyDeposit: bigint;
    readonly dstSafetyDeposit: bigint;
    readonly whitelist: Address[];
    readonly recommendedPreset: PresetEnum;
    readonly prices: Cost;
    readonly volume: Cost;
    readonly slippage: number;
    constructor(params: QuoterRequest, response: QuoterResponse);
    get srcChainId(): SupportedChain;
    get dstChainId(): SupportedChain;
    createOrder(params: CrossChainOrderParamsData): CrossChainOrder;
    getPreset(type?: PresetEnum): Preset;
    private getWhitelist;
}
