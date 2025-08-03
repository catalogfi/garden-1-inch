"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.LimitOrderV4TypeDataVersion = exports.LimitOrderV4TypeDataName = exports.Order = exports.EIP712Domain = void 0;
exports.EIP712Domain = [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' }
];
exports.Order = [
    { name: 'salt', type: 'uint256' },
    { name: 'maker', type: 'address' },
    { name: 'receiver', type: 'address' },
    { name: 'makerAsset', type: 'address' },
    { name: 'takerAsset', type: 'address' },
    { name: 'makingAmount', type: 'uint256' },
    { name: 'takingAmount', type: 'uint256' },
    { name: 'makerTraits', type: 'uint256' }
];
exports.LimitOrderV4TypeDataName = '1inch Aggregation Router';
exports.LimitOrderV4TypeDataVersion = '6';
//# sourceMappingURL=domain.js.map