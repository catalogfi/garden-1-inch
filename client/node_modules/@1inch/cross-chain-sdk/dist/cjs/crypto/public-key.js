"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PublicKey = void 0;
const tslib_1 = require("tslib");
const ecies25519 = tslib_1.__importStar(require("ecies-25519"));
const byte_utils_1 = require("@1inch/byte-utils");
const key_1 = require("./key");
class PublicKey extends key_1.Key {
    static fromHexString(hex) {
        return new PublicKey((0, byte_utils_1.hexToUint8Array)(hex));
    }
    /**
     * Encrypt text with PublicKey using ecies-25519
     * @param text hex encoded string
     *
     * @returns encrypted text encoded as hex
     */
    async encrypt(text) {
        const bytes = (0, byte_utils_1.hexToUint8Array)(text);
        const encrypted = await ecies25519.encrypt(bytes, this.key);
        return (0, byte_utils_1.uint8ArrayToHex)(encrypted);
    }
}
exports.PublicKey = PublicKey;
//# sourceMappingURL=public-key.js.map