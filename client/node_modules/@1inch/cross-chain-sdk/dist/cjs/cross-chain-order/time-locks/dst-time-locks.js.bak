"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DstTimeLocks = exports.DstStage = void 0;
const tslib_1 = require("tslib");
const byte_utils_1 = require("@1inch/byte-utils");
const fusion_sdk_1 = require("@1inch/fusion-sdk");
const assert_1 = tslib_1.__importDefault(require("assert"));
const base_time_lock_1 = require("./base-time-lock");
var DstStage;
(function (DstStage) {
    DstStage[DstStage["FinalityLock"] = 0] = "FinalityLock";
    DstStage[DstStage["PrivateWithdrawal"] = 1] = "PrivateWithdrawal";
    DstStage[DstStage["PublicWithdrawal"] = 2] = "PublicWithdrawal";
    DstStage[DstStage["PrivateCancellation"] = 3] = "PrivateCancellation";
})(DstStage || (exports.DstStage = DstStage = {}));
/**
 * Class to access destination chain related time-locks
 *
 * Intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation |
 * ^deployedAt
 */
class DstTimeLocks extends base_time_lock_1.BaseTimeLock {
    constructor(deployedAt, _withdrawal, _publicWithdrawal, _cancellation) {
        super(deployedAt);
        this._withdrawal = _withdrawal;
        this._publicWithdrawal = _publicWithdrawal;
        this._cancellation = _cancellation;
        (0, assert_1.default)(_withdrawal <= byte_utils_1.UINT_32_MAX, 'withdrawal can not be > uint32 max value');
        (0, assert_1.default)(_withdrawal < _publicWithdrawal, 'withdrawal can not be >= publicWithdrawal');
        (0, assert_1.default)(_publicWithdrawal <= byte_utils_1.UINT_32_MAX, 'publicWithdrawal can not be > uint32 max value');
        (0, assert_1.default)(_publicWithdrawal < _cancellation, 'publicWithdrawal can not be >= cancellation');
        (0, assert_1.default)(_cancellation <= byte_utils_1.UINT_32_MAX, 'cancellation can not be > uint32 max value');
        // technically it is valid for smart-contract, but delay as big as timestamp make no sense in terms of user order
        (0, assert_1.default)(deployedAt > _withdrawal, `deployedAt timestamp can not be less than withdrawal delay,` +
            `deployedAt: ${deployedAt}, withdrawal: ${_withdrawal}`);
        (0, assert_1.default)(deployedAt > _publicWithdrawal, `deployedAt timestamp can not be less than publicWithdrawal delay,` +
            `deployedAt: ${deployedAt}, publicWithdrawal: ${_publicWithdrawal}`);
        (0, assert_1.default)(deployedAt > _cancellation, `deployedAt timestamp can not be less than cancellation delay,` +
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
    isFinalityLock(time = (0, fusion_sdk_1.now)()) {
        return time < this.privateWithdrawal;
    }
    /**
     * Return true, when `time` in `private withdrawal` interval
     *
     * @param time default is `now()`
     */
    isPrivateWithdrawal(time = (0, fusion_sdk_1.now)()) {
        return time >= this.privateWithdrawal && time < this.publicWithdrawal;
    }
    /**
     * Return true, when `time` in `public withdrawal` interval
     *
     * @param time default is `now()`
     */
    isPublicWithdrawal(time = (0, fusion_sdk_1.now)()) {
        return time >= this.publicWithdrawal && time < this.privateCancellation;
    }
    /**
     * Return true, when `time` in `private cancellation` interval
     *
     * @param time default is `now()`
     */
    isPrivateCancellation(time = (0, fusion_sdk_1.now)()) {
        return time >= this.privateCancellation;
    }
    getStage(time = (0, fusion_sdk_1.now)()) {
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
exports.DstTimeLocks = DstTimeLocks;
//# sourceMappingURL=dst-time-locks.js.map