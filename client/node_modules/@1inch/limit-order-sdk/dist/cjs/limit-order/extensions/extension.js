"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Extension = void 0;
const tslib_1 = require("tslib");
const ethers_1 = require("ethers");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const constants_1 = require("../../constants");
class Extension {
    constructor(data = Extension.EMPTY) {
        this.makerAssetSuffix = constants_1.ZX;
        this.takerAssetSuffix = constants_1.ZX;
        this.makingAmountData = constants_1.ZX;
        this.takingAmountData = constants_1.ZX;
        this.predicate = constants_1.ZX;
        this.makerPermit = constants_1.ZX;
        this.preInteraction = constants_1.ZX;
        this.postInteraction = constants_1.ZX;
        this.customData = constants_1.ZX;
        Object.entries(data).forEach(([key, val]) => (0, assert_1.default)((0, byte_utils_1.isHexString)(val) || val === constants_1.ZX, `${key} must be valid hex string`));
        this.makerAssetSuffix = data.makerAssetSuffix;
        this.takerAssetSuffix = data.takerAssetSuffix;
        this.makingAmountData = data.makingAmountData;
        this.takingAmountData = data.takingAmountData;
        this.predicate = data.predicate;
        this.makerPermit = data.makerPermit;
        this.preInteraction = data.preInteraction;
        this.postInteraction = data.postInteraction;
        this.customData = data.customData;
    }
    get hasPredicate() {
        return this.predicate !== constants_1.ZX;
    }
    get hasMakerPermit() {
        return this.makerPermit !== constants_1.ZX;
    }
    static decode(bytes) {
        if (bytes === constants_1.ZX) {
            return Extension.default();
        }
        const iter = byte_utils_1.BytesIter.HexString(bytes);
        let offsets = BigInt(iter.nextUint256());
        let consumed = 0;
        const data = {};
        for (const field of Extension.fields) {
            const offset = Number(offsets & byte_utils_1.UINT_32_MAX);
            const bytesCount = offset - consumed;
            data[field] = iter.nextBytes(bytesCount);
            consumed += bytesCount;
            offsets = offsets >> 32n;
        }
        data.customData = iter.rest();
        return new Extension(data);
    }
    static default() {
        return new Extension();
    }
    keccak256() {
        return BigInt((0, ethers_1.keccak256)(this.encode()));
    }
    isEmpty() {
        const allInteractions = this.getAll();
        const allInteractionsConcat = allInteractions.map(byte_utils_1.trim0x).join('') + (0, byte_utils_1.trim0x)(this.customData);
        return allInteractionsConcat.length === 0;
    }
    encode() {
        const allInteractions = this.getAll();
        const allInteractionsConcat = allInteractions.map(byte_utils_1.trim0x).join('') + (0, byte_utils_1.trim0x)(this.customData);
        const cumulativeSum = ((sum) => (value) => {
            sum += value;
            return sum;
        })(0);
        const offsets = allInteractions
            .map((a) => a.length / 2 - 1)
            .map(cumulativeSum)
            .reduce((acc, a, i) => acc + (BigInt(a) << BigInt(32 * i)), 0n);
        let extension = '0x';
        if (allInteractionsConcat.length > 0) {
            extension +=
                offsets.toString(16).padStart(64, '0') + allInteractionsConcat;
        }
        return extension;
    }
    getAll() {
        return Extension.fields.map((f) => this[f]);
    }
}
exports.Extension = Extension;
Extension.EMPTY = {
    makerAssetSuffix: constants_1.ZX,
    takerAssetSuffix: constants_1.ZX,
    makingAmountData: constants_1.ZX,
    takingAmountData: constants_1.ZX,
    predicate: constants_1.ZX,
    makerPermit: constants_1.ZX,
    preInteraction: constants_1.ZX,
    postInteraction: constants_1.ZX,
    customData: constants_1.ZX
};
Extension.fields = [
    'makerAssetSuffix',
    'takerAssetSuffix',
    'makingAmountData',
    'takingAmountData',
    'predicate',
    'makerPermit',
    'preInteraction',
    'postInteraction'
];
//# sourceMappingURL=extension.js.map