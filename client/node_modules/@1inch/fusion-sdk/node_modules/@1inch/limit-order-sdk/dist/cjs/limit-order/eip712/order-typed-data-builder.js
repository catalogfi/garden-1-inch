"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getOrderHash = getOrderHash;
exports.buildOrderTypedData = buildOrderTypedData;
exports.getDomainSeparator = getDomainSeparator;
exports.getLimitOrderV4Domain = getLimitOrderV4Domain;
const ethers_1 = require("ethers");
const domain_1 = require("./domain");
const constants_1 = require("../../constants");
function getOrderHash(data) {
    return ethers_1.ethers.TypedDataEncoder.hash(data.domain, { Order: data.types.Order }, data.message);
}
function buildOrderTypedData(chainId, verifyingContract, name, version, order) {
    return {
        primaryType: 'Order',
        types: { EIP712Domain: domain_1.EIP712Domain, Order: domain_1.Order },
        domain: { name, version, chainId, verifyingContract },
        message: order
    };
}
function getDomainSeparator(name, version, chainId, verifyingContract) {
    return ethers_1.ethers.TypedDataEncoder.hashStruct('EIP712Domain', { EIP712Domain: domain_1.EIP712Domain }, { name, version, chainId, verifyingContract });
}
function getLimitOrderV4Domain(chainId) {
    return {
        name: domain_1.LimitOrderV4TypeDataName,
        version: domain_1.LimitOrderV4TypeDataVersion,
        chainId,
        verifyingContract: (0, constants_1.getLimitOrderContract)(chainId)
    };
}
//# sourceMappingURL=order-typed-data-builder.js.map