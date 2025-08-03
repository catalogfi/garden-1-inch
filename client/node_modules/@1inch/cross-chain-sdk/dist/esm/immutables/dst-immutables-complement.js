export class DstImmutablesComplement {
    constructor(maker, amount, token, safetyDeposit) {
        this.maker = maker;
        this.amount = amount;
        this.token = token;
        this.safetyDeposit = safetyDeposit;
    }
    static new(params) {
        return new DstImmutablesComplement(params.maker, params.amount, params.token, params.safetyDeposit);
    }
    toJSON() {
        return {
            maker: this.maker.toString(),
            amount: this.amount.toString(),
            token: this.token.toString(),
            safetyDeposit: this.safetyDeposit.toString()
        };
    }
}
//# sourceMappingURL=dst-immutables-complement.js.map