import { UINT_32_MAX } from '@1inch/byte-utils';
import assert from 'assert';
export class BaseTimeLock {
    constructor(deployedAt) {
        this.deployedAt = deployedAt;
        assert(deployedAt !== 0n, 'deployedAt must be > 0n');
        assert(deployedAt <= UINT_32_MAX, 'deployedAt can not be > uint32 max value');
    }
    getRescueStart(rescueDelay = BaseTimeLock.DEFAULT_RESCUE_DELAY) {
        return this.deployedAt + rescueDelay;
    }
}
BaseTimeLock.DEFAULT_RESCUE_DELAY = 604800n; // 7 days
//# sourceMappingURL=base-time-lock.js.map