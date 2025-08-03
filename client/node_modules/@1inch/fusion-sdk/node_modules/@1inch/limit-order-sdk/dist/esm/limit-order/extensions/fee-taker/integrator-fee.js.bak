import { Address } from '../../../address';
import { Bps } from '../../../bps';
export class IntegratorFee {
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
IntegratorFee.ZERO = new IntegratorFee(Address.ZERO_ADDRESS, Address.ZERO_ADDRESS, Bps.ZERO, Bps.ZERO);
//# sourceMappingURL=integrator-fee.js.map