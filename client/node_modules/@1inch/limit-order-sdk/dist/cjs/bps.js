"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Bps = void 0;
const tslib_1 = require("tslib");
const assert_1 = tslib_1.__importDefault(require("assert"));
class Bps {
    constructor(value) {
        this.value = value;
        (0, assert_1.default)(value >= 0 && value <= 10000, `invalid bps ${value}`);
    }
    static fromPercent(val, base = 1n) {
        return new Bps(BigInt(100 * val) / base);
    }
    static fromFraction(val, base = 1n) {
        return new Bps(BigInt(10000 * val) / base);
    }
    equal(other) {
        return this.value === other.value;
    }
    isZero() {
        return this.value === 0n;
    }
    toPercent(base = 1n) {
        return Number(this.value * base) / 100;
    }
    toFraction(base = 1n) {
        return Number(this.value * base) / 10000;
    }
    toString() {
        return this.value.toString();
    }
}
exports.Bps = Bps;
Bps.ZERO = new Bps(0n);
//# sourceMappingURL=bps.js.map