const FEE_BASE = 100000n;
const BPS_BASE = 10000n;
const BPS_TO_RATIO_NUMERATOR = FEE_BASE / BPS_BASE;
export function bpsToRatioFormat(bps) {
    if (!bps) {
        return 0n;
    }
    return BigInt(bps) * BPS_TO_RATIO_NUMERATOR;
}
export function addRatioToAmount(amount, ratio) {
    return amount + (amount * ratio) / FEE_BASE;
}
//# sourceMappingURL=utils.js.map