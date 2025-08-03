import { isAddress } from 'ethers';
import { add0x } from '@1inch/byte-utils';
import assert from 'assert';
export class Address {
    constructor(val) {
        assert(isAddress(val), `Invalid address ${val}`);
        this.val = val.toLowerCase();
    }
    static fromBigInt(val) {
        return new Address(add0x(val.toString(16).padStart(40, '0')));
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
        return add0x(this.val.slice(-20));
    }
}
Address.NATIVE_CURRENCY = new Address('0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee');
Address.ZERO_ADDRESS = new Address('0x0000000000000000000000000000000000000000');
//# sourceMappingURL=address.js.map