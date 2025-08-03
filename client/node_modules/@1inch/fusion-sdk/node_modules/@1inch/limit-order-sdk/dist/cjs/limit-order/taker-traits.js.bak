"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TakerTraits = exports.AmountMode = void 0;
const byte_utils_1 = require("@1inch/byte-utils");
const constants_1 = require("../constants");
var AmountMode;
(function (AmountMode) {
    AmountMode[AmountMode["taker"] = 0] = "taker";
    AmountMode[AmountMode["maker"] = 1] = "maker";
})(AmountMode || (exports.AmountMode = AmountMode = {}));
class TakerTraits {
    constructor(flag, data) {
        this.flags = new byte_utils_1.BN(flag);
        this.receiver = data.receiver;
        this.extension = data.extension;
        this.interaction = data.interaction;
    }
    static default() {
        return new TakerTraits(0n, {});
    }
    getAmountMode() {
        return this.flags.getBit(TakerTraits.MAKER_AMOUNT_FLAG);
    }
    setAmountMode(mode) {
        this.flags = this.flags.setBit(TakerTraits.MAKER_AMOUNT_FLAG, mode);
        return this;
    }
    isNativeUnwrapEnabled() {
        return this.flags.getBit(TakerTraits.UNWRAP_WETH_FLAG) === 1;
    }
    enableNativeUnwrap() {
        this.flags = this.flags.setBit(TakerTraits.UNWRAP_WETH_FLAG, 1);
        return this;
    }
    disableNativeUnwrap() {
        this.flags = this.flags.setBit(TakerTraits.UNWRAP_WETH_FLAG, 0);
        return this;
    }
    isOrderPermitSkipped() {
        return Boolean(this.flags.getBit(TakerTraits.SKIP_ORDER_PERMIT_FLAG));
    }
    skipOrderPermit() {
        this.flags = this.flags.setBit(TakerTraits.SKIP_ORDER_PERMIT_FLAG, 1);
        return this;
    }
    isPermit2Enabled() {
        return this.flags.getBit(TakerTraits.USE_PERMIT2_FLAG) === 1;
    }
    enablePermit2() {
        this.flags = this.flags.setBit(TakerTraits.USE_PERMIT2_FLAG, 1);
        return this;
    }
    disablePermit2() {
        this.flags = this.flags.setBit(TakerTraits.USE_PERMIT2_FLAG, 0);
        return this;
    }
    setReceiver(receiver) {
        this.receiver = receiver;
        return this;
    }
    removeReceiver() {
        this.receiver = undefined;
        return this;
    }
    setExtension(ext) {
        this.extension = ext;
        return this;
    }
    removeExtension() {
        this.extension = undefined;
        return this;
    }
    setAmountThreshold(threshold) {
        this.flags = this.flags.setMask(TakerTraits.THRESHOLD_MASK, threshold);
        return this;
    }
    getAmountThreshold() {
        return this.flags.getMask(TakerTraits.THRESHOLD_MASK).value;
    }
    removeAmountThreshold() {
        this.flags = this.flags.setMask(TakerTraits.THRESHOLD_MASK, 0n);
        return this;
    }
    setInteraction(interaction) {
        this.interaction = interaction;
        return this;
    }
    removeInteraction() {
        this.interaction = undefined;
        return this;
    }
    encode() {
        const extensionLen = this.extension
            ? (0, byte_utils_1.getBytesCount)(this.extension.encode())
            : 0n;
        const interactionLen = this.interaction
            ? (0, byte_utils_1.getBytesCount)(this.interaction.encode())
            : 0n;
        const flags = this.flags
            .setBit(TakerTraits.ARGS_HAS_RECEIVER, this.receiver ? 1 : 0)
            .setMask(TakerTraits.ARGS_EXTENSION_LENGTH_MASK, extensionLen)
            .setMask(TakerTraits.ARGS_INTERACTION_LENGTH_MASK, interactionLen);
        const args = (this.receiver?.toString() || constants_1.ZX) +
            (0, byte_utils_1.trim0x)(this.extension?.encode() || '') +
            (0, byte_utils_1.trim0x)(this.interaction?.encode() || '');
        return {
            trait: flags.value,
            args
        };
    }
}
exports.TakerTraits = TakerTraits;
TakerTraits.MAKER_AMOUNT_FLAG = 255n;
TakerTraits.UNWRAP_WETH_FLAG = 254n;
TakerTraits.SKIP_ORDER_PERMIT_FLAG = 253n;
TakerTraits.USE_PERMIT2_FLAG = 252n;
TakerTraits.ARGS_HAS_RECEIVER = 251n;
TakerTraits.THRESHOLD_MASK = new byte_utils_1.BitMask(0n, 185n);
TakerTraits.ARGS_INTERACTION_LENGTH_MASK = new byte_utils_1.BitMask(200n, 224n);
TakerTraits.ARGS_EXTENSION_LENGTH_MASK = new byte_utils_1.BitMask(224n, 248n);
//# sourceMappingURL=taker-traits.js.map