"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ExtensionBuilder = void 0;
const tslib_1 = require("tslib");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const extension_1 = require("./extension");
const constants_1 = require("../../constants");
class ExtensionBuilder {
    constructor() {
        this.makerAssetSuffix = constants_1.ZX;
        this.takerAssetSuffix = constants_1.ZX;
        this.makingAmountData = constants_1.ZX;
        this.takingAmountData = constants_1.ZX;
        this.predicate = constants_1.ZX;
        this.makerPermit = constants_1.ZX;
        this.preInteraction = constants_1.ZX;
        this.postInteraction = constants_1.ZX;
        this.customData = constants_1.ZX;
    }
    withMakerAssetSuffix(suffix) {
        (0, assert_1.default)((0, byte_utils_1.isHexString)(suffix), 'MakerAssetSuffix must be valid hex string');
        this.makerAssetSuffix = suffix;
        return this;
    }
    withTakerAssetSuffix(suffix) {
        (0, assert_1.default)((0, byte_utils_1.isHexString)(suffix), 'TakerAssetSuffix must be valid hex string');
        this.takerAssetSuffix = suffix;
        return this;
    }
    withMakingAmountData(address, data) {
        (0, assert_1.default)((0, byte_utils_1.isHexString)(data), 'MakingAmountData must be valid hex string');
        this.makingAmountData = address.toString() + (0, byte_utils_1.trim0x)(data);
        return this;
    }
    withTakingAmountData(address, data) {
        (0, assert_1.default)((0, byte_utils_1.isHexString)(data), 'TakingAmountData must be valid hex string');
        this.takingAmountData = address.toString() + (0, byte_utils_1.trim0x)(data);
        return this;
    }
    withPredicate(predicate) {
        (0, assert_1.default)((0, byte_utils_1.isHexString)(predicate), 'Predicate must be valid hex string');
        this.predicate = predicate;
        return this;
    }
    withMakerPermit(tokenFrom, permitData) {
        (0, assert_1.default)((0, byte_utils_1.isHexString)(permitData), 'Permit data must be valid hex string');
        this.makerPermit = tokenFrom.toString() + (0, byte_utils_1.trim0x)(permitData);
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
        (0, assert_1.default)((0, byte_utils_1.isHexString)(data), 'Custom data must be valid hex string');
        this.customData = data;
        return this;
    }
    build() {
        return new extension_1.Extension({
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
exports.ExtensionBuilder = ExtensionBuilder;
//# sourceMappingURL=extension-builder.js.map