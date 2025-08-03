import { ethers } from 'ethers';
import { EIP712Domain, LimitOrderV4TypeDataName, LimitOrderV4TypeDataVersion, Order } from './domain';
import { getLimitOrderContract } from '../../constants';
export function getOrderHash(data) {
    return ethers.TypedDataEncoder.hash(data.domain, { Order: data.types.Order }, data.message);
}
export function buildOrderTypedData(chainId, verifyingContract, name, version, order) {
    return {
        primaryType: 'Order',
        types: { EIP712Domain, Order },
        domain: { name, version, chainId, verifyingContract },
        message: order
    };
}
export function getDomainSeparator(name, version, chainId, verifyingContract) {
    return ethers.TypedDataEncoder.hashStruct('EIP712Domain', { EIP712Domain: EIP712Domain }, { name, version, chainId, verifyingContract });
}
export function getLimitOrderV4Domain(chainId) {
    return {
        name: LimitOrderV4TypeDataName,
        version: LimitOrderV4TypeDataVersion,
        chainId,
        verifyingContract: getLimitOrderContract(chainId)
    };
}
//# sourceMappingURL=order-typed-data-builder.js.map