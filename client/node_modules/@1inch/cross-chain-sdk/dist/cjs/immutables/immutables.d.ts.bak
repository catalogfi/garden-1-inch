import { Address } from '@1inch/fusion-sdk';
import { DstImmutablesComplement } from './dst-immutables-complement';
import { HashLock } from '../cross-chain-order/hash-lock';
import { TimeLocks } from '../cross-chain-order/time-locks';
/**
 * Contract representation of class
 */
export type ImmutablesData = {
    orderHash: string;
    hashlock: string;
    maker: string;
    taker: string;
    token: string;
    amount: string;
    safetyDeposit: string;
    timelocks: string;
};
/**
 * Contains escrow params for both source and destination chains
 * Determinate addresses of escrow contracts
 */
export declare class Immutables {
    readonly orderHash: string;
    readonly hashLock: HashLock;
    readonly maker: Address;
    /**
     * Address who can withdraw funds, also to this address funds will be transferred in case of public withdrawal
     */
    readonly taker: Address;
    readonly token: Address;
    readonly amount: bigint;
    readonly safetyDeposit: bigint;
    readonly timeLocks: TimeLocks;
    static readonly Web3Type: string;
    private constructor();
    static new(params: {
        orderHash: string;
        hashLock: HashLock;
        maker: Address;
        taker: Address;
        token: Address;
        amount: bigint;
        safetyDeposit: bigint;
        timeLocks: TimeLocks;
    }): Immutables;
    /**
     * Create instance from encoded bytes
     * @param bytes 0x prefixed hex string
     */
    static decode(bytes: string): Immutables;
    toJSON(): ImmutablesData;
    withComplement(dstComplement: DstImmutablesComplement): Immutables;
    withDeployedAt(time: bigint): Immutables;
    withTaker(taker: Address): Immutables;
    withHashLock(hashLock: HashLock): Immutables;
    withAmount(amount: bigint): Immutables;
    /**
     * Return keccak256 hash of instance
     */
    hash(): string;
    build(): ImmutablesData;
    /**
     * Encode instance as bytes
     */
    encode(): string;
}
