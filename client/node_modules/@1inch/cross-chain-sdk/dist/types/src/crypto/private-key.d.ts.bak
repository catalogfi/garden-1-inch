import { Key } from './key';
export declare class PrivateKey extends Key {
    static fromHexString(hex: string): PrivateKey;
    /**
     * Decrypt cipher with PrivateKey using ecies-25519
     * @param cipher hex string
     *
     * @returns hex encoded string
     */
    decrypt(cipher: string): Promise<string>;
}
