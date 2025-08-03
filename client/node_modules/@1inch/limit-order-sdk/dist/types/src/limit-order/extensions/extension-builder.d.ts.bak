import { Extension } from './extension';
import { IExtensionBuilder } from '../types';
import { Interaction } from '../interaction';
import { Address } from '../../address';
export declare class ExtensionBuilder implements IExtensionBuilder {
    private makerAssetSuffix;
    private takerAssetSuffix;
    private makingAmountData;
    private takingAmountData;
    private predicate;
    private makerPermit;
    private preInteraction;
    private postInteraction;
    private customData;
    withMakerAssetSuffix(suffix: string): this;
    withTakerAssetSuffix(suffix: string): this;
    withMakingAmountData(address: Address, data: string): this;
    withTakingAmountData(address: Address, data: string): this;
    withPredicate(predicate: string): this;
    withMakerPermit(tokenFrom: Address, permitData: string): this;
    withPreInteraction(interaction: Interaction): this;
    withPostInteraction(interaction: Interaction): this;
    withCustomData(data: string): this;
    build(): Extension;
}
