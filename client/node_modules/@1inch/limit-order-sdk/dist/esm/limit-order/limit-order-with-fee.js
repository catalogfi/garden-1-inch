import { UINT_40_MAX } from '@1inch/byte-utils';
import assert from 'assert';
import { FeeTakerExtension } from './extensions/fee-taker/fee-taker.extension';
import { LimitOrder } from './limit-order';
import { MakerTraits } from './maker-traits';
import { calcMakingAmount, calcTakingAmount } from './amounts';
import { Address } from '../address';
import { randBigInt } from '../utils/rand-bigint';
export class LimitOrderWithFee extends LimitOrder {
    constructor(orderInfo, makerTraits = new MakerTraits(0n), feeExtension) {
        makerTraits.enablePostInteraction();
        super({ ...orderInfo, receiver: feeExtension.address }, makerTraits, feeExtension.build());
        this.feeExtension = feeExtension;
    }
    static withRandomNonce(orderInfo, feeExtension, makerTraits = new MakerTraits(0n)) {
        makerTraits.withNonce(randBigInt(UINT_40_MAX));
        return new LimitOrderWithFee(orderInfo, makerTraits, feeExtension);
    }
    static fromDataAndExtension(data, extension) {
        const makerTraits = new MakerTraits(BigInt(data.makerTraits));
        const feeExt = FeeTakerExtension.fromExtension(extension);
        assert(feeExt.address.equal(new Address(data.receiver)), `invalid order: receiver must be FeeTaker extension address`);
        return new LimitOrderWithFee({
            salt: BigInt(data.salt),
            maker: new Address(data.maker),
            makerAsset: new Address(data.makerAsset),
            takerAsset: new Address(data.takerAsset),
            makingAmount: BigInt(data.makingAmount),
            takingAmount: BigInt(data.takingAmount)
        }, makerTraits, feeExt);
    }
    getTakingAmount(taker, makingAmount = this.makingAmount) {
        const takingAmount = calcTakingAmount(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getTakingAmount(taker, takingAmount);
    }
    getMakingAmount(taker, takingAmount = this.takingAmount) {
        const makingAmount = calcMakingAmount(takingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getMakingAmount(taker, makingAmount);
    }
    getResolverFee(taker, makingAmount = this.makingAmount) {
        const takingAmount = calcTakingAmount(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getResolverFee(taker, takingAmount);
    }
    getIntegratorFee(taker, makingAmount = this.makingAmount) {
        const takingAmount = calcTakingAmount(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getIntegratorFee(taker, takingAmount);
    }
    getProtocolFee(taker, makingAmount = this.makingAmount) {
        const takingAmount = calcTakingAmount(makingAmount, this.makingAmount, this.takingAmount);
        return this.feeExtension.getProtocolFee(taker, takingAmount);
    }
}
//# sourceMappingURL=limit-order-with-fee.js.map