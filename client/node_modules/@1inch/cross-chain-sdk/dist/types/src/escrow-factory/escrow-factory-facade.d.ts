import { Address, Interaction, NetworkEnum } from '@1inch/fusion-sdk';
import { EscrowFactory } from './escrow-factory';
import { DstImmutablesComplement, Immutables } from '../immutables';
import { MerkleLeaf } from '../cross-chain-order/hash-lock/hash-lock';
export declare class EscrowFactoryFacade implements EscrowFactory {
    private factory;
    constructor(chainId: NetworkEnum, factoryAddress: Address);
    get address(): Address;
    static getFactory(chainId: NetworkEnum, factoryAddress: Address): EscrowFactory;
    getEscrowAddress(
    /**
     * @see Immutables.hash
     */
    immutablesHash: string, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress: Address): Address;
    getSrcEscrowAddress(
    /**
     * From `SrcEscrowCreated` event (with correct timeLock.deployedAt)
     */
    srcImmutables: Immutables, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress: Address): Address;
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
