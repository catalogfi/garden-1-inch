import { EIP712TypedData } from './eip712';
import { LimitOrderV4Struct, OrderInfoData } from './types';
import { MakerTraits } from './maker-traits';
import { Extension } from './extensions/extension';
import { Address } from '../address';
export declare class LimitOrder {
    readonly extension: Extension;
    private static readonly Web3Type;
    readonly maker: Address;
    readonly receiver: Address;
    readonly makerAsset: Address;
    readonly takerAsset: Address;
    readonly makingAmount: bigint;
    readonly takingAmount: bigint;
    readonly makerTraits: MakerTraits;
    private _salt;
    constructor(orderInfo: OrderInfoData, makerTraits?: MakerTraits, extension?: Extension);
    get salt(): bigint;
    static buildSalt(extension: Extension, baseSalt?: bigint): bigint;
    static verifySalt(salt: bigint, extension: Extension): bigint;
    static fromCalldata(bytes: string): LimitOrder;
    static fromDataAndExtension(data: LimitOrderV4Struct, extension: Extension): LimitOrder;
    setSource(source: string): this;
    toCalldata(): string;
    build(): LimitOrderV4Struct;
    getTypedData(chainId: number): EIP712TypedData;
    getOrderHash(chainId: number): string;
    isPrivate(): boolean;
}
