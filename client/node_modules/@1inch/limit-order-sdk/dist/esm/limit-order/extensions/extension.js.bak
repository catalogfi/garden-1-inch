import { keccak256 } from 'ethers';
import { BytesIter, isHexString, trim0x, UINT_32_MAX } from '@1inch/byte-utils';
import assert from 'assert';
import { ZX } from '../../constants';
export class Extension {
    constructor(data = Extension.EMPTY) {
        this.makerAssetSuffix = ZX;
        this.takerAssetSuffix = ZX;
        this.makingAmountData = ZX;
        this.takingAmountData = ZX;
        this.predicate = ZX;
        this.makerPermit = ZX;
        this.preInteraction = ZX;
        this.postInteraction = ZX;
        this.customData = ZX;
        Object.entries(data).forEach(([key, val]) => assert(isHexString(val) || val === ZX, `${key} must be valid hex string`));
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
        return this.predicate !== ZX;
    }
    get hasMakerPermit() {
        return this.makerPermit !== ZX;
    }
    static decode(bytes) {
        if (bytes === ZX) {
            return Extension.default();
        }
        const iter = BytesIter.HexString(bytes);
        let offsets = BigInt(iter.nextUint256());
        let consumed = 0;
        const data = {};
        for (const field of Extension.fields) {
            const offset = Number(offsets & UINT_32_MAX);
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
        return BigInt(keccak256(this.encode()));
    }
    isEmpty() {
        const allInteractions = this.getAll();
        const allInteractionsConcat = allInteractions.map(trim0x).join('') + trim0x(this.customData);
        return allInteractionsConcat.length === 0;
    }
    encode() {
        const allInteractions = this.getAll();
        const allInteractionsConcat = allInteractions.map(trim0x).join('') + trim0x(this.customData);
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
Extension.EMPTY = {
    makerAssetSuffix: ZX,
    takerAssetSuffix: ZX,
    makingAmountData: ZX,
    takingAmountData: ZX,
    predicate: ZX,
    makerPermit: ZX,
    preInteraction: ZX,
    postInteraction: ZX,
    customData: ZX
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