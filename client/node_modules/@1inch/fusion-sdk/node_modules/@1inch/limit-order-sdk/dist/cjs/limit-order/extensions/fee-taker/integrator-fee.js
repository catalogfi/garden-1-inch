"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.IntegratorFee = void 0;
const address_1 = require("../../../address");
const bps_1 = require("../../../bps");
class IntegratorFee {
    constructor(integrator, protocol, fee, share) {
        this.integrator = integrator;
        this.protocol = protocol;
        this.fee = fee;
        this.share = share;
        if (fee.isZero()) {
            if (!share.isZero()) {
                throw new Error('integrator share must be zero if fee is zero');
            }
            if (!integrator.isZero()) {
                throw new Error('integrator address must be zero if fee is zero');
            }
            if (!protocol.isZero()) {
                throw new Error('protocol address must be zero if fee is zero');
            }
        }
        if ((integrator.isZero() || protocol.isZero()) && !fee.isZero()) {
            throw new Error('fee must be zero if integrator or protocol is zero address');
        }
    }
}
exports.IntegratorFee = IntegratorFee;
IntegratorFee.ZERO = new IntegratorFee(address_1.Address.ZERO_ADDRESS, address_1.Address.ZERO_ADDRESS, bps_1.Bps.ZERO, bps_1.Bps.ZERO);
//# sourceMappingURL=integrator-fee.js.map