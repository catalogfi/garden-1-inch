"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RfqOrder = void 0;
const limit_order_1 = require("../limit-order");
class RfqOrder extends limit_order_1.LimitOrder {
    constructor(orderInfo, options) {
        const { allowedSender, nonce, expiration, usePermit2 } = options;
        const makerTraits = new limit_order_1.MakerTraits(0n)
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
exports.RfqOrder = RfqOrder;
//# sourceMappingURL=rfq-order.js.map