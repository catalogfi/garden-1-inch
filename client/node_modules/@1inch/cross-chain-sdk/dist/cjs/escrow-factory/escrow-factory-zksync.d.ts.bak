import { Address } from '@1inch/fusion-sdk';
import { EscrowFactory } from './escrow-factory';
export declare class EscrowFactoryZksync extends EscrowFactory {
    private static create2Prefix;
    /**
     * ZkSync proxy bytecode do not depends on implementation address
     *
     * @see proxy example - https://explorer.zksync.io/address/0xd5317Ded4FBb98526AdD35A15d63cFBFB929efc7
     */
    private static minimalProxyBytecodeHash;
    /**
     * Calculate address of escrow contract in ZkSync Era
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
}
