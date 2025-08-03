import { FeeTakerExtension } from './extensions/fee-taker/fee-taker.extension';
import { LimitOrder } from './limit-order';
import { LimitOrderV4Struct, OrderInfoData } from './types';
import { MakerTraits } from './maker-traits';
import { Extension } from './extensions/extension';
import { Address } from '../address';
export declare class LimitOrderWithFee extends LimitOrder {
    readonly feeExtension: FeeTakerExtension;
    constructor(orderInfo: Omit<OrderInfoData, 'receiver'>, makerTraits: MakerTraits | undefined, feeExtension: FeeTakerExtension);
    static withRandomNonce(orderInfo: Omit<OrderInfoData, 'receiver'>, feeExtension: FeeTakerExtension, makerTraits?: MakerTraits): LimitOrderWithFee;
    static fromDataAndExtension(data: LimitOrderV4Struct, extension: Extension): LimitOrderWithFee;
    getTakingAmount(taker: Address, makingAmount?: bigint): bigint;
    getMakingAmount(taker: Address, takingAmount?: bigint): bigint;
    getResolverFee(taker: Address, makingAmount?: bigint): bigint;
    getIntegratorFee(taker: Address, makingAmount?: bigint): bigint;
    getProtocolFee(taker: Address, makingAmount?: bigint): bigint;
}
