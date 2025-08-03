"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Address = void 0;
const tslib_1 = require("tslib");
const ethers_1 = require("ethers");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
class Address {
    constructor(val) {
        (0, assert_1.default)((0, ethers_1.isAddress)(val), `Invalid address ${val}`);
        this.val = val.toLowerCase();
    }
    static fromBigInt(val) {
        return new Address((0, byte_utils_1.add0x)(val.toString(16).padStart(40, '0')));
    }
    static fromFirstBytes(bytes) {
        return new Address(bytes.slice(0, 42));
    }
    toString() {
        return this.val;
    }
    equal(other) {
        return this.val === other.val;
    }
    isNative() {
        return this.equal(Address.NATIVE_CURRENCY);
    }
    isZero() {
        return this.equal(Address.ZERO_ADDRESS);
    }
    lastHalf() {
        return (0, byte_utils_1.add0x)(this.val.slice(-20));
    }
}
exports.Address = Address;
Address.NATIVE_CURRENCY = new Address('0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee');
Address.ZERO_ADDRESS = new Address('0x0000000000000000000000000000000000000000');
//# sourceMappingURL=address.js.map