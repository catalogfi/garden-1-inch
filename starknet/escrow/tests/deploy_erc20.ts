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

async function main() {
  console.log("🚀 Starting ERC20 deployment script...");

  // Initialize provider
  const starknetProvider = new RpcProvider({
    nodeUrl: STARKNET_DEVNET_URL,
  });

  // Initialize deployer account (using Alice from config)
  const alice = new Account(
    starknetProvider,
    accounts[0].address,
    accounts[0].privateKey,
    "1",
    "0x3"
  );

  console.log("📋 Deployer (Alice) address:", alice.address);

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
    // Step 1: Deploy ERC20 Token
    console.log("\n📦 Step 1: Deploying ERC20 token...");
    
    // Try to get ERC20 contract files - you may need to adjust the contract name
    // based on your actual ERC20 contract compilation
    let erc20ContractName = "escrow_USDC";
    
    try {
      const { sierraCode: erc20Sierra, casmCode: erc20Casm } = await getCompiledCode(erc20ContractName);
      
      const erc20CallData = new CallData(erc20Sierra.abi);
      
      // USDC constructor parameters
      const initialSupply = cairo.uint256("1000000000000"); // 1,000,000 USDC (6 decimals)
      const recipient = alice.address;
      
      const erc20Constructor = erc20CallData.compile("constructor", {
        fixed_supply: initialSupply,
        recipient: recipient,
      });

      const erc20DeployResponse = await alice.declareAndDeploy({
        contract: erc20Sierra,
        casm: erc20Casm,
        constructorCalldata: erc20Constructor,
        salt: sn.randomAddress(),
      });

      const erc20Address = erc20DeployResponse.deploy.contract_address;
      console.log("✅ USDC token deployed successfully!");
      console.log("📍 USDC Address:", erc20Address);
      console.log("📝 Token Name: USDC");
      console.log("🔤 Token Symbol: USDC");
      console.log("🔢 Decimals: 6");
      console.log("💰 Initial Supply: 1,000,000 USDC");

      // Step 2: Log deployment summary
      console.log("\n📊 Deployment Summary:");
      console.log("========================");
      console.log("🪙 USDC Token:", erc20Address);
      console.log("👤 Deployer:", alice.address);
      console.log("👤 Recipient:", alice.address);
      console.log("📝 Token Name: USDC");
      console.log("🔤 Token Symbol: USDC");
      console.log("🔢 Decimals: 6");
      console.log("💰 Initial Supply: 1,000,000 USDC");
      console.log("🌐 Network:", STARKNET_DEVNET_URL);
      console.log("========================");
      console.log("✅ ERC20 deployment completed successfully!");

    } catch (error) {
      console.log("❌ Failed to deploy ERC20 - contract files not found or compilation error");
      console.log("💡 Make sure you have compiled an ERC20 contract in your project");
      console.log("📁 Expected contract name:", erc20ContractName);
      console.log("🔧 Error details:", error);
    }

  } catch (error) {
    console.error("Deployment failed:", error);
    process.exit(1);
  }
}

// Run the deployment script
main().catch((error) => {
  console.error("Script execution failed:", error);
  process.exit(1);
});
