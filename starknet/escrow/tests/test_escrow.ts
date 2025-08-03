import {
  Account,
  cairo,
  CallData,
  Contract,
  RpcProvider,
  stark as sn,
} from "starknet";
import { ethers, parseEther, sha256 } from "ethers";
import { randomBytes } from "crypto";
import path from "path";
import { readFile } from "fs/promises";
import { ABI } from "../abi";
import { 
  STARKNET_DEVNET_URL, 
  accounts, 
  STARK, 
  ETH, 
  ZERO_ADDRESS, 
  TIMELOCK, 
  AMOUNT 
} from "./config";

describe("Starknet ESCROW", () => {
  const starknetProvider = new RpcProvider({
    nodeUrl: STARKNET_DEVNET_URL,
  });

  let stark: Contract;
  let starknetESCROW: Contract;
  let callData: CallData;

  let alice: Account;
  let bob: Account;
  let charlie: Account;

  let secret1: string;
  let secret2: string;
  let secret3: string;
  let secret4: string;
  let secret5: string;
  let secret6: string;
  let secret7: string;

  let secretHash1: number[];
  let secretHash2: number[];
  let secretHash3: number[];
  let secretHash4: number[];
  let secretHash5: number[];
  let secretHash6: number[];
  let secretHash7: number[];

  let CHAIN_ID: string;

  let sierraCode: any;
  let casmCode: any;

  // Helper function to convert hex to u32 array
  const hexToU32Array = (hex: string): number[] => {
    const bytes = ethers.getBytes(hex);
    const u32Array = [];
    for (let i = 0; i < bytes.length; i += 4) {
      const chunk = bytes.slice(i, i + 4);
      const value = new DataView(chunk.buffer).getUint32(0, false);
      u32Array.push(value);
    }
    return u32Array;
  };

  // Helper function to generate order ID
  const generateOrderId = (): string => {
    // Generate a random felt252-compatible value
    const randomValue = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
    return "0x" + randomValue.toString(16);
  };

  // Helper function to get compiled code
  const getCompiledCode = async (contractName: string) => {
    const sierraFilePath = path.join(
      __dirname,
      `../target/dev/${contractName}.contract_class.json`
    );
    const casmFilePath = path.join(
      __dirname,
      `../target/dev/${contractName}.compiled_contract_class.json`
    );
  
    const [sierraCode, casmCode] = await Promise.all([
      readFile(sierraFilePath, "utf8").then(JSON.parse),
      readFile(casmFilePath, "utf8").then(JSON.parse)
    ]);
  
    return {
      sierraCode,
      casmCode,
    };
  };

  const deployESCROW = async (sierraCode: any, casmCode: any) : Promise<{sierraCode: any, address: string}> => {

    callData = new CallData(sierraCode.abi);

    const constructor = callData.compile("constructor", {});

    const deployResponse = await alice.declareAndDeploy({
      contract: sierraCode,
      casm: casmCode,
      constructorCalldata: constructor,
      salt: sn.randomAddress(),
    });
    return {
      sierraCode: sierraCode.abi,
      address: deployResponse.deploy.contract_address
    };
  };

  beforeAll(async () => {
    secret1 = sha256(randomBytes(32));
    secret2 = sha256(randomBytes(32));
    secret3 = sha256(randomBytes(32));
    secret4 = sha256(randomBytes(32));
    secret5 = sha256(randomBytes(32));
    secret6 = sha256(randomBytes(32));
    secret7 = sha256(randomBytes(32));

    secretHash1 = hexToU32Array(sha256(secret1));
    secretHash2 = hexToU32Array(sha256(secret2));
    secretHash3 = hexToU32Array(sha256(secret3));
    secretHash4 = hexToU32Array(sha256(secret4));
    secretHash5 = hexToU32Array(sha256(secret5));
    secretHash6 = hexToU32Array(sha256(secret6));
    secretHash7 = hexToU32Array(sha256(secret7));

    CHAIN_ID = (await starknetProvider.getChainId()).toString();
    console.log("Chain ID: ", CHAIN_ID);

    alice = new Account(
      starknetProvider,
      accounts[0].address,
      accounts[0].privateKey,
      "1",
      "0x3"
    );

    bob = new Account(
      starknetProvider,
      accounts[1].address,
      accounts[1].privateKey,
      "1",
      "0x3"
    );
    charlie = new Account(
      starknetProvider,
      accounts[2].address,
      accounts[2].privateKey,
      "1",
      "0x3"
    );

    const { sierraCode: sierraCode2, casmCode: casmCode2 } = await getCompiledCode("escrow_ESCROW");

    const contractData = await starknetProvider.getClassAt(STARK);
    stark = new Contract(contractData.abi, STARK, starknetProvider);
    // const { sierraCode, address } = await deployESCROW(sierraCode2, casmCode2);
    const address = "0x2b3021e22c36d1b709c819d4c08b5ffcfe745eaac6aa3e9c141e098b802287c";
    starknetESCROW = new Contract(sierraCode2.abi, address, starknetProvider).typedv2(ABI);
    let ok = starknetESCROW.functions;
    console.log("Functions: ", ok);

    callData = new CallData(sierraCode2.abi);

    console.log("ESCROW address: ", address);

    stark.connect(alice);
    let approve_res = await stark.approve(starknetESCROW.address, parseEther("200")); // Increased to cover amount + security deposit
    console.log("Approve: ", approve_res.transaction_hash);
    
    await starknetProvider.waitForTransaction(approve_res.transaction_hash);
  
    let allowance = await stark.allowance(alice.address, starknetESCROW.address);
    console.log("Allowance: ", allowance);
  }, 100000);

  describe("- Pre-Conditions -", () => {
    it("Escrow contract should be deployed", async () => {
      expect(starknetESCROW).toBeDefined();
    });

  });

  // describe("-- ESCROW Transfer Funds --", () => {
  //   it("Should transfer funds from alice to escrow", async () => {
  //     let transfer_funds_res = await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "transfer_funds",
  //       calldata: [STARK, cairo.uint256(parseEther("1")).low, cairo.uint256(parseEther("1")).high]
  //     });
  //     console.log("Transfer funds: ", transfer_funds_res);
  //   });
  // });

  // describe("-- ESCROW Verify Signature --", () => {
  //   it("Should verify signature", async () => {

  //     const SIGNATURE_VERIFY_TYPE = {
  //       StarknetDomain: [
  //         { name: "name", type: "shortstring" },
  //         { name: "version", type: "shortstring" },
  //         { name: "chainId", type: "shortstring" },
  //         { name: "revision", type: "shortstring" },
  //       ],
  //       VerifySignature: [
  //         { name: "token", type: "ContractAddress" },
  //         { name: "amount", type: "u256" },
  //       ],
  //     };

  //     const DOMAIN = {
  //       name: "ESCROW",
  //       version: shortString.encodeShortString("1"),
  //       chainId: CHAIN_ID,
  //       revision: TypedDataRevision.ACTIVE,
  //     };

  //     const userIntent = {
  //       token: STARK,
  //       amount: cairo.uint256(AMOUNT),
  //     };

  //     const typedData = {
  //       domain: DOMAIN,
  //       primaryType: "VerifySignature",
  //       types: SIGNATURE_VERIFY_TYPE,
  //       message: userIntent,
  //     };

  //     let signature = await alice.signMessage(typedData) as WeierstrassSignatureType;
  //     const { r, s } = signature;
  //     const signatureArray = [r, s];

  //     console.log("Signature: ", signature);
  //     let verify_signature_res = await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "verify_signature",
  //       calldata: [STARK, cairo.uint256(AMOUNT).low, cairo.uint256(AMOUNT).high,signatureArray]
  //     });

  //     console.log("Verify signature: ", verify_signature_res);
  //   });
  // });

  // describe("-- ESCROW Create Outbound Order --", () => {
  //   it("Should create outbound order with valid signature", async () => {
  //     // Define the UserIntent type structure for signing
  //     const USER_INTENT_TYPE = {
  //       StarknetDomain: [
  //         { name: "name", type: "shortstring" },
  //         { name: "version", type: "shortstring" },
  //         { name: "chainId", type: "shortstring" },
  //         { name: "revision", type: "shortstring" },
  //       ],
  //       UserIntent: [
  //         { name: "salt", type: "u256" },
  //         { name: "maker", type: "ContractAddress" },
  //         { name: "receiver", type: "ContractAddress" },
  //         { name: "maker_asset", type: "ContractAddress" },
  //         { name: "taker_asset", type: "ContractAddress" },
  //         { name: "making_amount", type: "u256" },
  //         { name: "taking_amount", type: "u256" },
  //       ],
  //     };

  //     const DOMAIN = {
  //       name: "ESCROW",
  //       version: shortString.encodeShortString("1"),
  //       chainId: CHAIN_ID,
  //       revision: TypedDataRevision.ACTIVE,
  //     };

  //     const userIntent = {
  //       salt: cairo.uint256(123456),
  //       maker: alice.address,
  //       receiver: bob.address,
  //       maker_asset: STARK,
  //       taker_asset: ETH,
  //       making_amount: cairo.uint256(parseEther("0.1")),
  //       taking_amount: cairo.uint256(parseEther("0.1")),
  //     };

  //     console.log("UserIntent: ", userIntent);

  //     // Create typed data for signing
  //     const typedData: TypedData = {
  //       domain: DOMAIN,
  //       primaryType: "UserIntent",
  //       types: USER_INTENT_TYPE,
  //       message: userIntent,
  //     };

  //     console.log("TypedData: ", typedData);

  //     // Sign the UserIntent with alice's account
  //     const signature = (await alice.signMessage(typedData)) as WeierstrassSignatureType;
  //     const { r, s } = signature;

  //     // Convert signature to the format expected by the contract
  //     const signatureArray = [r, s];

  //     console.log("Signature: ", signature);

  //     let isVerified = await alice.verifyMessageInStarknet(typedData, signature, alice.address);
  //     console.log("Is verified: ", isVerified);

  //     const orderHash = generateOrderId();

  //     console.log("AMOUNT: ", parseEther("0.1"));
  //     console.log("AMOUNT_HIGH: ", cairo.uint256(parseEther("0.1")).high);
  //     console.log("AMOUNT_LOW: ", cairo.uint256(parseEther("0.1")).low);

  //     const input = {
  //       user_intent: {
  //         salt: userIntent.salt,
  //         maker: userIntent.maker,
  //         receiver: userIntent.receiver,
  //         maker_asset: userIntent.maker_asset,
  //         taker_asset: userIntent.taker_asset,
  //         making_amount: userIntent.making_amount,
  //         taking_amount: userIntent.taking_amount,
  //       },
  //       signature: signatureArray,
  //       token: STARK,
  //       order_hash: orderHash,
  //       // user_address: alice.address,
  //       // resolver_address: bob.address,
  //       user_address: alice.address,
  //       resolver_address: bob.address,
  //       timelock: TIMELOCK,
  //       secret_hash: secretHash1,
  //       amount: cairo.uint256(parseEther("0.1")).low
  //     };

  //     // const call_data = [
  //     //   7,
  //     //   userIntent.salt.low,
  //     //   userIntent.salt.high,
  //     //   userIntent.maker,
  //     //   userIntent.receiver,
  //     //   userIntent.maker_asset,
  //     //   userIntent.taker_asset,
  //     //   userIntent.making_amount.low,
  //     //   userIntent.making_amount.high,
  //     //   userIntent.taking_amount.low,
  //     //   userIntent.taking_amount.high,
  //     //   ...signatureArray,
  //     //   STARK,
  //     //   orderHash,
  //     //   alice.address,
  //     //   bob.address,
  //     //   TIMELOCK,
  //     //   cairo.uint256(parseEther("0.1")).low,
  //     //   cairo.uint256(parseEther("0.1")).high,
  //     //   ...secretHash1
  //     // ]

  //     // console.log("Call data: ", call_data);

  //     // let create_outbound_order_res = await starknetESCROW.create_outbound_order(input);
  //     // console.log("Create outbound order: ", create_outbound_order_res);

  //     let create_outbound_order_res = await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_outbound_order",
  //       calldata: callData.compile("create_outbound_order", {
  //         user_intent: userIntent,
  //         signature: signatureArray,
  //         token: STARK,
  //         order_hash: orderHash,
  //         user_address: alice.address,
  //         resolver_address: bob.address,
  //         timelock: TIMELOCK,
  //       })
  //     });
  //     console.log("Create outbound order: ", create_outbound_order_res);
  //   });
  // });

  //   // it("Should not create outbound order with invalid signature", async () => {
  //   //   // Define the UserIntent type structure for signing
  //   //   const USER_INTENT_TYPE = {
  //   //     StarknetDomain: [
  //   //       { name: "name", type: "shortstring" },
  //   //       { name: "version", type: "shortstring" },
  //   //       { name: "chainId", type: "shortstring" },
  //   //       { name: "revision", type: "shortstring" },
  //   //     ],
  //   //     UserIntent: [
  //   //       { name: "salt", type: "u256" },
  //   //       { name: "maker", type: "ContractAddress" },
  //   //       { name: "receiver", type: "ContractAddress" },
  //   //       { name: "maker_asset", type: "ContractAddress" },
  //   //       { name: "taker_asset", type: "ContractAddress" },
  //   //       { name: "making_amount", type: "u256" },
  //   //       { name: "taking_amount", type: "u256" },
  //   //     ],
  //   //   };

  //   //   const DOMAIN = {
  //   //     name: "ESCROW",
  //   //     version: shortString.encodeShortString("1"),
  //   //     chainId: CHAIN_ID,
  //   //     revision: TypedDataRevision.ACTIVE,
  //   //   };

  //   //   const userIntent = {
  //   //     salt: cairo.uint256(123456),
  //   //     maker: alice.address,
  //   //     receiver: bob.address,
  //   //     maker_asset: STARK,
  //   //     taker_asset: ZERO_ADDRESS,
  //   //     making_amount: cairo.uint256(AMOUNT),
  //   //     taking_amount: cairo.uint256(AMOUNT),
  //   //   };

  //   //   // Create typed data for signing
  //   //   const typedData: TypedData = {
  //   //     domain: DOMAIN,
  //   //     primaryType: "UserIntent",
  //   //     types: USER_INTENT_TYPE,
  //   //     message: userIntent,
  //   //   };

  //   //   // Sign with alice but use wrong signer (bob) for verification
  //   //   const signature = (await alice.signMessage(typedData)) as WeierstrassSignatureType;
  //   //   const { r, s } = signature;

  //   //   // Create an invalid signature by modifying the values
  //   //   const invalidSignature = [r + 1n, s + 1n];

  //   //   const orderHash = generateOrderId();

  //   //   const input = {
  //   //     user_intent: {
  //   //       salt: userIntent.salt,
  //   //       maker: userIntent.maker,
  //   //       receiver: userIntent.receiver,
  //   //       maker_asset: userIntent.maker_asset,
  //   //       taker_asset: userIntent.taker_asset,
  //   //       making_amount: userIntent.making_amount,
  //   //       taking_amount: userIntent.taking_amount,
  //   //     },
  //   //     signature: invalidSignature, // Invalid signature
  //   //     token: STARK,
  //   //     order_hash: orderHash,
  //   //     user_address: alice.address,
  //   //     resolver_address: bob.address,
  //   //     timelock: TIMELOCK,
  //   //     amount: cairo.uint256(AMOUNT),
  //   //     secret_hash: secretHash2,
  //   //   };

  //   //   await expect(
  //   //     alice.execute({
  //   //       contractAddress: starknetESCROW.address,
  //   //       entrypoint: "create_outbound_order",
  //   //       calldata: callData.compile("create_outbound_order", input),
  //   //     })
  //   //   ).rejects.toThrow("ESCROW: invalid user signature");
  //   // });
  // });


  // describe("-- ESCROW Create Inbound Order --", () => {
  //   it("Should create inbound order without signature", async () => {
  //     const orderHash = generateOrderId(
  //       CHAIN_ID,
  //       bob.address,
  //       alice.address,
  //       TIMELOCK,
  //       AMOUNT,
  //       secretHash3
  //     );

  //     const input = {
  //       token: STARK,
  //       order_hash: orderHash,
  //       resolver_address: bob.address,
  //       user_address: alice.address,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash3,
  //     };

  //     await bob.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_inbound_order",
  //       calldata: callData.compile("create_inbound_order", input),
  //     });

  //     // Verify order was created
  //   const order = await starknetESCROW.get_order(STARK, orderHash);
  //     expect(order.initiator).toBe(BigInt(bob.address));
  //     expect(order.redeemer).toBe(BigInt(alice.address));
  //     expect(order.amount).toBe(AMOUNT);
  //     expect(order.timelock).toBe(TIMELOCK);
  //     expect(order.is_fulfilled).toBe(false);
  //   });
  // });

  // describe("-- ESCROW Withdraw --", () => {
  //   it("Should withdraw with correct secret", async () => {
  //     // First create an order
  //     const orderHash = generateOrderId(
  //       CHAIN_ID,
  //       alice.address,
  //       bob.address,
  //       TIMELOCK,
  //       AMOUNT,
  //       secretHash4
  //     );

  //     const input = {
  //       token: STARK,
  //       order_hash: orderHash,
  //       resolver_address: alice.address,
  //       user_address: bob.address,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash4,
  //     };

  //     await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_inbound_order",
  //       calldata: callData.compile("create_inbound_order", input),
  //     });

  //     // Now withdraw with correct secret
  //     const bobOldBalance = await stark.balanceOf(bob.address);
  //     const secret = hexToU32Array(secret4);

  //     await bob.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "withdraw",
  //       calldata: {
  //         token: STARK,
  //         order_hash: orderHash,
  //         secret: secret,
  //       },
  //     });

  //     const bobBalanceAfterRedeem = await stark.balanceOf(bob.address);
  //     expect(bobBalanceAfterRedeem).toBe(bobOldBalance + AMOUNT);

  //     // Verify order is fulfilled
  //     const order = await starknetESCROW.get_order(STARK, orderHash);
  //     expect(order.is_fulfilled).toBe(true);
  //   });

  //   it("Should not withdraw with incorrect secret", async () => {
  //     const orderHash = generateOrderId(
  //       CHAIN_ID,
  //       alice.address,
  //       bob.address,
  //       TIMELOCK,
  //       AMOUNT,
  //       secretHash5
  //     );

  //     const input = {
  //       token: STARK,
  //       order_hash: orderHash,
  //       resolver_address: alice.address,
  //       user_address: bob.address,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash5,
  //     };

  //     await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_inbound_order",
  //       calldata: callData.compile("create_inbound_order", input),
  //     });

  //     const invalidSecret = hexToU32Array(sha256(randomBytes(32)));

  //     await expect(
  //       bob.execute({
  //         contractAddress: starknetESCROW.address,
  //         entrypoint: "withdraw",
  //         calldata: {
  //           token: STARK,
  //           order_hash: orderHash,
  //           secret: invalidSecret,
  //         },
  //       })
  //     ).rejects.toThrow("ESCROW: incorrect secret");
  //   });

  //   it("Should not withdraw already fulfilled order", async () => {
  //     const orderHash = generateOrderId(
  //       CHAIN_ID,
  //       alice.address,
  //       bob.address,
  //       TIMELOCK,
  //       AMOUNT,
  //       secretHash6
  //     );

  //     const input = {
  //       token: STARK,
  //       order_hash: orderHash,
  //       resolver_address: alice.address,
  //       user_address: bob.address,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash6,
  //     };

  //     await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_inbound_order",
  //       calldata: callData.compile("create_inbound_order", input),
  //     });

  //     // First withdrawal should succeed
  //     const secret = hexToU32Array(secret6);
  //     await bob.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "withdraw",
  //       calldata: {
  //         token: STARK,
  //         order_hash: orderHash,
  //         secret: secret,
  //       },
  //     });

  //     // Second withdrawal should fail
  //     await expect(
  //       bob.execute({
  //         contractAddress: starknetESCROW.address,
  //         entrypoint: "withdraw",
  //         calldata: {
  //           token: STARK,
  //           order_hash: orderHash,
  //           secret: secret,
  //         },
  //       })
  //     ).rejects.toThrow("ESCROW: order fulfilled");
  //   });
  // });

  // describe("-- ESCROW Rescue --", () => {
  //   it("Should rescue after timelock expires", async () => {
  //     const orderHash = generateOrderId(
  //       CHAIN_ID,
  //       alice.address,
  //       bob.address,
  //       TIMELOCK,
  //       AMOUNT,
  //       secretHash7
  //     );

  //     const input = {
  //       token: STARK,
  //       order_hash: orderHash,
  //       resolver_address: alice.address,
  //       user_address: bob.address,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash7,
  //     };

  //     await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_inbound_order",
  //       calldata: callData.compile("create_inbound_order", input),
  //     });

  //     // Wait for timelock to expire (in real test, you'd advance time)
  //     // For now, we'll assume time has passed
      
  //     const aliceBalanceBefore = await stark.balanceOf(alice.address);

  //     await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "rescue",
  //       calldata: {
  //         token: STARK,
  //         order_hash: orderHash,
  //       },
  //     });

  //     const aliceBalanceAfter = await stark.balanceOf(alice.address);
  //     expect(aliceBalanceAfter).toBe(aliceBalanceBefore + AMOUNT);

  //     // Verify order is fulfilled
  //         const order = await starknetESCROW.get_order(STARK, orderHash);
  //     expect(order.is_fulfilled).toBe(true);
  //   });

  //   it("Should not rescue before timelock expires", async () => {
  //     const orderHash = generateOrderId(
  //       CHAIN_ID,
  //       alice.address,
  //       bob.address,
  //       TIMELOCK,
  //       AMOUNT,
  //       secretHash1
  //     );

  //     const input = {
  //       token: STARK,
  //       order_hash: orderHash,
  //       resolver_address: alice.address,
  //       user_address: bob.address,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash1,
  //     };

  //     await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_inbound_order",
  //       calldata: callData.compile("create_inbound_order", input),
  //     });

  //     // Try to rescue immediately (before timelock expires)
  //     await expect(
  //       alice.execute({
  //         contractAddress: starknetESCROW.address,
  //         entrypoint: "rescue",
  //         calldata: {
  //           token: STARK,
  //           order_hash: orderHash,
  //         },
  //       })
  //     ).rejects.toThrow("ESCROW: order not expired");
  //   });
  // });

  // describe("-- ESCROW Get Order --", () => {
  //   it("Should return correct order information", async () => {
  //     const orderHash = generateOrderId(
  //       CHAIN_ID,
  //       alice.address,
  //       bob.address,
  //       TIMELOCK,
  //       AMOUNT,
  //       secretHash2
  //     );

  //     const input = {
  //       token: STARK,
  //       order_hash: orderHash,
  //       resolver_address: alice.address,
  //       user_address: bob.address,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash2,
  //     };

  //     await alice.execute({
  //       contractAddress: starknetESCROW.address,
  //       entrypoint: "create_inbound_order",
  //       calldata: callData.compile("create_inbound_order", input),
  //     });

  //     const order = await starknetESCROW.get_order(STARK, orderHash);

  //     expect(order).toBeTruthy();
  //     expect(order.is_fulfilled).toBe(false);
  //     expect(order.initiator).toBe(BigInt(alice.address));
  //     expect(order.redeemer).toBe(BigInt(bob.address));
  //     expect(typeof order.initiated_at).toBe("bigint");
  //     expect(order.timelock).toBe(TIMELOCK);
  //     expect(order.amount).toBe(AMOUNT);
  //   });
  // });

  // it("Should create outbound order with proper parameters", async () => {
  //   // Generate a random order hash
  //   const orderHash = generateOrderId();
    
  //   // Generate a random secret and its hash
  //   const secret = sha256(randomBytes(32));
  //   const secretHash = hexToU32Array(sha256(secret));

  //   console.log("Secret: ", secret);
  //   console.log("Secret Hash: ", secretHash);
    
  //   // Create UserIntent for signature verification
  //   const userIntent = {
  //     salt: cairo.uint256(123456),
  //     maker: alice.address,
  //     receiver: charlie.address,
  //     maker_asset: STARK,
  //     taker_asset: ZERO_ADDRESS,
  //     making_amount: cairo.uint256(AMOUNT),
  //     taking_amount: cairo.uint256(AMOUNT),
  //   };

  //   // Define the UserIntent type structure for signing
  //   const USER_INTENT_TYPE = {
  //     StarknetDomain: [
  //       { name: "name", type: "shortstring" },
  //       { name: "version", type: "shortstring" },
  //       { name: "chainId", type: "shortstring" },
  //       { name: "revision", type: "shortstring" },
  //     ],
  //     UserIntent: [
  //       { name: "salt", type: "u256" },
  //       { name: "maker", type: "ContractAddress" },
  //       { name: "receiver", type: "ContractAddress" },
  //       { name: "maker_asset", type: "ContractAddress" },
  //       { name: "taker_asset", type: "ContractAddress" },
  //       { name: "making_amount", type: "u256" },
  //       { name: "taking_amount", type: "u256" },
  //     ],
  //   };

  //   const DOMAIN = {
  //     name: "ESCROW",
  //     version: shortString.encodeShortString("1"),
  //     chainId: CHAIN_ID,
  //     revision: TypedDataRevision.ACTIVE,
  //   };

  //   // Create typed data for signing
  //   const typedData: TypedData = {
  //     domain: DOMAIN,
  //     primaryType: "UserIntent",
  //     types: USER_INTENT_TYPE,
  //     message: userIntent,
  //   };

  //   // Sign with alice
  //   const signature = (await alice.signMessage(typedData)) as WeierstrassSignatureType;
  //   const { r, s } = signature;
  //   const signatureArray = [r, s];

  //   // Create the OutboundOrderInput structure
  //   const outboundOrderInput = {
  //     user_intent: userIntent,
  //     signature: signatureArray,
  //     token: STARK,
  //     order_hash: orderHash,
  //     user_address: alice.address,      // who signs (initiator)
  //     resolver_address: charlie.address,    // who executes (redeemer)
  //     timelock: TIMELOCK,
  //     secret_hash: secretHash,
  //     amount: cairo.uint256(AMOUNT)
  //   };

  //   console.log("Creating outbound order with parameters:", {
  //     orderHash,
  //     userAddress: alice.address,
  //     resolverAddress: bob.address,
  //     token: STARK,
  //     amount: AMOUNT.toString(),
  //     timelock: TIMELOCK.toString()
  //   });

  //   // Call create_outbound_order using the typed contract
  //   try {
  //     starknetESCROW.connect(alice);
  //     const createOutboundOrderRes = await starknetESCROW.create_outbound_order(outboundOrderInput);
  //     console.log("Create outbound order successful:", createOutboundOrderRes);
      
  //     // Verify the order was created by getting it
  //     const order = await starknetESCROW.get_order(STARK, orderHash);
  //     console.log("Order details:", order);
      
  //   } catch (error) {
  //     console.error("Error creating outbound order:", error);
  //     throw error;
  //   }
  // });

  it("Should withdraw order using secret", async () => {
    // Use the same order hash and secret from the previous test
    const orderHash = "0x25c6545ab56f2"; // From the previous test output
    const secret = "0x91807027dabc40c15ec2416fa710379d6518948e78a680cef6c474f07a27b8a9"; // From the previous test output
    
    // Convert secret to u32 array for withdrawal
    const secretBytes = ethers.getBytes(secret);
    const secretArray = [];
    for (let i = 0; i < secretBytes.length; i += 4) {
      const chunk = secretBytes.slice(i, i + 4);
      const value = new DataView(chunk.buffer).getUint32(0, false);
      secretArray.push(value);
    }

    console.log("Withdrawing order with parameters:", {
      orderHash,
      secret,
      secretArray
    });

    // Connect alice to the contract
    starknetESCROW.connect(charlie);

    try {
      // Call withdraw function with the secret
      const withdrawRes = await starknetESCROW.withdraw(STARK, orderHash, secretArray);
      console.log("Withdraw successful:", withdrawRes);
      
    } catch (error) {
      console.error("Error withdrawing order:", error);
      throw error;
    }
  });


}); 