export declare class Bps {
    readonly value: bigint;
    static ZERO: Bps;
    constructor(value: bigint);
    static fromPercent(val: number, base?: bigint): Bps;
    static fromFraction(val: number, base?: bigint): Bps;
    equal(other: Bps): boolean;
    isZero(): boolean;
    toPercent(base?: bigint): number;
    toFraction(base?: bigint): number;
    toString(): string;
}
