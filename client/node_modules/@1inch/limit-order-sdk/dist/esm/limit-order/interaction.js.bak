import { BytesIter, isHexBytes, trim0x } from '@1inch/byte-utils';
import assert from 'assert';
import { Address } from '../address';
export class Interaction {
    constructor(target, data) {
        this.target = target;
        this.data = data;
        assert(isHexBytes(data), 'Interaction data must be valid hex bytes');
    }
    static decode(bytes) {
        const iter = BytesIter.HexString(bytes);
        return new Interaction(new Address(iter.nextUint160()), iter.rest());
    }
    encode() {
        return this.target.toString() + trim0x(this.data);
    }
}
//# sourceMappingURL=interaction.js.map