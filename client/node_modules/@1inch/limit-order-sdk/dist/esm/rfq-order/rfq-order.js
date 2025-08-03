import { LimitOrder, MakerTraits } from '../limit-order';
export class RfqOrder extends LimitOrder {
    constructor(orderInfo, options) {
        const { allowedSender, nonce, expiration, usePermit2 } = options;
        const makerTraits = new MakerTraits(0n)
            .disableMultipleFills()
            .allowPartialFills()
            .withExpiration(expiration)
            .withNonce(nonce);
        if (allowedSender) {
            makerTraits.withAllowedSender(allowedSender);
        }
        if (usePermit2) {
            makerTraits.enablePermit2();
        }
        super(orderInfo, makerTraits);
    }
}
//# sourceMappingURL=rfq-order.js.map