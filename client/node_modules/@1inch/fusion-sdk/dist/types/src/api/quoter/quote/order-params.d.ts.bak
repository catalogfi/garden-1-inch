import { Address } from '@1inch/limit-order-sdk';
import { FusionOrderParamsData } from './types';
import { PresetEnum } from '../types';
export declare class FusionOrderParams {
    readonly preset: PresetEnum;
    readonly receiver: Address;
    readonly permit: string | undefined;
    readonly nonce: bigint | undefined;
    readonly delayAuctionStartTimeBy: bigint;
    readonly isPermit2?: boolean;
    constructor(params: FusionOrderParamsData);
    static new(params: FusionOrderParamsData): FusionOrderParams;
}
