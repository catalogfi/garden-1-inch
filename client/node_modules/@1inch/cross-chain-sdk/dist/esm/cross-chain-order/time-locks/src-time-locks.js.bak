import { UINT_32_MAX } from '@1inch/byte-utils';
import { now } from '@1inch/fusion-sdk';
import assert from 'assert';
import { BaseTimeLock } from './base-time-lock';
export var SrcStage;
(function (SrcStage) {
    SrcStage[SrcStage["FinalityLock"] = 0] = "FinalityLock";
    SrcStage[SrcStage["PrivateWithdrawal"] = 1] = "PrivateWithdrawal";
    SrcStage[SrcStage["PublicWithdrawal"] = 2] = "PublicWithdrawal";
    SrcStage[SrcStage["PrivateCancellation"] = 3] = "PrivateCancellation";
    SrcStage[SrcStage["PublicCancellation"] = 4] = "PublicCancellation";
})(SrcStage || (SrcStage = {}));
/**
 * Class to access source chain related time-locks
 *
 * Intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation | public cancellation |
 * ^deployedAt
 */
export class SrcTimeLocks extends BaseTimeLock {
    constructor(deployedAt, _withdrawal, _publicWithdrawal, _cancellation, _publicCancellation) {
        super(deployedAt);
        this._withdrawal = _withdrawal;
        this._publicWithdrawal = _publicWithdrawal;
        this._cancellation = _cancellation;
        this._publicCancellation = _publicCancellation;
        assert(_withdrawal <= UINT_32_MAX, 'withdrawal can not be > uint32 max value');
        assert(_withdrawal < _publicWithdrawal, 'withdrawal can not be >= publicWithdrawal');
        assert(_publicWithdrawal <= UINT_32_MAX, 'publicWithdrawal can not be > uint32 max value');
        assert(_publicWithdrawal < _cancellation, 'publicWithdrawal can not be >= cancellation');
        assert(_cancellation <= UINT_32_MAX, 'cancellation can not be > uint32 max value');
        assert(_cancellation < _publicCancellation, 'cancellation can not be >= publicCancellation');
        assert(_publicCancellation <= UINT_32_MAX, 'publicCancellation can not be > uint32 max value');
        // technically it is valid for smart-contract, but delay as big as timestamp make no sense in terms of user order
        assert(deployedAt > _withdrawal, `deployedAt timestamp can not be less than withdrawal delay,` +
            `deployedAt: ${deployedAt}, withdrawal: ${_withdrawal}`);
        assert(deployedAt > _publicWithdrawal, `deployedAt timestamp can not be less than publicWithdrawal delay,` +
            `deployedAt: ${deployedAt}, publicWithdrawal: ${_publicWithdrawal}`);
        assert(deployedAt > _cancellation, `deployedAt timestamp can not be less than cancellation delay,` +
            `deployedAt: ${deployedAt}, cancellation: ${_cancellation}`);
        assert(deployedAt > _publicCancellation, `deployedAt timestamp can not be less than publicCancellation delay,` +
            `deployedAt: ${deployedAt}, publicCancellation: ${_publicCancellation}`);
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
    /** Timestamp at which ends `private cancellation` and starts `public cancellation` */
    get publicCancellation() {
        return this.deployedAt + this._publicCancellation;
    }
    static new(params) {
        return new SrcTimeLocks(params.deployedAt, params.withdrawal, params.publicWithdrawal, params.cancellation, params.publicCancellation);
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
        return (time >= this.privateCancellation && time < this.publicCancellation);
    }
    /**
     * Return true, when `time` in `public cancellation` interval
     *
     * @param time default is `now()`
     */
    isPublicCancellation(time = now()) {
        return time >= this.publicCancellation;
    }
    getStage(time = now()) {
        if (this.isFinalityLock(time))
            return SrcStage.FinalityLock;
        if (this.isPrivateWithdrawal(time))
            return SrcStage.PrivateWithdrawal;
        if (this.isPublicWithdrawal(time))
            return SrcStage.PublicWithdrawal;
        if (this.isPrivateCancellation(time))
            return SrcStage.PrivateCancellation;
        if (this.isPublicCancellation(time))
            return SrcStage.PublicCancellation;
        throw new Error('Unreachable');
    }
}
//# sourceMappingURL=src-time-locks.js.map