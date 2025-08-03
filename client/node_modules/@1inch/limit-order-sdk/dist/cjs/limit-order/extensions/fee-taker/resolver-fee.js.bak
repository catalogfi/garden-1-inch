"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ResolverFee = void 0;
const tslib_1 = require("tslib");
const assert_1 = tslib_1.__importDefault(require("assert"));
const address_1 = require("../../../address");
const bps_1 = require("../../../bps");
class ResolverFee {
    constructor(receiver, fee, whitelistDiscount = bps_1.Bps.ZERO) {
        this.receiver = receiver;
        this.fee = fee;
        this.whitelistDiscount = whitelistDiscount;
        if (receiver.isZero() && !fee.isZero()) {
            throw new Error('fee must be zero if receiver is zero address');
        }
        if (!receiver.isZero() && fee.isZero()) {
            throw new Error('receiver must be zero address if fee is zero');
        }
        if (fee.isZero() && !whitelistDiscount.isZero()) {
            throw new Error('whitelist discount must be zero if fee is zero');
        }
        (0, assert_1.default)(this.whitelistDiscount.value % 100n === 0n, `whitelist discount must have percent precision: 1%, 2% and so on`);
    }
}
exports.ResolverFee = ResolverFee;
ResolverFee.ZERO = new ResolverFee(address_1.Address.ZERO_ADDRESS, bps_1.Bps.ZERO);
//# sourceMappingURL=resolver-fee.js.map