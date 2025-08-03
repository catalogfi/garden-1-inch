"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.LimitOrderWithFee = void 0;
const tslib_1 = require("tslib");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
const fee_taker_extension_1 = require("./extensions/fee-taker/fee-taker.extension");
const limit_order_1 = require("./limit-order");
const maker_traits_1 = require("./maker-traits");
const amounts_1 = require("./amounts");
const address_1 = require("../address");
const rand_bigint_1 = require("../utils/rand-bigint");
class LimitOrderWithFee extends limit_order_1.LimitOrder {
    constructor(orderInfo, makerTraits = new maker_traits_1.MakerTraits(0n), feeExtension) {
        makerTraits.enablePostInteraction();
        super({ ...orderInfo, receiver: feeExtension.address }, makerTraits, feeExtension.build());
        this.feeExtension = feeExtension;
    }
    static withRandomNonce(orderInfo, feeExtension, makerTraits = new maker_traits_1.MakerTraits(0n)) {
        makerTraits.withNonce((0, rand_bigint_1.randBigInt)(byte_utils_1.UINT_40_MAX));
        return new LimitOrderWithFee(orderInfo, makerTraits, feeExtension);
    }
    static fromDataAndExtension(data, extension) {
        const makerTraits = new maker_traits_1.MakerTraits(BigInt(data.makerTraits));
        const feeExt = fee_taker_extension_1.FeeTakerExtension.fromExtension(extension);
        (0, assert_1.default)(feeExt.address.equal(new address_1.Address(data.receiver)), `invalid order: receiver must be FeeTaker extension address`);
        return new LimitOrderWithFee({
            salt: BigInt(data.salt),
            maker: new address_1.Address(data.maker),
            makerAsset: new address_1.Address(data.makerAsset),
            takerAsset: new address_1.Address(data.takerAsset),
            makingAmount: BigInt(data.makingAmount),
            takingAmount: BigInt(data.takingAmount)
        }, makerTraits, feeExt);
    }
    getTakingAmount(taker, makingAmount = this.makingAmount) {
        const takingAmount = (0, amounts_1.calcTakingAmount)(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getTakingAmount(taker, takingAmount);
    }
    getMakingAmount(taker, takingAmount = this.takingAmount) {
        const makingAmount = (0, amounts_1.calcMakingAmount)(takingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getMakingAmount(taker, makingAmount);
    }
    getResolverFee(taker, makingAmount = this.makingAmount) {
        const takingAmount = (0, amounts_1.calcTakingAmount)(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getResolverFee(taker, takingAmount);
    }
    getIntegratorFee(taker, makingAmount = this.makingAmount) {
        const takingAmount = (0, amounts_1.calcTakingAmount)(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getIntegratorFee(taker, takingAmount);
    }
    getProtocolFee(taker, makingAmount = this.makingAmount) {
        const takingAmount = (0, amounts_1.calcTakingAmount)(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getProtocolFee(taker, takingAmount);
    }
}
exports.LimitOrderWithFee = LimitOrderWithFee;
//# sourceMappingURL=limit-order-with-fee.js.map