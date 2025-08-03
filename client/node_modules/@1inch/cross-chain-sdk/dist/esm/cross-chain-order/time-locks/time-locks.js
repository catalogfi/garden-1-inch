import { BitMask, BN, UINT_32_MAX } from '@1inch/byte-utils';
import assert from 'assert';
import { SrcTimeLocks } from './src-time-locks';
import { DstTimeLocks } from './dst-time-locks';
/**
 * Contains the duration of each stage of swap for source and destination chain
 *
 * Source chain intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation | public cancellation |
 * ^deployedAt
 *
 * Destination chain intervals layout
 * | finality lock | private withdrawal | public withdrawal | private cancellation |
 * ^deployedAt
 *
 * @see SrcTimeLocks
 * @see DstTimeLocks
 */
export class TimeLocks {
    constructor(_srcWithdrawal, _srcPublicWithdrawal, _srcCancellation, _srcPublicCancellation, _dstWithdrawal, _dstPublicWithdrawal, _dstCancellation, _deployedAt) {
        this._srcWithdrawal = _srcWithdrawal;
        this._srcPublicWithdrawal = _srcPublicWithdrawal;
        this._srcCancellation = _srcCancellation;
        this._srcPublicCancellation = _srcPublicCancellation;
        this._dstWithdrawal = _dstWithdrawal;
        this._dstPublicWithdrawal = _dstPublicWithdrawal;
        this._dstCancellation = _dstCancellation;
        this._deployedAt = _deployedAt;
        assert(_deployedAt <= UINT_32_MAX, 'deployedAt can not be > uint32 max value');
        assert(_srcWithdrawal <= UINT_32_MAX, 'srcWithdrawal can not be > uint32 max value');
        assert(_srcWithdrawal < _srcPublicWithdrawal, 'srcWithdrawal can not be >= srcPublicWithdrawal');
        assert(_srcPublicWithdrawal <= UINT_32_MAX, 'srcPublicWithdrawal can not be > uint32 max value');
        assert(_srcPublicWithdrawal < _srcCancellation, 'srcPublicWithdrawal can not be >= srcCancellation');
        assert(_srcCancellation <= UINT_32_MAX, 'srcCancellation can not be > uint32 max value');
        assert(_srcCancellation < _srcPublicCancellation, 'srcCancellation can not be >= srcPublicCancellation');
        assert(_srcPublicCancellation <= UINT_32_MAX, 'srcPublicCancellation can not be > uint32 max value');
        assert(_dstWithdrawal <= UINT_32_MAX, 'dstWithdrawal can not be > uint32 max value');
        assert(_dstWithdrawal < _dstPublicWithdrawal, 'dstWithdrawal can not be >= dstPublicWithdrawal');
        assert(_dstPublicWithdrawal <= UINT_32_MAX, 'dstPublicWithdrawal can not be > uint32 max value');
        assert(_dstPublicWithdrawal < _dstCancellation, 'dstPublicWithdrawal can not be >= dstCancellation');
        assert(_dstCancellation <= UINT_32_MAX, 'dstCancellation can not be > uint32 max value');
    }
    get deployedAt() {
        return this._deployedAt;
    }
    static new(params) {
        return new TimeLocks(params.srcWithdrawal, params.srcPublicWithdrawal, params.srcCancellation, params.srcPublicCancellation, params.dstWithdrawal, params.dstPublicWithdrawal, params.dstCancellation, 0n);
    }
    static fromDurations(durations) {
        return new TimeLocks(durations.srcFinalityLock, durations.srcFinalityLock + durations.srcPrivateWithdrawal, durations.srcFinalityLock +
            durations.srcPrivateWithdrawal +
            durations.srcPublicWithdrawal, durations.srcFinalityLock +
            durations.srcPrivateWithdrawal +
            durations.srcPublicWithdrawal +
            durations.srcPrivateCancellation, durations.dstFinalityLock, durations.dstFinalityLock + durations.dstPrivateWithdrawal, durations.dstFinalityLock +
            durations.dstPrivateWithdrawal +
            durations.dstPublicWithdrawal, 0n);
    }
    static fromBigInt(val) {
        const valBN = new BN(val);
        const params = Array.from({ length: 8 }).map((_, i) => {
            return valBN.getMask(new BitMask(BigInt(i) * 32n, BigInt(i + 1) * 32n)).value;
        }); // ts can not infer that result is 8 elements
        return new TimeLocks(...params);
    }
    toJSON() {
        return '0x' + this.build().toString(16);
    }
    build() {
        return [
            this.deployedAt,
            this._dstCancellation,
            this._dstPublicWithdrawal,
            this._dstWithdrawal,
            this._srcPublicCancellation,
            this._srcCancellation,
            this._srcPublicWithdrawal,
            this._srcWithdrawal
        ].reduce((acc, el) => (acc << 32n) | el);
    }
    setDeployedAt(time) {
        this._deployedAt = time;
        return this;
    }
    toSrcTimeLocks(deployedAt = this.deployedAt) {
        return SrcTimeLocks.new({
            deployedAt,
            withdrawal: this._srcWithdrawal,
            publicWithdrawal: this._srcPublicWithdrawal,
            cancellation: this._srcCancellation,
            publicCancellation: this._srcPublicCancellation
        });
    }
    toDstTimeLocks(deployedAt = this.deployedAt) {
        return DstTimeLocks.new({
            deployedAt,
            withdrawal: this._dstWithdrawal,
            publicWithdrawal: this._dstPublicWithdrawal,
            cancellation: this._dstCancellation
        });
    }
}
TimeLocks.DEFAULT_RESCUE_DELAY = 604800n; // 7 days
TimeLocks.Web3Type = 'uint256';
//# sourceMappingURL=time-locks.js.map