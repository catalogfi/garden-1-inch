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
    const ESCROW_ADDRESS = "0x29f6c83325473668b0eb0444db24ee8fbee7cf784a511e388bfffc753f3ad95";

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
    //   escrow_contract: ESCROW_ADDRESS,
    // });
    // const resolverDeployResponse = await resolver.declareAndDeploy({
    //   contract: sierraCode,
    //   casm: casmCode,
    //   constructorCalldata: resolverConstructor,
    //   salt: sn.randomAddress(),
    // });
    // const RESOLVER_CONTRACT_ADDRESS = resolverDeployResponse.deploy.contract_address;
    const RESOLVER_CONTRACT_ADDRESS = "0x6b96700855961261698513b949b53a5ee4162efcbbf7a6eb6a2382d89989433";

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

    usdc.connect(resolver);
    const approveTx = await usdc.approve(starknetESCROW.address, AMOUNT_USDC);
    console.log("Resolver approved USDC for ESCROW, tx hash:", approveTx.transaction_hash);
    
    // Wait for the approval transaction to be processed
    await starknetProvider.waitForTransaction(approveTx.transaction_hash);
    console.log("Resolver approval transaction confirmed");

    await new Promise(resolve => setTimeout(resolve, 5000));

    // Transfer USDC from Alice to resolver
    usdc.connect(alice);
    const transferTx = await usdc.transfer(resolver.address, AMOUNT_USDC);
    console.log("Alice transferred USDC to resolver, tx hash:", transferTx.transaction_hash);
    
    // Wait for the transfer transaction to be processed
    await starknetProvider.waitForTransaction(transferTx.transaction_hash);
    console.log("USDC transfer transaction confirmed");

    const resolverBalance = await usdc.balanceOf(resolver.address);
    console.log("Resolver's USDC balance:", resolverBalance);
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

  describe("-- Resolver Create Destination --", () => {
    it("Should create destination order without signature", async () => {
      const orderHash = generateOrderId();

      console.log("Order hash:", orderHash);
      console.log("Secret hash:", secretHash1);
      console.log("Secret:", secret1);

      console.log("Creating destination order with amount:", AMOUNT_USDC);
      const input = {
        user_address: alice.address,
        resolver_address: resolver.address,
        order_hash: orderHash,
        timelock: TIMELOCK,
        token: USDC,
        secret_hash: secretHash1,
        amount: cairo.uint256(AMOUNT_USDC)
      };

      let res = await resolver.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "create_destination",
        calldata: callData.compile("create_destination", input),
      });
      console.log("Destination order creation result:", res);
    });
  });

  describe("-- Resolver Withdraw --", () => {
    it("Should withdraw USDC with valid secret", async () => {
      const orderHash = "0x18a8edf448cfa8";
      const secret = hexToU32Array("0xb4b641127d9574551ebd2ccaf8625829364fff0c44da9b78194060db00f3d68f");
      
      console.log("Withdrawing USDC with order hash:", orderHash);
      console.log("Secret:", secret);
      
      const input = {
        token: USDC,
        order_hash: orderHash,
        secret: secret
      };

      console.log("Alice address:", alice.address);

      let res = await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "withdraw",
        calldata: callData.compile("withdraw", input),
      });

      console.log("Withdraw result:", res);
    });
  });

}); 