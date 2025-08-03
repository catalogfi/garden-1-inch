import { Interface } from 'ethers';
import assert from 'assert';
import LimitOrderABI from '../../abi/AggregationRouterV6.abi.json';
import { isHexBytes } from '../../validations';
const lopAbi = new Interface(LimitOrderABI);
export function encodeCancelOrder(hash, makerTraits) {
    assert(isHexBytes(hash), 'Invalid order hash');
    return lopAbi.encodeFunctionData('cancelOrder', [
        makerTraits.asBigInt(),
        hash
    ]);
}
//# sourceMappingURL=cancel-order.encoder.js.map