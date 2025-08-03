"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.InnerOrder = void 0;
const fusion_sdk_1 = require("@1inch/fusion-sdk");
/**
 * Inner order class, not intended for public usage
 */
class InnerOrder extends fusion_sdk_1.FusionOrder {
    constructor(extension, orderInfo, extra) {
        super(extension.address, orderInfo, extension.auctionDetails, extension.postInteractionData, extra, extension);
        this.escrowExtension = extension;
    }
}
exports.InnerOrder = InnerOrder;
//# sourceMappingURL=inner-order.js.map