import {
  Account,
  cairo,
  CallData,
  Contract,
  RpcProvider,
  stark as sn,
} from "starknet";
import path from "path";
import { readFile } from "fs/promises";
import { 
  STARKNET_DEVNET_URL, 
  accounts
} from "./config";

const RESOLVER_PRIVATE_KEY = "0x014b647de5269b2e0069f3c1ef93c1c8e64ae8d842181df21afb0b32b3db081a";
const RESOLVER_ADDRESS = "0x07c3ddf1d8b12ca535493becae82782e537884172a20ffc239b9c859e0280052";
export const RPC_URL = "https://starknet-sepolia.public.blastapi.io/rpc/v0_8";

async function main() {
  console.log("🚀 Starting deployment script...");

  // Initialize provider
  const starknetProvider = new RpcProvider({
    nodeUrl: RPC_URL,
  });

  // Initialize deployer account (using first account from config)
  const deployer = new Account(
    starknetProvider,
    RESOLVER_ADDRESS,
    RESOLVER_PRIVATE_KEY,
    "1",
    "0x3"
  );

  console.log("📋 Deployer address:", deployer.address);

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

  try {
    // Step 1: Deploy ESCROW Contract
    console.log("\n📦 Step 1: Deploying ESCROW contract...");
    
    const { sierraCode: escrowSierra, casmCode: escrowCasm } = await getCompiledCode("escrow_ESCROW");
    
    const escrowCallData = new CallData(escrowSierra.abi);
    const escrowConstructor = escrowCallData.compile("constructor", {});

    const escrowDeployResponse = await deployer.declareAndDeploy({
      contract: escrowSierra,
      casm: escrowCasm,
      constructorCalldata: escrowConstructor,
      salt: sn.randomAddress(),
    });

    const escrowAddress = escrowDeployResponse.deploy.contract_address;
    console.log("✅ ESCROW contract deployed successfully!");
    console.log("📍 ESCROW Address:", escrowAddress);

    // Step 2: Deploy Resolver Contract
    console.log("\n📦 Step 2: Deploying Resolver contract...");
    
    const { sierraCode: resolverSierra, casmCode: resolverCasm } = await getCompiledCode("escrow_Resolver");
    
    const resolverCallData = new CallData(resolverSierra.abi);
    const resolverConstructor = resolverCallData.compile("constructor", {
      escrow_contract: escrowAddress,
    });

    const resolverDeployResponse = await deployer.declareAndDeploy({
      contract: resolverSierra,
      casm: resolverCasm,
      constructorCalldata: resolverConstructor,
      salt: sn.randomAddress(),
    });

    const resolverAddress = resolverDeployResponse.deploy.contract_address;
    console.log("✅ Resolver contract deployed successfully!");
    console.log("📍 Resolver Address:", resolverAddress);

    // Step 3: Log deployment summary
    console.log("\n📊 Deployment Summary:");
    console.log("========================");
    console.log("🔐 ESCROW Contract:", escrowAddress);
    console.log("🔧 Resolver Contract:", resolverAddress);
    console.log("👤 Deployer:", deployer.address);
    console.log("🔑 Resolver PK (placeholder):", RESOLVER_PRIVATE_KEY);
    console.log("📍 Resolver Address (placeholder):", RESOLVER_ADDRESS);
    console.log("🌐 Network:", STARKNET_DEVNET_URL);
    console.log("========================");
    console.log("✅ Deployment completed successfully!");

  } catch (error) {
    console.error("❌ Deployment failed:", error);
    process.exit(1);
  }
}

// Run the deployment script
main().catch((error) => {
  console.error("❌ Script execution failed:", error);
  process.exit(1);
});
