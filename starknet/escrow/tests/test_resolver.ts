import {
  Account,
  cairo,
  CallData,
  Contract,
  RpcProvider,
  stark as sn
} from "starknet";
import { ethers, parseEther, sha256 } from "ethers";
import { randomBytes } from "crypto";
import path from "path";
import { readFile } from "fs/promises";
import { 
  STARKNET_DEVNET_URL, 
  accounts, 
  STARK, 
  ETH, 
  ZERO_ADDRESS, 
  TIMELOCK, 
  AMOUNT 
} from "./config";

describe("Starknet Resolver", () => {
  const starknetProvider = new RpcProvider({
    nodeUrl: STARKNET_DEVNET_URL,
  });

  const AMOUNT_BIGINT = parseEther("0.1");

  let stark: Contract;
  let starknetESCROW: Contract;
  let starknetResolver: Contract;
  let callData: CallData;

  let alice: Account;
  let bob: Account;
  let charlie: Account;
  let resolver: Account;

  let secret1: string;
  let secret2: string;
  let secret3: string;

  let secretHash1: number[];
  let secretHash2: number[];
  let secretHash3: number[];

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
  const generateOrderId = (
    chainId: string,
    initiator: string,
    redeemer: string,
    timelock: bigint,
    amount: bigint,
    secretHash: number[]
  ): string => {
    return "0x" + randomBytes(32).toString("hex");
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
      ({ sierraCode, casmCode } = await getCompiledCode("starknet_escrow_ESCROW"));
    } catch (error: any) {
      console.log("Failed to read contract files", error);
      process.exit(1);
    }
    callData = new CallData(sierraCode.abi);

    // Deploy ESCROW
    const escrowConstructor = callData.compile("constructor", {});
    const escrowDeployResponse = await alice.declareAndDeploy({
      contract: sierraCode,
      casm: casmCode,
      constructorCalldata: escrowConstructor,
      salt: sn.randomAddress(),
    });

    starknetESCROW = new Contract(
      sierraCode.abi,
      escrowDeployResponse.deploy.contract_address,
      starknetProvider
    );

    // Deploy Resolver
    const resolverConstructor = callData.compile("constructor", {
      escrow_contract: escrowDeployResponse.deploy.contract_address,
    });
    const resolverDeployResponse = await alice.declareAndDeploy({
      contract: sierraCode,
      casm: casmCode,
      constructorCalldata: resolverConstructor,
      salt: sn.randomAddress(),
    });

    starknetResolver = new Contract(
      sierraCode.abi,
      resolverDeployResponse.deploy.contract_address,
      starknetProvider
    );
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
      accounts[0].privateKey
    );

    bob = new Account(
      starknetProvider,
      accounts[1].address,
      accounts[1].privateKey
    );
    charlie = new Account(
      starknetProvider,
      accounts[2].address,
      accounts[2].privateKey
    );

    resolver = new Account(
      starknetProvider,
      accounts[0].address, // Using alice as resolver for testing
      accounts[0].privateKey
    );

    const contractData = await starknetProvider.getClassAt(STARK);
    stark = new Contract(contractData.abi, STARK, starknetProvider);
    await deployContracts();

    // allowance for ESCROW
    stark.connect(alice);
    await stark.approve(starknetESCROW.address, parseEther("500"));
    stark.connect(bob);
    await stark.approve(starknetESCROW.address, parseEther("500"));
    stark.connect(charlie);
    await stark.approve(starknetESCROW.address, parseEther("500"));
  }, 100000);

  describe("- Pre-Conditions -", () => {
    it("Resolver should be deployed with correct ESCROW address", async () => {
      const escrowAddress = await starknetResolver.escrow_contract();
      expect(escrowAddress).toBe(starknetESCROW.address);
    });

    it("Resolver should have correct owner", async () => {
      const owner = await starknetResolver.owner();
      expect(owner).toBe(alice.address);
    });
  });

  describe("-- Resolver Authorization --", () => {
    it("Should authorize resolver", async () => {
      await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "authorize_resolver",
        calldata: callData.compile("authorize_resolver", {
          resolver_address: resolver.address,
          is_authorized: true,
        }),
      });

      const isAuthorized = await starknetResolver.is_resolver_authorized(resolver.address);
      expect(isAuthorized).toBe(true);
    });

    it("Should deauthorize resolver", async () => {
      await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "authorize_resolver",
        calldata: callData.compile("authorize_resolver", {
          resolver_address: resolver.address,
          is_authorized: false,
        }),
      });

      const isAuthorized = await starknetResolver.is_resolver_authorized(resolver.address);
      expect(isAuthorized).toBe(false);
    });

    it("Should not allow non-owner to authorize resolver", async () => {
      await expect(
        bob.execute({
          contractAddress: starknetResolver.address,
          entrypoint: "authorize_resolver",
          calldata: callData.compile("authorize_resolver", {
            resolver_address: resolver.address,
            is_authorized: true,
          }),
        })
      ).rejects.toThrow("Resolver: only owner");
    });
  });

  describe("-- Resolver Create Source --", () => {
    beforeEach(async () => {
      // Authorize resolver before each test
      await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "authorize_resolver",
        calldata: callData.compile("authorize_resolver", {
          resolver_address: resolver.address,
          is_authorized: true,
        }),
      });
    });

    it("Should create source order with valid signature", async () => {
      const userIntent = {
        salt: cairo.uint256(123456),
        maker: alice.address,
        receiver: resolver.address,
        maker_asset: STARK,
        taker_asset: ZERO_ADDRESS,
        making_amount: cairo.uint256(AMOUNT),
        taking_amount: cairo.uint256(AMOUNT),
      };

      const orderHash = generateOrderId(
        CHAIN_ID,
        alice.address,
        resolver.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash1
      );

      const input = {
        user_address: alice.address,
        user_intent: userIntent,
        signature: [123456, 789012], // Mock signature
        resolver_address: resolver.address,
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash1,
      };

      await resolver.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "create_source",
        calldata: callData.compile("create_source", input),
      });

      // Verify the function completed without error
      // In a real test, we would verify the ESCROW contract was called correctly
    });

    it("Should not create source order with unauthorized resolver", async () => {
      // Deauthorize resolver
      await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "authorize_resolver",
        calldata: callData.compile("authorize_resolver", {
          resolver_address: resolver.address,
          is_authorized: false,
        }),
      });

      const userIntent = {
        salt: cairo.uint256(123456),
        maker: alice.address,
        receiver: resolver.address,
        maker_asset: STARK,
        taker_asset: ZERO_ADDRESS,
        making_amount: cairo.uint256(AMOUNT),
        taking_amount: cairo.uint256(AMOUNT),
      };

      const orderHash = generateOrderId(
        CHAIN_ID,
        alice.address,
        resolver.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash2
      );

      const input = {
        user_address: alice.address,
        user_intent: userIntent,
        signature: [123456, 789012],
        resolver_address: resolver.address,
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash2,
      };

      await expect(
        resolver.execute({
          contractAddress: starknetResolver.address,
          entrypoint: "create_source",
          calldata: callData.compile("create_source", input),
        })
      ).rejects.toThrow("Resolver: not authorized");
    });

    it("Should not create source order with caller mismatch", async () => {
      const userIntent = {
        salt: cairo.uint256(123456),
        maker: alice.address,
        receiver: resolver.address,
        maker_asset: STARK,
        taker_asset: ZERO_ADDRESS,
        making_amount: cairo.uint256(AMOUNT),
        taking_amount: cairo.uint256(AMOUNT),
      };

      const orderHash = generateOrderId(
        CHAIN_ID,
        alice.address,
        resolver.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash3
      );

      const input = {
        user_address: alice.address,
        user_intent: userIntent,
        signature: [123456, 789012],
        resolver_address: bob.address, // Different from caller
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash3,
      };

      await expect(
        resolver.execute({
          contractAddress: starknetResolver.address,
          entrypoint: "create_source",
          calldata: callData.compile("create_source", input),
        })
      ).rejects.toThrow("Resolver: caller mismatch");
    });
  });

  describe("-- Resolver Create Destination --", () => {
    beforeEach(async () => {
      // Authorize resolver before each test
      await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "authorize_resolver",
        calldata: callData.compile("authorize_resolver", {
          resolver_address: resolver.address,
          is_authorized: true,
        }),
      });
    });

    it("Should create destination order", async () => {
      const orderHash = generateOrderId(
        CHAIN_ID,
        resolver.address,
        alice.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash1
      );

      const input = {
        user_address: alice.address,
        resolver_address: resolver.address,
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash1,
        token: STARK,
      };

      await resolver.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "create_destination",
        calldata: callData.compile("create_destination", input),
      });

      // Verify the function completed without error
      // In a real test, we would verify the ESCROW contract was called correctly
    });

    it("Should not create destination order with unauthorized resolver", async () => {
      // Deauthorize resolver
      await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "authorize_resolver",
        calldata: callData.compile("authorize_resolver", {
          resolver_address: resolver.address,
          is_authorized: false,
        }),
      });

      const orderHash = generateOrderId(
        CHAIN_ID,
        resolver.address,
        alice.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash2
      );

      const input = {
        user_address: alice.address,
        resolver_address: resolver.address,
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash2,
        token: STARK,
      };

      await expect(
        resolver.execute({
          contractAddress: starknetResolver.address,
          entrypoint: "create_destination",
          calldata: callData.compile("create_destination", input),
        })
      ).rejects.toThrow("Resolver: not authorized");
    });

    it("Should not create destination order with caller mismatch", async () => {
      const orderHash = generateOrderId(
        CHAIN_ID,
        resolver.address,
        alice.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash3
      );

      const input = {
        user_address: alice.address,
        resolver_address: bob.address, // Different from caller
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash3,
        token: STARK,
      };

      await expect(
        resolver.execute({
          contractAddress: starknetResolver.address,
          entrypoint: "create_destination",
          calldata: callData.compile("create_destination", input),
        })
      ).rejects.toThrow("Resolver: caller mismatch");
    });
  });

  describe("-- Resolver Integration with ESCROW --", () => {
    beforeEach(async () => {
      // Authorize resolver before each test
      await alice.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "authorize_resolver",
        calldata: callData.compile("authorize_resolver", {
          resolver_address: resolver.address,
          is_authorized: true,
        }),
      });
    });

    it("Should create destination order and verify ESCROW integration", async () => {
      const orderHash = generateOrderId(
        CHAIN_ID,
        resolver.address,
        alice.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash1
      );

      const input = {
        user_address: alice.address,
        resolver_address: resolver.address,
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash1,
        token: STARK,
      };

      await resolver.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "create_destination",
        calldata: callData.compile("create_destination", input),
      });

      // Verify order was created in ESCROW contract
      const order = await starknetESCROW.get_order(STARK, orderHash);
      expect(order.initiator).toBe(BigInt(resolver.address));
      expect(order.redeemer).toBe(BigInt(alice.address));
      expect(order.amount).toBe(AMOUNT);
      expect(order.timelock).toBe(TIMELOCK);
      expect(order.is_fulfilled).toBe(false);
    });

    it("Should create source order and verify ESCROW integration", async () => {
      const userIntent = {
        salt: cairo.uint256(123456),
        maker: alice.address,
        receiver: resolver.address,
        maker_asset: STARK,
        taker_asset: ZERO_ADDRESS,
        making_amount: cairo.uint256(AMOUNT),
        taking_amount: cairo.uint256(AMOUNT),
      };

      const orderHash = generateOrderId(
        CHAIN_ID,
        alice.address,
        resolver.address,
        TIMELOCK,
        AMOUNT_BIGINT,
        secretHash2
      );

      const input = {
        user_address: alice.address,
        user_intent: userIntent,
        signature: [123456, 789012], // Mock signature
        resolver_address: resolver.address,
        order_hash: orderHash,
        timelock: TIMELOCK,
        amount: cairo.uint256(AMOUNT),
        secret_hash: secretHash2,
      };

      await resolver.execute({
        contractAddress: starknetResolver.address,
        entrypoint: "create_source",
        calldata: callData.compile("create_source", input),
      });

      // Verify order was created in ESCROW contract
      const order = await starknetESCROW.get_order(STARK, orderHash);
      expect(order.initiator).toBe(BigInt(alice.address));
      expect(order.redeemer).toBe(BigInt(resolver.address));
      expect(order.amount).toBe(AMOUNT);
      expect(order.timelock).toBe(TIMELOCK);
      expect(order.is_fulfilled).toBe(false);
    });
  });

  describe("-- Resolver Storage Tests --", () => {
    it("Should store and retrieve ESCROW contract address", async () => {
      const escrowAddress = await starknetResolver.escrow_contract();
      expect(escrowAddress).toBe(starknetESCROW.address);
    });

    it("Should store and retrieve chain ID", async () => {
      const chainId = await starknetResolver.chain_id();
      expect(chainId).toBe(CHAIN_ID);
    });

    it("Should store and retrieve owner", async () => {
      const owner = await starknetResolver.owner();
      expect(owner).toBe(alice.address);
    });
  });
}); 