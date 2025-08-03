import assert from 'assert';
import { Address } from '../../../address';
import { Bps } from '../../../bps';
export class ResolverFee {
    constructor(receiver, fee, whitelistDiscount = Bps.ZERO) {
        this.receiver = receiver;
        this.fee = fee;
        this.whitelistDiscount = whitelistDiscount;
        if (receiver.isZero() && !fee.isZero()) {
            throw new Error('fee must be zero if receiver is zero address');
        }
        if (!receiver.isZero() && fee.isZero()) {
            throw new Error('receiver must be zero address if fee is zero');
        }
        if (fee.isZero() && !whitelistDiscount.isZero()) {
            throw new Error('whitelist discount must be zero if fee is zero');
        }
        assert(this.whitelistDiscount.value % 100n === 0n, `whitelist discount must have percent precision: 1%, 2% and so on`);
    }
}
ResolverFee.ZERO = new ResolverFee(Address.ZERO_ADDRESS, Bps.ZERO);
//# sourceMappingURL=resolver-fee.js.map