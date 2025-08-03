import { add0x, BitMask, BN } from '@1inch/byte-utils';
import assert from 'assert';
export class MakerTraits {
    constructor(val) {
        this.value = new BN(val);
    }
    static default() {
        return new MakerTraits(0n);
    }
    allowedSender() {
        return this.value
            .getMask(MakerTraits.ALLOWED_SENDER_MASK)
            .value.toString(16)
            .padStart(20, '0');
    }
    isPrivate() {
        return this.value.getMask(MakerTraits.ALLOWED_SENDER_MASK).value !== 0n;
    }
    withAllowedSender(sender) {
        assert(!sender.isZero(), 'Use withAnySender() to remove sender check');
        const lastHalf = add0x(sender.toString().slice(-20));
        this.value = this.value.setMask(MakerTraits.ALLOWED_SENDER_MASK, BigInt(lastHalf));
        return this;
    }
    withAnySender() {
        this.value = this.value.setMask(MakerTraits.ALLOWED_SENDER_MASK, BigInt(0));
        return this;
    }
    expiration() {
        const timestampSec = this.value.getMask(MakerTraits.EXPIRATION_MASK);
        if (timestampSec.isZero()) {
            return null;
        }
        return timestampSec.value;
    }
    withExpiration(expiration) {
        const expirationSec = expiration === null ? 0n : expiration;
        this.value = this.value.setMask(MakerTraits.EXPIRATION_MASK, expirationSec);
        return this;
    }
    nonceOrEpoch() {
        return this.value.getMask(MakerTraits.NONCE_OR_EPOCH_MASK).value;
    }
    withNonce(nonce) {
        this.value = this.value.setMask(MakerTraits.NONCE_OR_EPOCH_MASK, nonce);
        return this;
    }
    withEpoch(series, epoch) {
        this.setSeries(series);
        this.enableEpochManagerCheck();
        return this.withNonce(epoch);
    }
    series() {
        return this.value.getMask(MakerTraits.SERIES_MASK).value;
    }
    hasExtension() {
        return this.value.getBit(MakerTraits.HAS_EXTENSION_FLAG) === 1;
    }
    withExtension() {
        this.value = this.value.setBit(MakerTraits.HAS_EXTENSION_FLAG, 1);
        return this;
    }
    isPartialFillAllowed() {
        return this.value.getBit(MakerTraits.NO_PARTIAL_FILLS_FLAG) === 0;
    }
    disablePartialFills() {
        this.value = this.value.setBit(MakerTraits.NO_PARTIAL_FILLS_FLAG, 1);
        return this;
    }
    allowPartialFills() {
        this.value = this.value.setBit(MakerTraits.NO_PARTIAL_FILLS_FLAG, 0);
        return this;
    }
    setPartialFills(val) {
        return val ? this.allowPartialFills() : this.disablePartialFills();
    }
    isMultipleFillsAllowed() {
        return this.value.getBit(MakerTraits.ALLOW_MULTIPLE_FILLS_FLAG) === 1;
    }
    allowMultipleFills() {
        this.value = this.value.setBit(MakerTraits.ALLOW_MULTIPLE_FILLS_FLAG, 1);
        return this;
    }
    disableMultipleFills() {
        this.value = this.value.setBit(MakerTraits.ALLOW_MULTIPLE_FILLS_FLAG, 0);
        return this;
    }
    setMultipleFills(val) {
        return val ? this.allowMultipleFills() : this.disableMultipleFills();
    }
    hasPreInteraction() {
        return this.value.getBit(MakerTraits.PRE_INTERACTION_CALL_FLAG) === 1;
    }
    enablePreInteraction() {
        this.value = this.value.setBit(MakerTraits.PRE_INTERACTION_CALL_FLAG, 1);
        return this;
    }
    disablePreInteraction() {
        this.value = this.value.setBit(MakerTraits.PRE_INTERACTION_CALL_FLAG, 0);
        return this;
    }
    hasPostInteraction() {
        return this.value.getBit(MakerTraits.POST_INTERACTION_CALL_FLAG) === 1;
    }
    enablePostInteraction() {
        this.value = this.value.setBit(MakerTraits.POST_INTERACTION_CALL_FLAG, 1);
        return this;
    }
    disablePostInteraction() {
        this.value = this.value.setBit(MakerTraits.POST_INTERACTION_CALL_FLAG, 0);
        return this;
    }
    isEpochManagerEnabled() {
        return (this.value.getBit(MakerTraits.NEED_CHECK_EPOCH_MANAGER_FLAG) === 1);
    }
    isPermit2() {
        return this.value.getBit(MakerTraits.USE_PERMIT2_FLAG) === 1;
    }
    enablePermit2() {
        this.value = this.value.setBit(MakerTraits.USE_PERMIT2_FLAG, 1);
        return this;
    }
    disablePermit2() {
        this.value = this.value.setBit(MakerTraits.USE_PERMIT2_FLAG, 0);
        return this;
    }
    isNativeUnwrapEnabled() {
        return this.value.getBit(MakerTraits.UNWRAP_WETH_FLAG) === 1;
    }
    enableNativeUnwrap() {
        this.value = this.value.setBit(MakerTraits.UNWRAP_WETH_FLAG, 1);
        return this;
    }
    disableNativeUnwrap() {
        this.value = this.value.setBit(MakerTraits.UNWRAP_WETH_FLAG, 0);
        return this;
    }
    asBigInt() {
        return this.value.value;
    }
    isBitInvalidatorMode() {
        return !this.isPartialFillAllowed() || !this.isMultipleFillsAllowed();
    }
    enableEpochManagerCheck() {
        assert(!this.isBitInvalidatorMode(), 'Epoch manager allowed only when partialFills and multipleFills enabled');
        this.value = this.value.setBit(MakerTraits.NEED_CHECK_EPOCH_MANAGER_FLAG, 1);
    }
    setSeries(series) {
        this.value = this.value.setMask(MakerTraits.SERIES_MASK, series);
    }
}
MakerTraits.ALLOWED_SENDER_MASK = new BitMask(0n, 80n);
MakerTraits.EXPIRATION_MASK = new BitMask(80n, 120n);
MakerTraits.NONCE_OR_EPOCH_MASK = new BitMask(120n, 160n);
MakerTraits.SERIES_MASK = new BitMask(160n, 200n);
MakerTraits.NO_PARTIAL_FILLS_FLAG = 255n;
MakerTraits.ALLOW_MULTIPLE_FILLS_FLAG = 254n;
MakerTraits.PRE_INTERACTION_CALL_FLAG = 252n;
MakerTraits.POST_INTERACTION_CALL_FLAG = 251n;
MakerTraits.NEED_CHECK_EPOCH_MANAGER_FLAG = 250n;
MakerTraits.HAS_EXTENSION_FLAG = 249n;
MakerTraits.USE_PERMIT2_FLAG = 248n;
MakerTraits.UNWRAP_WETH_FLAG = 247n;
//# sourceMappingURL=maker-traits.js.map