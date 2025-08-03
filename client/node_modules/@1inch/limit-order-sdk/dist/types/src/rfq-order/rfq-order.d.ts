import { LimitOrder, OrderInfoData } from '../limit-order';
import { Address } from '../address';
export declare class RfqOrder extends LimitOrder {
    constructor(orderInfo: Omit<OrderInfoData, 'salt' | 'receiver'>, options: {
        allowedSender?: Address;
        expiration: bigint;
        nonce: bigint;
        usePermit2?: boolean;
    });
}
