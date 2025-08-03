import { Fees } from './fees';
import { Whitelist } from './types';
import { Address } from '../../../address';
export declare class FeeCalculator {
    readonly fees: Fees;
    readonly whitelist: Whitelist;
    constructor(fees: Fees, whitelist: Whitelist);
    getTakingAmount(taker: Address, orderTakingAmount: bigint): bigint;
    getMakingAmount(taker: Address, makingAmount: bigint): bigint;
    getResolverFee(taker: Address, orderTakingAmount: bigint): bigint;
    getIntegratorFee(taker: Address, orderTakingAmount: bigint): bigint;
    getProtocolShareOfIntegratorFee(taker: Address, orderTakingAmount: bigint): bigint;
    getProtocolFee(taker: Address, orderTakingAmount: bigint): bigint;
    private getFeesForTaker;
}
