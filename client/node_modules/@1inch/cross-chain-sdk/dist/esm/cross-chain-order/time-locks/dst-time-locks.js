import { UINT_32_MAX } from '@1inch/byte-utils';
import { now } from '@1inch/fusion-sdk';
import assert from 'assert';
import { BaseTimeLock } from './base-time-lock';
export var DstStage;
(function (DstStage) {
    DstStage[DstStage["FinalityLock"] = 0] = "FinalityLock";
    DstStage[DstStage["PrivateWithdrawal"] = 1] = "PrivateWithdrawal";
    DstStage[DstStage["PublicWithdrawal"] = 2] = "PublicWithdrawal";
    DstStage[DstStage["PrivateCancellation"] = 3] = "PrivateCancellation";
})(DstStage || (DstStage = {}));
/**
 * Class to access destination chain related time-locks
 *
 * Intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation |
 * ^deployedAt
 */
export class DstTimeLocks extends BaseTimeLock {
    constructor(deployedAt, _withdrawal, _publicWithdrawal, _cancellation) {
        super(deployedAt);
        this._withdrawal = _withdrawal;
        this._publicWithdrawal = _publicWithdrawal;
        this._cancellation = _cancellation;
        assert(_withdrawal <= UINT_32_MAX, 'withdrawal can not be > uint32 max value');
        assert(_withdrawal < _publicWithdrawal, 'withdrawal can not be >= publicWithdrawal');
        assert(_publicWithdrawal <= UINT_32_MAX, 'publicWithdrawal can not be > uint32 max value');
        assert(_publicWithdrawal < _cancellation, 'publicWithdrawal can not be >= cancellation');
        assert(_cancellation <= UINT_32_MAX, 'cancellation can not be > uint32 max value');
        // technically it is valid for smart-contract, but delay as big as timestamp make no sense in terms of user order
        assert(deployedAt > _withdrawal, `deployedAt timestamp can not be less than withdrawal delay,` +
            `deployedAt: ${deployedAt}, withdrawal: ${_withdrawal}`);
        assert(deployedAt > _publicWithdrawal, `deployedAt timestamp can not be less than publicWithdrawal delay,` +
            `deployedAt: ${deployedAt}, publicWithdrawal: ${_publicWithdrawal}`);
        assert(deployedAt > _cancellation, `deployedAt timestamp can not be less than cancellation delay,` +
            `deployedAt: ${deployedAt}, cancellation: ${_cancellation}`);
    }
    /** Timestamp at which ends `finality lock` and starts `private withdrawal` */
    get privateWithdrawal() {
        return this.deployedAt + this._withdrawal;
    }
    /** Timestamp at which ends `private withdrawal` and starts `public withdrawal` */
    get publicWithdrawal() {
        return this.deployedAt + this._publicWithdrawal;
    }
    /** Timestamp at which ends `public withdrawal` and starts `private cancellation` */
    get privateCancellation() {
        return this.deployedAt + this._cancellation;
    }
    static new(params) {
        return new DstTimeLocks(params.deployedAt, params.withdrawal, params.publicWithdrawal, params.cancellation);
    }
    /**
     * Return true, when `time` in `finality lock` interval
     *
     * @param time default is `now()`
     */
    isFinalityLock(time = now()) {
        return time < this.privateWithdrawal;
    }
    /**
     * Return true, when `time` in `private withdrawal` interval
     *
     * @param time default is `now()`
     */
    isPrivateWithdrawal(time = now()) {
        return time >= this.privateWithdrawal && time < this.publicWithdrawal;
    }
    /**
     * Return true, when `time` in `public withdrawal` interval
     *
     * @param time default is `now()`
     */
    isPublicWithdrawal(time = now()) {
        return time >= this.publicWithdrawal && time < this.privateCancellation;
    }
    /**
     * Return true, when `time` in `private cancellation` interval
     *
     * @param time default is `now()`
     */
    isPrivateCancellation(time = now()) {
        return time >= this.privateCancellation;
    }
    getStage(time = now()) {
        if (this.isFinalityLock(time))
            return DstStage.FinalityLock;
        if (this.isPrivateWithdrawal(time))
            return DstStage.PrivateWithdrawal;
        if (this.isPublicWithdrawal(time))
            return DstStage.PublicWithdrawal;
        if (this.isPrivateCancellation(time))
            return DstStage.PrivateCancellation;
        throw new Error('Unreachable');
    }
}
//# sourceMappingURL=dst-time-locks.js.map