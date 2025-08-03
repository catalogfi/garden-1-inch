"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Fees = void 0;
const tslib_1 = require("tslib");
const assert_1 = tslib_1.__importDefault(require("assert"));
const resolver_fee_1 = require("./resolver-fee");
const integrator_fee_1 = require("./integrator-fee");
class Fees {
    constructor(resolver, integrator) {
        this.resolver = resolver;
        this.integrator = integrator;
        if (!resolver.fee.isZero() && !integrator.fee.isZero()) {
            (0, assert_1.default)(resolver.receiver.equal(integrator.protocol), 'resolver fee receiver address and integrator fee protocol address must be same');
        }
        (0, assert_1.default)(!(resolver.fee.isZero() && integrator.fee.isZero()), 'at least one fee must be set');
        (0, assert_1.default)(this.integrator.fee.toFraction() < 0.6553, 'max fee is 65.53%');
        (0, assert_1.default)(this.resolver.fee.toFraction() < 0.6553, 'max fee is 65.53%');
    }
    get protocol() {
        return this.integrator.fee.isZero()
            ? this.resolver.receiver
            : this.integrator.protocol;
    }
    static resolverFee(fee) {
        return new Fees(fee, integrator_fee_1.IntegratorFee.ZERO);
    }
    static integratorFee(fee) {
        return new Fees(resolver_fee_1.ResolverFee.ZERO, fee);
    }
}
exports.Fees = Fees;
Fees.BASE_1E5 = 100000n;
Fees.BASE_1E2 = 100n;
//# sourceMappingURL=fees.js.map