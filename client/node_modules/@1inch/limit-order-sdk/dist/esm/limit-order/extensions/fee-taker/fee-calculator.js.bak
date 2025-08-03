import { Fees } from './fees';
import { mulDiv, Rounding } from '../../../utils/mul-div';
export class FeeCalculator {
    constructor(fees, whitelist) {
        this.fees = fees;
        this.whitelist = whitelist;
    }
    getTakingAmount(taker, orderTakingAmount) {
        const fees = this.getFeesForTaker(taker);
        return mulDiv(orderTakingAmount, Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee, Fees.BASE_1E5, Rounding.Ceil);
    }
    getMakingAmount(taker, makingAmount) {
        const fees = this.getFeesForTaker(taker);
        return mulDiv(makingAmount, Fees.BASE_1E5, Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
    }
    getResolverFee(taker, orderTakingAmount) {
        const takingAmount = this.getTakingAmount(taker, orderTakingAmount);
        const fees = this.getFeesForTaker(taker);
        return mulDiv(takingAmount, fees.resolverFee, Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
    }
    getIntegratorFee(taker, orderTakingAmount) {
        const takingAmount = this.getTakingAmount(taker, orderTakingAmount);
        const fees = this.getFeesForTaker(taker);
        const total = mulDiv(takingAmount, fees.integratorFee, Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
        return mulDiv(total, BigInt(this.fees.integrator.share.toFraction(Fees.BASE_1E2)), Fees.BASE_1E2);
    }
    getProtocolShareOfIntegratorFee(taker, orderTakingAmount) {
        const takingAmount = this.getTakingAmount(taker, orderTakingAmount);
        const fees = this.getFeesForTaker(taker);
        const total = mulDiv(takingAmount, fees.integratorFee, Fees.BASE_1E5 + fees.resolverFee + fees.integratorFee);
        return total - this.getIntegratorFee(taker, orderTakingAmount);
    }
    getProtocolFee(taker, orderTakingAmount) {
        const resolverFee = this.getResolverFee(taker, orderTakingAmount);
        const integratorPart = this.getProtocolShareOfIntegratorFee(taker, orderTakingAmount);
        return integratorPart + resolverFee;
    }
    getFeesForTaker(taker) {
        const discountNumerator = this.whitelist.isWhitelisted(taker)
            ? Number(Fees.BASE_1E2) -
                this.fees.resolver.whitelistDiscount.toFraction(Fees.BASE_1E2)
            : 100;
        const resolverFee = BigInt(discountNumerator *
            this.fees.resolver.fee.toFraction(Fees.BASE_1E5)) / Fees.BASE_1E2;
        const resolverFeeBN = BigInt(resolverFee);
        const integratorFeeBN = BigInt(this.fees.integrator.fee.toFraction(Fees.BASE_1E5));
        return {
            resolverFee: resolverFeeBN,
            integratorFee: integratorFeeBN
        };
    }
}
//# sourceMappingURL=fee-calculator.js.map