import { Address } from '../address';
export declare class Interaction {
    readonly target: Address;
    readonly data: string;
    constructor(target: Address, data: string);
    static decode(bytes: string): Interaction;
    encode(): string;
}
