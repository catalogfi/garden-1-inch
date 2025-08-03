"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.randBigInt = randBigInt;
const ethers_1 = require("ethers");
function randBigInt(max) {
    let bytesCount = 0;
    max = BigInt(max) + 1n;
    let rest = max;
    while (rest) {
        rest = rest >> 8n;
        bytesCount += 1;
    }
    const bytes = (0, ethers_1.randomBytes)(bytesCount);
    const val = bytes.reduce((acc, val, i) => acc + (BigInt(val) << BigInt(i * 8)), 0n);
    return val % max;
}
//# sourceMappingURL=rand-bigint.js.map