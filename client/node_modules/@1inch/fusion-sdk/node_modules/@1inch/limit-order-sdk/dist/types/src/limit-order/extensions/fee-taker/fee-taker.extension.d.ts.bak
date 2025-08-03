import { Fees } from './fees';
import { WhitelistHalfAddress } from './whitelist-half-address';
import { FeeCalculator } from './fee-calculator';
import { Address } from '../../../address';
import { Interaction } from '../../interaction';
import { Extension } from '../extension';
export declare class FeeTakerExtension {
    readonly address: Address;
    readonly fees: Fees;
    readonly whitelist: WhitelistHalfAddress;
    readonly makerPermit?: Interaction | undefined;
    readonly extraInteraction?: Interaction | undefined;
    readonly customReceiver?: Address | undefined;
    private static CUSTOM_RECEIVER_FLAG_BIT;
    private constructor();
    static new(address: Address, fees: Fees, whitelist?: Address[], extra?: {
        makerPermit?: Interaction;
        customReceiver?: Address;
        extraInteraction?: Interaction;
    }): FeeTakerExtension;
    static decode(bytes: string): FeeTakerExtension;
    static fromExtension(extension: Extension): FeeTakerExtension;
    getFeeCalculator(): FeeCalculator;
    build(): Extension;
    getTakingAmount(taker: Address, takingAmount: bigint): bigint;
    getMakingAmount(taker: Address, makingAmount: bigint): bigint;
    getResolverFee(taker: Address, takingAmount: bigint): bigint;
    getIntegratorFee(taker: Address, takingAmount: bigint): bigint;
    getProtocolShareOfIntegratorFee(taker: Address, takingAmount: bigint): bigint;
    getProtocolFee(taker: Address, takingAmount: bigint): bigint;
    private buildAmountGetterData;
    private buildInteractionData;
}
