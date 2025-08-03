import { NetworkEnum } from '@1inch/fusion-sdk';
import { TupleToUnion } from './type-utils';
export declare const SupportedChains: readonly [NetworkEnum.ETHEREUM, NetworkEnum.POLYGON, NetworkEnum.BINANCE, NetworkEnum.OPTIMISM, NetworkEnum.ARBITRUM, NetworkEnum.AVALANCHE, NetworkEnum.GNOSIS, NetworkEnum.COINBASE, NetworkEnum.ZKSYNC, NetworkEnum.LINEA, NetworkEnum.SONIC, NetworkEnum.UNICHAIN];
type UnsupportedChain = Exclude<NetworkEnum, TupleToUnion<typeof SupportedChains>>;
export type SupportedChain = Exclude<NetworkEnum, UnsupportedChain>;
export declare const isSupportedChain: (chain: unknown) => chain is SupportedChain;
export {};
