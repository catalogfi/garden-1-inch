import { isHexString, trim0x } from '@1inch/byte-utils';
import assert from 'assert';
import { Extension } from './extension';
import { ZX } from '../../constants';
export class ExtensionBuilder {
    constructor() {
        this.makerAssetSuffix = ZX;
        this.takerAssetSuffix = ZX;
        this.makingAmountData = ZX;
        this.takingAmountData = ZX;
        this.predicate = ZX;
        this.makerPermit = ZX;
        this.preInteraction = ZX;
        this.postInteraction = ZX;
        this.customData = ZX;
    }
    withMakerAssetSuffix(suffix) {
        assert(isHexString(suffix), 'MakerAssetSuffix must be valid hex string');
        this.makerAssetSuffix = suffix;
        return this;
    }
    withTakerAssetSuffix(suffix) {
        assert(isHexString(suffix), 'TakerAssetSuffix must be valid hex string');
        this.takerAssetSuffix = suffix;
        return this;
    }
    withMakingAmountData(address, data) {
        assert(isHexString(data), 'MakingAmountData must be valid hex string');
        this.makingAmountData = address.toString() + trim0x(data);
        return this;
    }
    withTakingAmountData(address, data) {
        assert(isHexString(data), 'TakingAmountData must be valid hex string');
        this.takingAmountData = address.toString() + trim0x(data);
        return this;
    }
    withPredicate(predicate) {
        assert(isHexString(predicate), 'Predicate must be valid hex string');
        this.predicate = predicate;
        return this;
    }
    withMakerPermit(tokenFrom, permitData) {
        assert(isHexString(permitData), 'Permit data must be valid hex string');
        this.makerPermit = tokenFrom.toString() + trim0x(permitData);
        return this;
    }
    withPreInteraction(interaction) {
        this.preInteraction = interaction.encode();
        return this;
    }
    withPostInteraction(interaction) {
        this.postInteraction = interaction.encode();
        return this;
    }
    withCustomData(data) {
        assert(isHexString(data), 'Custom data must be valid hex string');
        this.customData = data;
        return this;
    }
    build() {
        return new Extension({
            makerAssetSuffix: this.makerAssetSuffix,
            takerAssetSuffix: this.takerAssetSuffix,
            makingAmountData: this.makingAmountData,
            takingAmountData: this.takingAmountData,
            predicate: this.predicate,
            makerPermit: this.makerPermit,
            preInteraction: this.preInteraction,
            postInteraction: this.postInteraction,
            customData: this.customData
        });
    }
}
//# sourceMappingURL=extension-builder.js.map