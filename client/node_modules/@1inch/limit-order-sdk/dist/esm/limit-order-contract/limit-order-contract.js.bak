import { Interface, Signature } from 'ethers';
import assert from 'assert';
import LOP_V4_ABI from '../abi/AggregationRouterV6.abi.json';
import { ZX } from '../constants';
const lopContract = new Interface(LOP_V4_ABI);
export class LimitOrderContract {
    static getFillOrderCalldata(order, signature, takerTraits, amount) {
        const { r, yParityAndS: vs } = Signature.from(signature);
        const { args, trait } = takerTraits.encode();
        assert(args === ZX, 'takerTraits contains args data, use LimitOrderContract.getFillOrderArgsCalldata method');
        return lopContract.encodeFunctionData('fillOrder', [
            order,
            r,
            vs,
            amount,
            trait
        ]);
    }
    static getFillContractOrderCalldata(order, signature, takerTraits, amount) {
        const { args, trait } = takerTraits.encode();
        assert(args === ZX, 'takerTraits contains args data, use LimitOrderContract.getFillContractOrderArgsCalldata method');
        return lopContract.encodeFunctionData('fillContractOrder', [
            order,
            signature,
            amount,
            trait,
            args
        ]);
    }
    static getFillOrderArgsCalldata(order, signature, takerTraits, amount) {
        const { r, yParityAndS: vs } = Signature.from(signature);
        const { args, trait } = takerTraits.encode();
        return lopContract.encodeFunctionData('fillOrderArgs', [
            order,
            r,
            vs,
            amount,
            trait,
            args
        ]);
    }
    static getFillContractOrderArgsCalldata(order, signature, takerTraits, amount) {
        const { args, trait } = takerTraits.encode();
        return lopContract.encodeFunctionData('fillContractOrderArgs', [
            order,
            signature,
            amount,
            trait,
            args
        ]);
    }
}
//# sourceMappingURL=limit-order-contract.js.map