import { Key } from './key';
export declare class PublicKey extends Key {
    static fromHexString(hex: string): PublicKey;
    /**
     * Encrypt text with PublicKey using ecies-25519
     * @param text hex encoded string
     *
     * @returns encrypted text encoded as hex
     */
    encrypt(text: string): Promise<string>;
}
