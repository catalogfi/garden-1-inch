"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Interaction = void 0;
const tslib_1 = require("tslib");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const address_1 = require("../address");
class Interaction {
    constructor(target, data) {
        this.target = target;
        this.data = data;
        (0, assert_1.default)((0, byte_utils_1.isHexBytes)(data), 'Interaction data must be valid hex bytes');
    }
    static decode(bytes) {
        const iter = byte_utils_1.BytesIter.HexString(bytes);
        return new Interaction(new address_1.Address(iter.nextUint160()), iter.rest());
    }
    encode() {
        return this.target.toString() + (0, byte_utils_1.trim0x)(this.data);
    }
}
exports.Interaction = Interaction;
//# sourceMappingURL=interaction.js.map