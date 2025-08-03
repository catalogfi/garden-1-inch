"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FeeCalculator = void 0;
const fees_1 = require("./fees");
const mul_div_1 = require("../../../utils/mul-div");
class FeeCalculator {
    constructor(fees, whitelist) {
        this.fees = fees;
        this.whitelist = whitelist;
    }
    getTakingAmount(taker, orderTakingAmount) {
        const fees = this.getFeesForTaker(taker);
        return (0, mul_div_1.mulDiv)(orderTakingAmount, fees_1.Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee, fees_1.Fees.BASE_1E5, mul_div_1.Rounding.Ceil);
    }
    getMakingAmount(taker, makingAmount) {
        const fees = this.getFeesForTaker(taker);
        return (0, mul_div_1.mulDiv)(makingAmount, fees_1.Fees.BASE_1E5, fees_1.Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
    }
    getResolverFee(taker, orderTakingAmount) {
        const takingAmount = this.getTakingAmount(taker, orderTakingAmount);
        const fees = this.getFeesForTaker(taker);
        return (0, mul_div_1.mulDiv)(takingAmount, fees.resolverFee, fees_1.Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
    }
    getIntegratorFee(taker, orderTakingAmount) {
        const takingAmount = this.getTakingAmount(taker, orderTakingAmount);
        const fees = this.getFeesForTaker(taker);
        const total = (0, mul_div_1.mulDiv)(takingAmount, fees.integratorFee, fees_1.Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
        return (0, mul_div_1.mulDiv)(total, BigInt(this.fees.integrator.share.toFraction(fees_1.Fees.BASE_1E2)), fees_1.Fees.BASE_1E2);
    }
    getProtocolShareOfIntegratorFee(taker, orderTakingAmount) {
        const takingAmount = this.getTakingAmount(taker, orderTakingAmount);
        const fees = this.getFeesForTaker(taker);
        const total = (0, mul_div_1.mulDiv)(takingAmount, fees.integratorFee, fees_1.Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
        return total - this.getIntegratorFee(taker, orderTakingAmount);
    }
    getProtocolFee(taker, orderTakingAmount) {
        const resolverFee = this.getResolverFee(taker, orderTakingAmount);
        const integratorPart = this.getProtocolShareOfIntegratorFee(taker, orderTakingAmount);
        return integratorPart + resolverFee;
    }
    getFeesForTaker(taker) {
        const discountNumerator = this.whitelist.isWhitelisted(taker)
            ? Number(fees_1.Fees.BASE_1E2) -
                this.fees.resolver.whitelistDiscount.toFraction(fees_1.Fees.BASE_1E2)
            : 100;
        const resolverFee = BigInt(discountNumerator *
            this.fees.resolver.fee.toFraction(fees_1.Fees.BASE_1E5)) / fees_1.Fees.BASE_1E2;
        const resolverFeeBN = BigInt(resolverFee);
        const integratorFeeBN = BigInt(this.fees.integrator.fee.toFraction(fees_1.Fees.BASE_1E5));
        return {
            resolverFee: resolverFeeBN,
            integratorFee: integratorFeeBN
        };
    }
}
exports.FeeCalculator = FeeCalculator;
//# sourceMappingURL=fee-calculator.js.map