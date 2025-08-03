"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BaseTimeLock = void 0;
const tslib_1 = require("tslib");
const byte_utils_1 = require("@1inch/byte-utils");
const assert_1 = tslib_1.__importDefault(require("assert"));
class BaseTimeLock {
    constructor(deployedAt) {
        this.deployedAt = deployedAt;
        (0, assert_1.default)(deployedAt !== 0n, 'deployedAt must be > 0n');
        (0, assert_1.default)(deployedAt <= byte_utils_1.UINT_32_MAX, 'deployedAt can not be > uint32 max value');
    }
    getRescueStart(rescueDelay = BaseTimeLock.DEFAULT_RESCUE_DELAY) {
        return this.deployedAt + rescueDelay;
    }
}
exports.BaseTimeLock = BaseTimeLock;
BaseTimeLock.DEFAULT_RESCUE_DELAY = 604800n; // 7 days
//# sourceMappingURL=base-time-lock.js.map