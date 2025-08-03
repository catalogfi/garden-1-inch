import { Address, Interaction } from '@1inch/fusion-sdk';
import { Immutables, DstImmutablesComplement } from '../immutables';
import { MerkleLeaf } from '../cross-chain-order/hash-lock/hash-lock';
export declare class EscrowFactory {
    readonly address: Address;
    constructor(address: Address);
    /**
     * See https://github.com/1inch/cross-chain-swap/blob/03d99b9604d8f7a5a396720fbe1059f7d94db762/contracts/libraries/ProxyHashLib.sol#L14
     */
    protected static calcProxyBytecodeHash(impl: Address): string;
    /**
     * Calculate address of escrow contract
     *
     * @return escrow address at same the chain as `this.address`
     */
    getEscrowAddress(
    /**
     * @see Immutables.hash
     */
    immutablesHash: string, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress: Address): Address;
    /**
     * Calculates source escrow address for given params
     *
     * Make sure you call it on source chain escrow factory
     */
    getSrcEscrowAddress(
    /**
     * From `SrcEscrowCreated` event (with correct timeLock.deployedAt)
     */
    srcImmutables: Immutables, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress: Address): Address;
    /**
     * Calculates destination escrow address for given params
     *
     * Make sure you call it on destination chain escrow factory
     */
    getDstEscrowAddress(
    /**
     * From `SrcEscrowCreated` event
     */
    srcImmutables: Immutables, 
    /**
     * From `SrcEscrowCreated` event
     */
    complement: DstImmutablesComplement, 
    /**
     * Block time when event `DstEscrowCreated` produced
     */
    blockTime: bigint, 
    /**
     * Taker from `DstEscrowCreated` event
     */
    taker: Address, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress: Address): Address;
    getMultipleFillInteraction(proof: MerkleLeaf[], idx: number, secretHash: string): Interaction;
}
