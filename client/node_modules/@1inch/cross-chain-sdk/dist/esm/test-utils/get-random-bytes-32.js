import { randomBytes } from 'ethers';
import { uint8ArrayToHex } from '@1inch/byte-utils';
export function getRandomBytes32() {
    return uint8ArrayToHex(randomBytes(32));
}
//# sourceMappingURL=get-random-bytes-32.js.map