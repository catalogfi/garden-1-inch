import { Interface, Signature, TransactionRequest, Contract as EthersContract, JsonRpcProvider, BaseContract } from 'ethers'
import Sdk from '@1inch/cross-chain-sdk'
import Contract from '../dist/contracts/Resolver.sol/Resolver.json'

// Type the contract methods properly
interface ResolverContractInterface extends BaseContract {
    deploySrc(
        immutables: any,
        order: any,
        r: string,
        vs: string,
        amount: bigint,
        takerTraits: bigint,
        args: string,
        overrides?: any
    ): Promise<any>

    deployDst(
        dstImmutables: any,
        srcCancellationTimestamp: bigint,
        overrides?: any
    ): Promise<any>

    withdraw(
        escrow: string,
        secret: string,
        immutables: any,
        overrides?: any
    ): Promise<any>

    cancel(
        escrow: string,
        immutables: any,
        overrides?: any
    ): Promise<any>

    owner(): Promise<string>
}

export class Resolver {
    private readonly iface = new Interface(Contract.abi)

    constructor(
        public readonly srcAddress: string,
        public readonly dstAddress: string
    ) { }

    public getSrcContract(provider: JsonRpcProvider): ResolverContractInterface {
        return new EthersContract(this.srcAddress, Contract.abi, provider) as unknown as ResolverContractInterface
    }

    public getDstContract(provider: JsonRpcProvider): ResolverContractInterface {
        return new EthersContract(this.dstAddress, Contract.abi, provider) as unknown as ResolverContractInterface
    }

    public deploySrc(
        chainId: number,
        order: Sdk.CrossChainOrder,
        signature: string,
        takerTraits: Sdk.TakerTraits,
        amount: bigint,
        hashLock = order.escrowExtension.hashLockInfo
    ): TransactionRequest {
        const { r, yParityAndS: vs } = Signature.from(signature)
        const { args, trait } = takerTraits.encode()
        const immutables = order.toSrcImmutables(chainId, new Sdk.Address(this.srcAddress), amount, hashLock)
        console.log("Immutables for src deploy", immutables.build());
        console.log("Order for src deploy", order.build());
        console.log("Signature for src deploy", { r, vs, amount, trait, args });

        return {
            to: this.srcAddress,
            data: this.iface.encodeFunctionData('deploySrc', [
                immutables.build(),
                order.build(),
                r,
                vs,
                amount,
                trait,
                args
            ]),
            value: order.escrowExtension.srcSafetyDeposit
        }
    }

    public deployDst(
        /**
         * Immutables from SrcEscrowCreated event with complement applied
         */
        immutables: Sdk.Immutables
    ): TransactionRequest {
        console.log("Immutables for dst deploy", JSON.stringify(immutables, null, 2));
        return {
            to: this.dstAddress,
            data: this.iface.encodeFunctionData('deployDst', [
                immutables.build(),
                immutables.timeLocks.toSrcTimeLocks().privateCancellation
            ]),
            value: immutables.safetyDeposit
        }
    }

    public withdraw(
        side: 'src' | 'dst',
        escrow: Sdk.Address,
        secret: string,
        immutables: Sdk.Immutables
    ): TransactionRequest {
        return {
            to: side === 'src' ? this.srcAddress : this.dstAddress,
            data: this.iface.encodeFunctionData('withdraw', [escrow.toString(), secret, immutables.build()])
        }
    }

    public cancel(side: 'src' | 'dst', escrow: Sdk.Address, immutables: Sdk.Immutables): TransactionRequest {
        return {
            to: side === 'src' ? this.srcAddress : this.dstAddress,
            data: this.iface.encodeFunctionData('cancel', [escrow.toString(), immutables.build()])
        }
    }
}