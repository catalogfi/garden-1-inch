import { BytesBuilder } from '@1inch/byte-utils';
import { Address } from 'address';
import { Whitelist } from './types';
export declare class WhitelistHalfAddress implements Whitelist {
    private readonly addresses;
    constructor(addresses: string[]);
    get length(): number;
    static new(addresses: Address[]): WhitelistHalfAddress;
    isWhitelisted(address: Address): boolean;
    encodeTo(builder: BytesBuilder): BytesBuilder;
}
