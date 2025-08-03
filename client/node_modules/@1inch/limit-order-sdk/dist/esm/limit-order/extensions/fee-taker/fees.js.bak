import assert from 'assert';
import { ResolverFee } from './resolver-fee';
import { IntegratorFee } from './integrator-fee';
export class Fees {
    constructor(resolver, integrator) {
        this.resolver = resolver;
        this.integrator = integrator;
        if (!resolver.fee.isZero() && !integrator.fee.isZero()) {
            assert(resolver.receiver.equal(integrator.protocol), 'resolver fee receiver address and integrator fee protocol address must be same');
        }
        assert(!(resolver.fee.isZero() && integrator.fee.isZero()), 'at least one fee must be set');
        assert(this.integrator.fee.toFraction() < 0.6553, 'max fee is 65.53%');
        assert(this.resolver.fee.toFraction() < 0.6553, 'max fee is 65.53%');
    }
    get protocol() {
        return this.integrator.fee.isZero()
            ? this.resolver.receiver
            : this.integrator.protocol;
    }
    static resolverFee(fee) {
        return new Fees(fee, IntegratorFee.ZERO);
    }
    static integratorFee(fee) {
        return new Fees(ResolverFee.ZERO, fee);
    }
}
Fees.BASE_1E5 = 100000n;
Fees.BASE_1E2 = 100n;
//# sourceMappingURL=fees.js.map