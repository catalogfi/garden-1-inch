"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PrivateKey = void 0;
const tslib_1 = require("tslib");
const ecies25519 = tslib_1.__importStar(require("ecies-25519"));
const byte_utils_1 = require("@1inch/byte-utils");
const key_1 = require("./key");
class PrivateKey extends key_1.Key {
    static fromHexString(hex) {
        return new PrivateKey((0, byte_utils_1.hexToUint8Array)(hex));
    }
    /**
     * Decrypt cipher with PrivateKey using ecies-25519
     * @param cipher hex string
     *
     * @returns hex encoded string
     */
    async decrypt(cipher) {
        const bytes = (0, byte_utils_1.hexToUint8Array)(cipher);
        const decrypted = await ecies25519.decrypt(bytes, this.key);
        return (0, byte_utils_1.uint8ArrayToHex)(decrypted);
    }
}
exports.PrivateKey = PrivateKey;
//# sourceMappingURL=private-key.js.map