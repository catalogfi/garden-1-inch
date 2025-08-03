"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.LimitOrderContract = void 0;
const tslib_1 = require("tslib");
const ethers_1 = require("ethers");
const assert_1 = tslib_1.__importDefault(require("assert"));
const AggregationRouterV6_abi_json_1 = tslib_1.__importDefault(require("../abi/AggregationRouterV6.abi.json"));
const constants_1 = require("../constants");
const lopContract = new ethers_1.Interface(AggregationRouterV6_abi_json_1.default);
class LimitOrderContract {
    static getFillOrderCalldata(order, signature, takerTraits, amount) {
        const { r, yParityAndS: vs } = ethers_1.Signature.from(signature);
        const { args, trait } = takerTraits.encode();
        (0, assert_1.default)(args === constants_1.ZX, 'takerTraits contains args data, use LimitOrderContract.getFillOrderArgsCalldata method');
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
        (0, assert_1.default)(args === constants_1.ZX, 'takerTraits contains args data, use LimitOrderContract.getFillContractOrderArgsCalldata method');
        return lopContract.encodeFunctionData('fillContractOrder', [
            order,
            signature,
            amount,
            trait,
            args
        ]);
    }
    static getFillOrderArgsCalldata(order, signature, takerTraits, amount) {
        const { r, yParityAndS: vs } = ethers_1.Signature.from(signature);
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
exports.LimitOrderContract = LimitOrderContract;
//# sourceMappingURL=limit-order-contract.js.map