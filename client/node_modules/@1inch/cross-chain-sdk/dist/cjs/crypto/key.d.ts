export declare abstract class Key {
    protected readonly key: Uint8Array;
    constructor(key: Uint8Array);
    toString(): string;
    toJSON(): string;
}
