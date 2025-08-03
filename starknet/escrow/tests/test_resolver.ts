import {
  Account,
  cairo,
  CallData,
  Contract,
  RpcProvider,
  stark as sn,
  TypedData,
  TypedDataRevision,
  WeierstrassSignatureType
} from "starknet";
import { ethers, parseEther, sha256 } from "ethers";
import { randomBytes } from "crypto";
import path from "path";
import { readFile } from "fs/promises";
import { shortString } from "starknet";
import { 
  STARKNET_DEVNET_URL, 
  accounts, 
  STARK,
  ZERO_ADDRESS,
  AMOUNT,
  TIMELOCK
} from "./config";

describe("Starknet Resolver", () => {
  const starknetProvider = new RpcProvider({
    nodeUrl: STARKNET_DEVNET_URL,
  });

  const AMOUNT_BIGINT = 1000000n; // 1 USDC (6 decimals)

  let usdc: Contract;
  let starknetESCROW: Contract;
  let starknetResolver: Contract;
  let callData: CallData;

  let alice: Account;
  let resolver: Account;

  let secret1: string;
  let secret2: string;
  let secret3: string;

  let secretHash1: number[];
  let secretHash2: number[];
  let secretHash3: number[];

  const RESOLVER_PRIVATE_KEY = "0x014b647de5269b2e0069f3c1ef93c1c8e64ae8d842181df21afb0b32b3db081a";
  const RESOLVER_ADDRESS = "0x07c3ddf1d8b12ca535493becae82782e537884172a20ffc239b9c859e0280052";

  const USDC = "0x02fb897ed33fbd7f3b68bb51b3a1f1e94255d71c327c4447ec4db462848752bd";

  const AMOUNT_USDC = 10000000;

  let CHAIN_ID: string;

  let sierraCode: any, casmCode: any;

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

  async function getCompiledCode(filename: string) {
    const sierraFilePath = path.join(
      __dirname,
      `../target/dev/${filename}.contract_class.json`
    );
    const casmFilePath = path.join(
      __dirname,
      `../target/dev/${filename}.compiled_contract_class.json`
    );
  
    const [sierraCode, casmCode] = await Promise.all([
      readFile(sierraFilePath, "utf8").then(JSON.parse),
      readFile(casmFilePath, "utf8").then(JSON.parse)
    ]);
  
    return {
      sierraCode,
      casmCode,
    };
  }

  const deployContracts = async () => {
    try {
      ({ sierraCode, casmCode } = await getCompiledCode("escrow_ESCROW"));
    } catch (error: any) {
      console.log("Failed to read contract files", error);
      process.exit(1);
    }
    callData = new CallData(sierraCode.abi);

    // // Deploy ESCROW
    // const escrowConstructor = callData.compile("constructor", {});
    // const escrowDeployResponse = await alice.declareAndDeploy({
    //   contract: sierraCode,
    //   casm: casmCode,
    //   constructorCalldata: escrowConstructor,
    //   salt: sn.randomAddress(),
    // });

    // const ESCROW_ADDRESS = escrowDeployResponse.deploy.contract_address;
    const ESCROW_ADDRESS = "0x22f3e385ce47e1c1054b8b1ea3fc72f1d46763401f912652adf7438285f764c";

    starknetESCROW = new Contract(
      sierraCode.abi,
      ESCROW_ADDRESS,
      starknetProvider
    );

    console.log("ESCROW contract deployed :", ESCROW_ADDRESS);

    try {
      ({ sierraCode, casmCode } = await getCompiledCode("escrow_Resolver"));
    } catch (error: any) {
      console.log("Failed to read contract files", error);
      process.exit(1);
    }
    callData = new CallData(sierraCode.abi);

    // // Deploy Resolver
    // const resolverConstructor = callData.compile("constructor", {
    //   escrow_contract: escrowDeployResponse.deploy.contract_address,
    // });
    // const resolverDeployResponse = await resolver.declareAndDeploy({
    //   contract: sierraCode,
    //   casm: casmCode,
    //   constructorCalldata: resolverConstructor,
    //   salt: sn.randomAddress(),
    // });
    // const RESOLVER_CONTRACT_ADDRESS = resolverDeployResponse.deploy.contract_address;
    const RESOLVER_CONTRACT_ADDRESS = "0x4688ecf254dfa78275085ed99f1565bc72832c3ec92fe0e4d733e3978b007f4";

    starknetResolver = new Contract(
      sierraCode.abi,
      RESOLVER_CONTRACT_ADDRESS,
      starknetProvider
    );

    console.log("Resolver contract deployed :", RESOLVER_CONTRACT_ADDRESS);
  };

  beforeAll(async () => {
    secret1 = sha256(randomBytes(32));
    secret2 = sha256(randomBytes(32));
    secret3 = sha256(randomBytes(32));

    secretHash1 = hexToU32Array(sha256(secret1));
    secretHash2 = hexToU32Array(sha256(secret2));
    secretHash3 = hexToU32Array(sha256(secret3));

    CHAIN_ID = (await starknetProvider.getChainId()).toString();

    alice = new Account(
      starknetProvider,
      accounts[0].address,
      accounts[0].privateKey,
      "1",
      "0x3"
    );

    resolver = new Account(
      starknetProvider,
      RESOLVER_ADDRESS,
      RESOLVER_PRIVATE_KEY,
      "1",
      "0x3"
    );

    const contractData = await starknetProvider.getClassAt(USDC);
    usdc = new Contract(contractData.abi, USDC, starknetProvider);
    await deployContracts();

    console.log("USDC contract:", usdc.address);
    // allowance for ESCROW
    usdc.connect(alice);
    await usdc.approve(starknetESCROW.address, (AMOUNT_USDC * 10));
    console.log("Alice approved USDC for ESCROW");

    const aliceBalance = await usdc.balanceOf(alice.address);
    console.log("Alice's USDC balance:", aliceBalance);

    const aliceAllowance = await usdc.allowance(alice.address, "0x2a70495e904d030fcd6e8273c79af2177060f0afc192862feb56e66702c8aef");
    console.log("Alice's USDC allowance on ESCROW:", aliceAllowance);

    await new Promise(resolve => setTimeout(resolve, 5000));

    // // Transfer USDC from Alice to resolver
    // usdc.connect(alice);
    // await usdc.transfer(resolver.address, AMOUNT_USDC);
    // console.log("Alice transferred USDC to resolver");

    // const resolverBalance = await usdc.balanceOf(resolver.address);
    // console.log("Resolver's USDC balance:", resolverBalance);

    // // Resolver approves USDC for ESCROW
    // usdc.connect(resolver);
    // await usdc.approve(starknetESCROW.address, AMOUNT_USDC);
    // console.log("Resolver approved USDC for ESCROW");
  }, 100000);

  describe("-- Resolver Create Source --", () => {
    it("Should create source order with valid signature", async () => {
      const USER_INTENT_TYPE = {
        StarknetDomain: [
          { name: "name", type: "shortstring" },
          { name: "version", type: "shortstring" },
          { name: "chainId", type: "shortstring" },
          { name: "revision", type: "shortstring" },
        ],
        UserIntent: [
          { name: "salt", type: "u256" },
          { name: "maker", type: "ContractAddress" },
          { name: "receiver", type: "ContractAddress" },
          { name: "maker_asset", type: "ContractAddress" },
          { name: "taker_asset", type: "ContractAddress" },
          { name: "making_amount", type: "u256" },
          { name: "taking_amount", type: "u256" },
        ],
      };

      const DOMAIN = {
        name: "ESCROW",
        version: shortString.encodeShortString("1"),
        chainId: CHAIN_ID,
        revision: TypedDataRevision.ACTIVE,
      };

      const userIntent = {
        salt: cairo.uint256(123456),
        maker: alice.address,
        receiver: resolver.address,
        maker_asset: USDC,
        taker_asset: STARK,
        making_amount: cairo.uint256(AMOUNT_USDC),
        taking_amount: cairo.uint256(AMOUNT_USDC),
      };

      // Create typed data for signing
      const typedData: TypedData = {
        domain: DOMAIN,
        primaryType: "UserIntent",
        types: USER_INTENT_TYPE,
        message: userIntent,
      };

      // Sign with alice
      const signature = (await alice.signMessage(typedData)) as WeierstrassSignatureType;
      const { r, s } = signature;
      const signatureArray = [r, s];

      const orderHash = generateOrderId();

      console.log("Amount", AMOUNT_USDC);
      const input = {
        user_address: alice.address,
        resolver_address: resolver.address,
        user_intent: userIntent,
        signature: signatureArray,
        order_hash: orderHash,
        timelock: TIMELOCK,
        secret_hash: secretHash1,
        amount: cairo.uint256(AMOUNT_USDC)
      };

      let res = await resolver.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "create_source",
        calldata: callData.compile("create_source", input),
      });
      console.log("res", res);
    });
  });

  // describe("-- Resolver Create Destination --", () => {
  //   it("Should create destination order", async () => {
  //     const orderHash = generateOrderId();

  //     const input = {
  //       user_address: alice.address,
  //       resolver_address: resolver.address,
  //       order_hash: orderHash,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash1,
  //       token: STARK,
  //     };

  //     await resolver.execute({
  //       contractAddress: starknetResolver.address,
  //       entrypoint: "create_destination",
  //       calldata: callData.compile("create_destination", input),
  //     });

  //     // Verify the function completed without error
  //     // In a real test, we would verify the ESCROW contract was called correctly
  //   });

  //   it("Should not create destination order with caller mismatch", async () => {
  //     const orderHash = generateOrderId();

  //     const input = {
  //       user_address: alice.address,
  //       resolver_address: alice.address, // Different from caller (resolver.address)
  //       order_hash: orderHash,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash2,
  //       token: STARK,
  //     };

  //     await expect(
  //       resolver.execute({
  //         contractAddress: starknetResolver.address,
  //         entrypoint: "create_destination",
  //         calldata: callData.compile("create_destination", input),
  //       })
  //     ).rejects.toThrow("Resolver: caller mismatch");
  //   });
  // });

  // describe("-- Resolver Withdraw --", () => {
  //   it("Should withdraw with correct owner", async () => {
  //     const orderHash = generateOrderId();
  //     const secret = hexToU32Array(secret3);

  //     await alice.execute({
  //       contractAddress: starknetResolver.address,
  //       entrypoint: "withdraw",
  //       calldata: callData.compile("withdraw", {
  //         token: STARK,
  //         order_hash: orderHash,
  //         secret: secret,
  //       }),
  //     });

  //     // Verify the function completed without error
  //     // In a real test, we would verify the ESCROW contract was called correctly
  //   });

  //   it("Should not withdraw with non-owner", async () => {
  //     const orderHash = generateOrderId();
  //     const secret = hexToU32Array(secret3);

  //     await expect(
  //       resolver.execute({
  //         contractAddress: starknetResolver.address,
  //         entrypoint: "withdraw",
  //         calldata: callData.compile("withdraw", {
  //           token: STARK,
  //           order_hash: orderHash,
  //           secret: secret,
  //         }),
  //       })
  //     ).rejects.toThrow("Resolver: caller mismatch");
  //   });
  // });

  // describe("-- Resolver Integration with ESCROW --", () => {
  //   it("Should create destination order and verify ESCROW integration", async () => {
  //     const orderHash = generateOrderId();

  //     const input = {
  //       user_address: alice.address,
  //       resolver_address: resolver.address,
  //       order_hash: orderHash,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash1,
  //       token: STARK,
  //     };

  //     await resolver.execute({
  //       contractAddress: starknetResolver.address,
  //       entrypoint: "create_destination",
  //       calldata: callData.compile("create_destination", input),
  //     });

  //     // Verify order was created in ESCROW contract
  //     const order = await starknetESCROW.get_order(STARK, orderHash);
  //     expect(order.initiator).toBe(BigInt(resolver.address));
  //     expect(order.redeemer).toBe(BigInt(alice.address));
  //     expect(order.amount).toBe(AMOUNT);
  //     expect(order.timelock).toBe(TIMELOCK);
  //     expect(order.is_fulfilled).toBe(false);
  //   });

  //   it("Should create source order and verify ESCROW integration", async () => {
  //     const userIntent = {
  //       salt: cairo.uint256(123456),
  //       maker: alice.address,
  //       receiver: resolver.address,
  //       maker_asset: STARK,
  //       taker_asset: ZERO_ADDRESS,
  //       making_amount: cairo.uint256(AMOUNT),
  //       taking_amount: cairo.uint256(AMOUNT),
  //     };

  //     const orderHash = generateOrderId();

  //     const input = {
  //       user_address: alice.address,
  //       user_intent: userIntent,
  //       signature: [123456, 789012], // Mock signature
  //       resolver_address: resolver.address,
  //       order_hash: orderHash,
  //       timelock: TIMELOCK,
  //       amount: cairo.uint256(AMOUNT),
  //       secret_hash: secretHash2,
  //     };

  //     await resolver.execute({
  //       contractAddress: starknetResolver.address,
  //       entrypoint: "create_source",
  //       calldata: callData.compile("create_source", input),
  //     });

  //     // Verify order was created in ESCROW contract
  //     const order = await starknetESCROW.get_order(STARK, orderHash);
  //     expect(order.initiator).toBe(BigInt(alice.address));
  //     expect(order.redeemer).toBe(BigInt(resolver.address));
  //     expect(order.amount).toBe(AMOUNT);
  //     expect(order.timelock).toBe(TIMELOCK);
  //     expect(order.is_fulfilled).toBe(false);
  //   });
  // });
}); 