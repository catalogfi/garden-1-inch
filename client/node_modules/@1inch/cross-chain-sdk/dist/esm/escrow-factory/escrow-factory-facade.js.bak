import { NetworkEnum } from '@1inch/fusion-sdk';
import { EscrowFactory } from './escrow-factory';
import { EscrowFactoryZksync } from './escrow-factory-zksync';
export class EscrowFactoryFacade {
    constructor(chainId, factoryAddress) {
        this.factory = EscrowFactoryFacade.getFactory(chainId, factoryAddress);
    }
    get address() {
        return this.factory.address;
    }
    static getFactory(chainId, factoryAddress) {
        switch (chainId) {
            case NetworkEnum.ZKSYNC:
                return new EscrowFactoryZksync(factoryAddress);
            default:
                return new EscrowFactory(factoryAddress);
        }
    }
    getEscrowAddress(
    /**
     * @see Immutables.hash
     */
    immutablesHash, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress) {
        return this.factory.getEscrowAddress(immutablesHash, implementationAddress);
    }
    getSrcEscrowAddress(
    /**
     * From `SrcEscrowCreated` event (with correct timeLock.deployedAt)
     */
    srcImmutables, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress) {
        return this.factory.getSrcEscrowAddress(srcImmutables, implementationAddress);
    }
    getDstEscrowAddress(
    /**
     * From `SrcEscrowCreated` event
     */
    srcImmutables, 
    /**
     * From `SrcEscrowCreated` event
     */
    complement, 
    /**
     * Block time when event `DstEscrowCreated` produced
     */
    blockTime, 
    /**
     * Taker from `DstEscrowCreated` event
     */
    taker, 
    /**
     * Address of escrow implementation at the same chain as `this.address`
     */
    implementationAddress) {
        return this.factory.getDstEscrowAddress(srcImmutables, complement, blockTime, taker, implementationAddress);
    }
    getMultipleFillInteraction(proof, idx, secretHash) {
        return this.factory.getMultipleFillInteraction(proof, idx, secretHash);
    }
}
//# sourceMappingURL=escrow-factory-facade.js.map