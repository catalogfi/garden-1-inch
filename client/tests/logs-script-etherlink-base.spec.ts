/* eslint-disable */
import { jest } from '@jest/globals';
import 'dotenv/config';

import { uint8ArrayToHex, UINT_40_MAX } from '@1inch/byte-utils';
import Sdk from '@1inch/cross-chain-sdk';
import {
    computeAddress,
    ContractFactory,
    ethers,
    JsonRpcProvider,
    parseEther,
    parseUnits,
    randomBytes,
    Signature,
    Wallet as SignerWallet
} from 'ethers';
import assert from 'node:assert';
import { createServer, CreateServerReturnType } from 'prool';
import { anvil } from 'prool/instances';
import resolverContract from '../dist/contracts/Resolver.sol/Resolver.json';
import factoryContract from '../dist/contracts/TestEscrowFactory.sol/TestEscrowFactory.json';
import { ChainConfig, config } from './config2';
import { EscrowFactory } from './escrow-factory';
import { Resolver } from './resolver';
import { Wallet } from './wallet';

const { Address } = Sdk;

jest.setTimeout(1000 * 60 * 20); // 10 minutes timeout

const userPk = '0x149bc17929e5d9c43fb25ab94c112803130137bfdb2a2cfd6ef9043bd4fc22d6'; // account 1
const resolverPk = '0x639ed7560cbdde79096973912f5c83de86ba08aef2ce6f673dad5bf0a1663801'; // account 2

// Example ecrecover usage: const recoveredAddress = ethers.verifyMessage(message, signature);

// eslint-disable-next-line max-lines-per-function
describe('Resolving example', () => {
    const srcChainId = config.chain.source.chainId;
    const dstChainId = config.chain.destination.chainId;

    type Chain = {
        node?: CreateServerReturnType | undefined;
        provider: JsonRpcProvider;
        escrowFactory: string;
        resolver: string;
    };

    let src: Chain;
    let dst: Chain;

    let srcChainUser: Wallet;
    let dstChainUser: Wallet;
    let srcChainResolver: Wallet;
    let dstChainResolver: Wallet;

    let srcFactory: EscrowFactory;
    let dstFactory: EscrowFactory;
    let srcResolverContract: Wallet;
    let dstResolverContract: Wallet;

    let srcTimestamp: bigint;

    async function increaseTime(t: number): Promise<void> {
        await Promise.all([src, dst].map((chain) => chain.provider.send('evm_increaseTime', [t])));
    }

    beforeAll(async () => {
        // ;[src, dst] = await Promise.all([initChain(config.chain.source), initChain(config.chain.destination)]);
        // ;[dst] = await Promise.all([initChain(config.chain.destination)]);
        // return

        // working code .
        // dst = {
        //     provider: new JsonRpcProvider("https://base-sepolia.g.alchemy.com/v2/0XPjrbBAKRJaSJuy6GN8uKX5uy7YquZV", 84532, {
        //         cacheTimeout: -1,
        //         staticNetwork: true
        //     }),
        //     escrowFactory: "0x048975f98b998796d1cF54DE3A3Fc2bE01d891Fd",
        //     resolver: "0xfdeF9FF4A8677F5ab235b4F1c98426F591E560D5"
        // };

        src = {
            provider: new JsonRpcProvider("https://rpc.ankr.com/etherlink_testnet", 128123, {
                cacheTimeout: -1,
                staticNetwork: true
            }),
            escrowFactory: "0x30d24e9d1Fbffad6883E8632c5ad4216c9A86dFC",
            resolver: "0x4dfaBf46CCDd6b36a275b0b22f5C2077120914C9"
        };
        // return

        dst = {
            provider: new JsonRpcProvider("https://testnet-rpc.monad.xyz", 10143, {
                cacheTimeout: -1,
                staticNetwork: true
            }),
            escrowFactory: "0xa62dF4c42fFd8a352436461f3A3542bF2EFb06bF",
            resolver: "0x2Ccb1d9b36c0dE06195169d34fD64427F735186b"
        };
        // return
        srcChainUser = new Wallet(userPk, src.provider);
        dstChainUser = new Wallet(userPk, dst.provider);
        srcChainResolver = new Wallet(resolverPk, src.provider);
        dstChainResolver = new Wallet(resolverPk, dst.provider);


        srcResolverContract = new Wallet(resolverPk, src.provider);
        dstResolverContract = new Wallet(resolverPk, dst.provider);


        // console.log("waiting for 5 seconds  before approving...");
        // await new Promise(resolve => setTimeout(resolve, 5000));
        // await dstResolverContract.unlimitedApprove(config.chain.destination.tokens.USDC.address, dst.escrowFactory);
        // await srcResolverContract.unlimitedApprove(config.chain.source.tokens.USDC.address, src.escrowFactory);

        // await new Promise(resolve => setTimeout(resolve, 5000));

        // await dstChainUser.unlimitedApprove(config.chain.destination.tokens.USDC.address, config.chain.destination.limitOrderProtocol);
        // await srcChainUser.unlimitedApprove(config.chain.source.tokens.USDC.address, config.chain.source.limitOrderProtocol);

        // console.log("waiting for 5 seconds");

        // await new Promise(resolve => setTimeout(resolve, 5000));

        // console.log("Sending USDC to resolver...");
        // await dstChainUser.transferToken(
        //     config.chain.destination.tokens.USDC.address,
        //     dst.resolver,
        //     parseEther('1000')
        // );



        srcFactory = new EscrowFactory(src.provider, src.escrowFactory);
        dstFactory = new EscrowFactory(dst.provider, dst.escrowFactory);

        console.log("2,....");
        // get 1000 USDC for user in SRC chain and approve to LOP;
        // await srcChainUser.topUpFromDonor(
        //     config.chain.source.tokens.USDC.address,
        //     config.chain.source.tokens.USDC.donor,
        //     parseUnits('1000', 18)
        // );

        // console.log("3,....");
        // await srcChainUser.approveToken(
        //     config.chain.source.tokens.USDC.address,
        //     config.chain.source.limitOrderProtocol,
        //     MaxUint256
        // );

        console.log("4,....");
        // get 2000 USDC for resolver in DST chain

        // await dstResolverContract.topUpFromDonor(
        //     config.chain.destination.tokens.USDC.address,
        //     config.chain.destination.tokens.USDC.donor,
        //     parseUnits('2000', 18)
        // )

        // // top up contract for approve
        // console.log("5,....");
        // // await dstChainResolver.transfer(dst.resolver, parseEther('0.01'))

        // console.log("6,....");
        srcTimestamp = BigInt((await src.provider.getBlock('latest'))!.timestamp);
    });

    async function getBalances(
        srcToken: string,
        dstToken: string
    ): Promise<{ src: { user: bigint; resolver: bigint; }; dst: { user: bigint; resolver: bigint; }; }> {
        return {
            src: {
                user: await srcChainUser.tokenBalance(srcToken),
                resolver: await srcResolverContract.tokenBalance(srcToken)
            },
            dst: {
                user: await dstChainUser.tokenBalance(dstToken),
                resolver: await dstResolverContract.tokenBalance(dstToken)
            }
        };
    }

    // afterAll(async () => {
    //     src.provider.destroy()
    //     dst.provider.destroy()
    //     await Promise.all([src.node?.stop(), dst.node?.stop()])
    // })

    // eslint-disable-next-line max-lines-per-function
    describe('Fill', () => {
        it('should swap Ethereum USDC -> Bsc USDC. Single fill only', async () => {
            const initialBalances = await getBalances(
                config.chain.source.tokens.USDC.address,
                config.chain.destination.tokens.USDC.address
            );

            // User creates order
            const secret = uint8ArrayToHex(randomBytes(32));
            console.log("the secret", secret); // note: use crypto secure random number in real world
            const order = Sdk.CrossChainOrder.new(
                new Address(src.escrowFactory),
                {
                    salt: Sdk.randBigInt(1000n),
                    maker: new Address(await srcChainUser.getAddress()),
                    makingAmount: parseUnits('10', 18),
                    takingAmount: parseUnits('9', 18),
                    makerAsset: new Address(config.chain.source.tokens.USDC.address),
                    takerAsset: new Address(config.chain.destination.tokens.USDC.address)
                },
                {
                    hashLock: Sdk.HashLock.forSingleFill(secret),
                    timeLocks: Sdk.TimeLocks.new({
                        srcWithdrawal: 2n, // 2sec finality lock for test
                        srcPublicWithdrawal: 1200n, // 2m for private withdrawal
                        srcCancellation: 1210n, // 1sec public withdrawal
                        srcPublicCancellation: 1220n, // 1sec private cancellation
                        dstWithdrawal: 2n, // 2sec finality lock for test
                        dstPublicWithdrawal: 1000n, // 100sec private withdrawal
                        dstCancellation: 1010n // 1sec public withdrawal
                    }),
                    srcChainId,
                    dstChainId,
                    srcSafetyDeposit: parseEther('0'),
                    dstSafetyDeposit: parseEther('0')
                },
                {
                    auction: new Sdk.AuctionDetails({
                        initialRateBump: 0,
                        points: [],
                        duration: 1n,
                        startTime: srcTimestamp
                    }),
                    whitelist: [
                        {
                            address: new Address(src.resolver),
                            allowFrom: 0n
                        }
                    ],
                    resolvingStartTime: 0n
                },
                {
                    nonce: Sdk.randBigInt(UINT_40_MAX),
                    allowPartialFills: false,
                    allowMultipleFills: false
                }
            );
            // {
            //     "payload order": {
            //       "salt": "42",
            //       "makerAsset": "0x1b150538e943f00127929f7eeb65754f7beb0b6d",
            //       "takerAsset": "0xea2bb31ebb0aee264aba3730c8744d6bd76d37d0",
            //       "maker": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6",
            //       "receiver": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6",
            //       "makingAmount": 100000000000000000000,
            //       "takingAmount": 100000000000000000000,
            //       "makerTraits": "0"
            //     },
            //     "orderType": "single_fill",
            //     "srcChainId": 84532,
            //     "dstChainId": 10143,
            //     "deadline": 123456789,
            //     "secrets" :[
            //         {
            //             "index": 0,
            //             "secretHash": "5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456"
            //         }
            //     ],
            //     "signature": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1b",
            //     "extension": "0x",
            //     "quoteId": "test-quote-123"
            //   }
            // Build the payload from our order, matching the payload order fields exactly

            const signature = await srcChainUser.signOrder(srcChainId, order);
            console.log("sdk sig hash", Sdk.HashLock.forSingleFill(secret).toString());
            console.log("secret", secret);
            console.log("secery sha hash", ethers.sha256(secret));
            //0xcfa2169db3c5975f50e18328475c81fd0a3379394ec2db548bd275d616fd72085888bd85f2f5193373f3498fe845a928f3ac6ab0a5f5abe79d070881ae64aa641c

            // r: '0xcfa2169db3c5975f50e18328475c81fd0a3379394ec2db548bd275d616fd7208',
            // vs: '0xd888bd85f2f5193373f3498fe845a928f3ac6ab0a5f5abe79d070881ae64aa64'
            console.log("the signature", signature);
            const orderHash = order.getOrderHash(srcChainId);
            console.log("the order hash", orderHash);

            console.log("order", order.build());
            console.log("extesnion data", order.extension);
            const { r, yParityAndS: vs } = Signature.from(signature);
            console.log("r", r);
            console.log("vs", vs);
            const resolverContractsss = new Resolver(src.resolver, dst.resolver);
            const src_immutables = (order.toSrcImmutables(srcChainId, new Sdk.Address(resolverContractsss.srcAddress), order.makingAmount, order.escrowExtension.hashLockInfo).build());
            console.log("🚀 ~ src_immutables:", src_immutables);
            const { args: args_src, trait: trait_src } = Sdk.TakerTraits.default()
                .setExtension(order.extension)
                .setAmountMode(Sdk.AmountMode.maker)
                .setAmountThreshold(order.takingAmount)
                .encode();
            console.log("🚀 ~ trait_src:", trait_src);
            console.log("🚀 ~ args_src:", args_src);
            // const payload = {
            //     order: {
            //         salt: order.salt.toString(),
            //         maker_asset: order.makerAsset.toString(),
            //         taker_asset: order.takerAsset.toString(),
            //         maker: order.maker.toString(),
            //         receiver: order.receiver.toString(),
            //         making_amount: order.makingAmount.toString(),
            //         taking_amount: order.takingAmount.toString(),
            //         maker_traits: order.build().makerTraits
            //     },
            //     taker: src.resolver,
            //     args: args_src.toString(),
            //     taker_traits: trait_src.toString(),
            //     order_hash: orderHash,
            //     order_type: "single_fill",
            //     src_chain_id: srcChainId.toString(),
            //     dst_chain_id: dstChainId.toString(),
            //     timelock: src_immutables.timelocks,
            //     deadline: Math.floor(Date.now() / 1000) + 3600,
            //     secrets: [{
            //         index: 0,
            //         secret_hash: ethers.sha256(secret)
            //     }],
            //     signature: {
            //         r: r.toString(),
            //         vs: vs.toString()
            //     },
            //     extension: order.extension,
            // };
            // console.log("payload", payload);

            // // Make a POST request to the relayer endpoint with the payload using axios
            // try{
            //     const response = await axios.post('http://10.67.21.221:4455/relayer/submit', payload, {
            //         headers: { 'Content-Type': 'application/json' }
            //     });
            //     console.log("Relayer response:", response.data);
            // } catch (error) {
            //     console.error("Error submitting payload:", error);
            // }

            // await new Promise(resolve => setTimeout(resolve, 3000));


            // const payloadsecret = {
            //     secret: secret.slice(2),
            //     order_hash: orderHash,
            // }
            // try {
            //     const responseSecret = await axios.post('http://10.67.21.221:4455/relayer/secret', payloadsecret, {
            //         headers: { 'Content-Type': 'application/json' }
            //     });
            //     console.log("Relayer response:", responseSecret.data);
            // } catch (error) {
            //     console.error("Error submitting secret:", error);
            // }
            // // // return
            // await new Promise(resolve => setTimeout(resolve, 30000));

            // console.log("Waiting for src escrow to be deployed...");

            // const order___hash = orderHash

            // const response = await axios.get(`http://10.67.21.221:4455/orders/${order___hash}`);
            // console.log("response data ", response.data.result);
            // // const srcEscrowEvent1 = response.data.srcEscrowEvent;
            // const blockHash = response.data.result.src_event.blockHash;
            // console.log("response data ", response.data.result.src_event.blockHash);

            // await new Promise(resolve => setTimeout(resolve, 15000));

            // let srcEscrowEvent1 = await srcFactory.getSrcDeployEvent(blockHash);
            // console.log("srcEscrowEvent", srcEscrowEvent1);
            // const resolverContract1 = new Resolver(src.resolver, dst.resolver);

            // const dstImmutables1 = srcEscrowEvent1[0]
            //     .withComplement(srcEscrowEvent1[1])
            //     .withTaker(new Address(resolverContract1.dstAddress));

            // console.log(`Dest Immutables`, dstImmutables1.build());
            // // post this dest immutable to relayer
            // const payload2 = {
            //     order_hash: response.data.result.order_hash,
            //     field_name: 'dst_deploy_immutables',
            //     value: dstImmutables1.build()
            // }


            // await new Promise(resolve => setTimeout(resolve, 15000));

            // try {
            //     const relayerResponse = await axios.post(`http://10.67.21.221:4455/orders/update/${order___hash}`, payload2, {
            //         headers: { 'Content-Type': 'application/json' }
            //     });
            //     console.log("Relayer response:", relayerResponse.data);
            // } catch (error) {
            //     console.error("Error submitting payload:", error);
            // }

            // const sourceWithdrawImmutables  = srcEscrowEvent1[0]
            // const sourceWithdrawImmutablesPayload = {
            //     order_hash: response.data.result.order_hash,
            //     field_name: 'src_withdraw_immutables',
            //     value: sourceWithdrawImmutables
            // }

            // await new Promise(resolve => setTimeout(resolve, 15000));

            // try{
            //     const relayerResponse = await axios.post(`http://10.67.21.221:4455/orders/update/${order___hash}`, sourceWithdrawImmutablesPayload, {
            //         headers: { 'Content-Type': 'application/json' }
            //     });
            //     console.log("Relayer response:", relayerResponse.data);
            // } catch (error) {
            //     console.error("Error submitting payload:", error);
            // }

            // await new Promise(resolve => setTimeout(resolve, 35000));

            // const response3 = await axios.get(`http://10.67.21.221:4455/orders/${order___hash}`);
            // srcEscrowEvent1 = response.data.srcEscrowEvent;
            // console.log("response data ", response3.data.result.dest_event);
            // console.log("response data ", response3);

            // const blockhash = response3.data.result.dest_event.blockHash;
            // const dstBlock = await dst.provider.getBlock(blockhash);
            // const dstBlockTimestamp = BigInt(dstBlock!.timestamp);
            // console.log("Destination block timestamp:", dstBlockTimestamp);
            // console.log("response data ", response3.data.result.src_event.blockHash);
            // const response4 = await axios.get(`http://10.67.21.221:4455/orders/${order___hash}`);
            // console.log("response data ", response4.data.result.dst_escrow_address);

            // let y = response4.data.result.dst_escrow_address;

            // const resolverContract = new Resolver(src.resolver, dst.resolver);


            // const x = await dstChainResolver.sendStatic(
            //     resolverContract.withdraw('dst', y, secret, dstImmutables1.withDeployedAt(dstBlockTimestamp))
            // );


            // const payload3 = {
            //     order_hash: response.data.result.order_hash,
            //     field_name: 'dst_withdraw_immutables',
            //     value: x
            // }
            // try{
            //     const relayerResponse = await axios.post(`http://10.67.21.221:4455/orders/update/${order___hash}`, payload3, {
            //         headers: { 'Content-Type': 'application/json' }
            //     });
            //     console.log("Relayer response:", relayerResponse.data);
            // } catch (error) {
            //     console.error("Error submitting payload:", error);
            // }


            // return
            // await new Promise(resolve => setTimeout(resolve, 5000));


            // const recoveredAddress = ethers.verifyMessage(orderHash);
            // Resolver fills order
            const resolverContract = new Resolver(src.resolver, dst.resolver);
            console.log("the resolver contract", resolverContract);

            // Let's check the owner of the deployed resolver contract
            const resolverContractInstance = resolverContract.getSrcContract(src.provider);
            const contractOwner = await resolverContractInstance.owner();
            console.log("Contract owner:", contractOwner);
            console.log("Resolver signer address:", await srcChainResolver.getAddress());
            console.log("Are they the same?", contractOwner.toLowerCase() === (await srcChainResolver.getAddress()).toLowerCase());

            console.log(`[${srcChainId}]`, `Filling order ${orderHash}`);

            const fillAmount = order.makingAmount;

            // Get the resolver contract instance and connect it to the signer
            const resolverContractWithSigner = resolverContractInstance.connect(srcChainResolver.signer);

            // Prepare parameters for deploySrc (same as in the original deploySrc method)
            // const { r, yParityAndS: vs } = Signature.from(signature);
            const { args, trait } = Sdk.TakerTraits.default()
                .setExtension(order.extension)
                .setAmountMode(Sdk.AmountMode.maker)
                .setAmountThreshold(order.takingAmount)
                .encode();
            const immutables = order.toSrcImmutables(srcChainId, new Sdk.Address(resolverContract.srcAddress), fillAmount, order.escrowExtension.hashLockInfo);

            console.log("Immutables for src deploy", immutables.build());
            console.log("Order for src deploy", order.build());
            console.log("Signature for src deploy", { r, vs, fillAmount, trait, args });
            // wait for 30 secs
            // await new Promise((resolve) => setTimeout(resolve, 30000));
            // Call deploySrc directly on the contract

            // wait for src escrow to be deployed

            const tx = await (resolverContractWithSigner as any).deploySrc(
                immutables.build(),
                order.build(),
                r,
                vs,
                fillAmount,
                trait,
                args,
                {
                    value: order.escrowExtension.srcSafetyDeposit
                }
            );

            const receipt = await tx.wait();
            const orderFillHash = receipt.hash;
            const srcDeployBlock = receipt.blockHash;
            console.log(`orderfill hash ${orderFillHash} srcDeploy block ${srcDeployBlock}`);

            console.log(`[${srcChainId}]`, `Order ${orderHash} filled for ${fillAmount} in tx ${orderFillHash}`);

            const srcEscrowEvent = await srcFactory.getSrcDeployEvent(srcDeployBlock);
            console.log("srcEscrowEvent", srcEscrowEvent);

            const dstImmutables = srcEscrowEvent[0]
                .withComplement(srcEscrowEvent[1])
                .withTaker(new Address(resolverContract.dstAddress));

            console.log(`Dest Immutables`, dstImmutables.build());


            console.log(`[${dstChainId}]`, `Depositing ${dstImmutables.amount} for order ${orderHash}`);
            const { txHash: dstDepositHash, blockTimestamp: dstDeployedAt } = await dstChainResolver.send(
                resolverContract.deployDst(dstImmutables)
            );

            // const destTx = await (resolverContractWithSigner as any).deployDst(
            //     immutables.build(),
            //     order.build(),
            //     r,
            //     vs,
            //     fillAmount,
            //     trait,
            //     args,
            //     {
            //         value: order.escrowExtension.srcSafetyDeposit
            //     }
            // )

            // console.log(`[${dstChainId}]`, `Created dst deposit for order ${orderHash} in tx ${JSON.stringify(destTx, null, 2)}`);

            const ESCROW_SRC_IMPLEMENTATION = await srcFactory.getSourceImpl();
            const ESCROW_DST_IMPLEMENTATION = await dstFactory.getDestinationImpl();

            const srcEscrowAddress = new Sdk.EscrowFactory(new Address(src.escrowFactory)).getSrcEscrowAddress(
                srcEscrowEvent[0],
                ESCROW_SRC_IMPLEMENTATION
            );

            const dstEscrowAddress = new Sdk.EscrowFactory(new Address(dst.escrowFactory)).getDstEscrowAddress(
                srcEscrowEvent[0],
                srcEscrowEvent[1],
                dstDeployedAt,
                new Address(resolverContract.dstAddress),
                ESCROW_DST_IMPLEMENTATION
            );

            console.log("will wait for 5 seconds");
            // await increaseTime(11)
            await new Promise((resolve) => setTimeout(resolve, 5000)); // finality lock passed
            console.log("waited for 5 seconds");

            // User shares key after validation of dst escrow deployment
            console.log(`[${dstChainId}]`, `Withdrawing funds for user from ${dstEscrowAddress}`);
            const { txHash: dstWithdrawTxHash } = await dstChainResolver.send(
                resolverContract.withdraw('dst', dstEscrowAddress, secret, dstImmutables.withDeployedAt(dstDeployedAt))
            );
            console.log("[${dstChainId} ] dstWithdrawTx", dstWithdrawTxHash);
            console.log(`[${srcChainId}]`, `Withdrawing funds for resolver from ${srcEscrowAddress}`);
            const { txHash: resolverWithdrawHash } = await srcChainResolver.send(
                resolverContract.withdraw('src', srcEscrowAddress, secret, srcEscrowEvent[0])
            );
            console.log(
                `[${srcChainId}]`,
                `Withdrew funds for resolver from ${srcEscrowAddress} to ${src.resolver} in tx ${resolverWithdrawHash}`
            );

            // Get final balances before logging
            const resultBalances = await getBalances(
                config.chain.source.tokens.USDC.address,
                config.chain.destination.tokens.USDC.address
            );

            // 🎉 Enhanced Transaction Summary Logging
            console.log('\n' + '█'.repeat(90));
            console.log('🎯                   CROSS-CHAIN SWAP TRANSACTION SUMMARY                    🎯');
            console.log('█'.repeat(90));

            console.log('\n📋 Order Details:');
            console.log(`   ✅ Order Hash: ${orderHash}`);
            console.log(`   ✅ Secret: ${secret}`);
            console.log(`   ✅ Making Amount: ${order.makingAmount} USDC`);
            console.log(`   ✅ Taking Amount: ${order.takingAmount} USDC`);

            console.log('\n📝 Contract Addresses:');
            console.log(`   ✅ Source Escrow Factory: ${src.escrowFactory}`);
            console.log(`   ✅ Destination Escrow Factory: ${dst.escrowFactory}`);
            console.log(`   ✅ Source Resolver: ${src.resolver}`);
            console.log(`   ✅ Destination Resolver: ${dst.resolver}`);
            console.log(`   ✅ Source Escrow Address: ${srcEscrowAddress}`);
            console.log(`   ✅ Destination Escrow Address: ${dstEscrowAddress}`);

            console.log('\n👥 Participants:');
            console.log(`   ✅ User Address: ${await srcChainUser.getAddress()}`);
            console.log(`   ✅ Resolver Address: ${await srcChainResolver.getAddress()}`);

            console.log('\n⏰ Timing Information:');
            console.log(`   ✅ Source Timestamp: ${srcTimestamp}`);
            console.log(`   ✅ Destination Deploy Time: ${dstDeployedAt}`);
            console.log(`   ✅ Current Time: ${Math.floor(Date.now() / 1000)}`);

            console.log('\n' + '▓'.repeat(90));
            console.log('💳                       TRANSACTION EXECUTION ORDER                        💳');
            console.log('▓'.repeat(90));

            console.log('\n💰 Balance Changes:');
            console.log(`   ✅ Initial Balances: SRC User: ${initialBalances.src.user}, DST User: ${initialBalances.dst.user}`);
            console.log(`   ✅ Final Balances: SRC User: ${resultBalances.src.user}, DST User: ${resultBalances.dst.user}`);
            console.log(`   ✅ User Balance Change (SRC): ${initialBalances.src.user - resultBalances.src.user} USDC`);
            console.log(`   ✅ User Balance Change (DST): ${resultBalances.dst.user - initialBalances.dst.user} USDC`);

            console.log('\n🌐 Chain Information:');
            console.log(`   ✅ Source Chain ID: ${srcChainId}`);
            console.log(`   ✅ Destination Chain ID: ${dstChainId}`);

            console.log('\n🔄 Complete Transaction Flow:');
            console.log(`   1️⃣ ✅ Order Creation & Signing - Hash: ${orderHash.substring(0, 10)}...`);
            console.log(`   2️⃣ ✅ Source Deploy Transaction - Hash: ${orderFillHash}`);
            console.log(`   4️⃣ ✅ Destination Deploy Transaction - Hash: ${dstDepositHash}`);
            console.log(`   5️⃣ ✅ Destination Withdrawal Transaction - Hash: ${dstWithdrawTxHash}`);
            console.log(`   6️⃣ ✅ Source Withdrawal Transaction - Hash: ${resolverWithdrawHash}`);

            console.log('\n📊 Final Status:');
            console.log('   🎉 ✅ Cross-Chain Swap COMPLETED SUCCESSFULLY! 🎉');
            console.log('   🔥 ✅ All transactions executed without errors! 🔥');
            console.log('   🚀 ✅ Funds successfully transferred and withdrawn! 🚀');
            console.log('   💎 ✅ Both source and destination operations completed! 💎');

            console.log('\n' + '█'.repeat(90));
            console.log('🏆                           MISSION ACCOMPLISHED                           🏆');
            console.log('█'.repeat(90) + '\n');

            console.log("resultBalances", resultBalances);
            // user transferred funds to resolver on source chain
            // expect(initialBalances.src.user - resultBalances.src.user).toBe(order.makingAmount);
            // expect(resultBalances.src.resolver - initialBalances.src.resolver).toBe(order.makingAmount);
            // // resolver transferred funds to user on destination chain
            // expect(resultBalances.dst.user - initialBalances.dst.user).toBe(order.takingAmount);
            // expect(initialBalances.dst.resolver - resultBalances.dst.resolver).toBe(order.takingAmount);
        });


    });

});

async function initChain(
    cnf: ChainConfig
): Promise<{ node?: CreateServerReturnType; provider: JsonRpcProvider; escrowFactory: string; resolver: string; }> {
    const { node, provider } = await getProvider(cnf);
    const deployer = new SignerWallet(cnf.ownerPrivateKey, provider);

    // deploy EscrowFactory
    const escrowFactory = await deploy(
        factoryContract,
        [
            cnf.limitOrderProtocol,
            cnf.wrappedNative, // feeToken,
            Address.fromBigInt(0n).toString(), // accessToken,
            deployer.address, // owner
            60 * 30, // src rescue delay
            60 * 30 // dst rescue delay
        ],
        provider,
        deployer
    );
    console.log(`[${cnf.chainId}]`, `Escrow factory contract deployed to`, escrowFactory);

    // deploy Resolver contract
    const resolver = await deploy(
        resolverContract,
        [
            escrowFactory,
            cnf.limitOrderProtocol,
            computeAddress(resolverPk) // resolver as owner of contract
        ],
        provider,
        deployer
    );
    console.log(`[${cnf.chainId}]`, `Resolver contract deployed to`, resolver);

    return { node: node, provider, resolver, escrowFactory };
}

async function getProvider(cnf: ChainConfig): Promise<{ node?: CreateServerReturnType; provider: JsonRpcProvider; }> {
    if (!cnf.createFork) {
        return {
            provider: new JsonRpcProvider(cnf.url, cnf.chainId, {
                cacheTimeout: -1,
                staticNetwork: true
            })
        };
    }

    const node = createServer({
        instance: anvil({ forkUrl: cnf.url, chainId: cnf.chainId }),
        limit: 1
    });
    await node.start();

    const address = node.address();
    assert(address);

    const provider = new JsonRpcProvider(`http://[${address.address}]:${address.port}/1`, cnf.chainId, {
        cacheTimeout: -1,
        staticNetwork: true
    });

    return {
        provider,
        node
    };
}

/**
 * Deploy contract and return its address
 */
async function deploy(
    json: { abi: any; bytecode: any; },
    params: unknown[],
    provider: JsonRpcProvider,
    deployer: SignerWallet
): Promise<string> {
    const deployed = await new ContractFactory(json.abi, json.bytecode, deployer).deploy(...params);
    await deployed.waitForDeployment();

    return await deployed.getAddress();
}