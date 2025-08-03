import {
  Account,
  cairo,
  CallData,
  Contract,
  RpcProvider,
  stark as sn,
  hash,
} from "starknet";
import { ethers } from "ethers";
import * as fs from "fs";
import * as path from "path";

// Configuration
interface DeployConfig {
  network: "devnet" | "testnet" | "mainnet";
  rpcUrl: string;
  accountAddress: string;
  privateKey: string;
  tokenAddress?: string; // Optional token address for ESCROW
}

// Contract artifacts interface
interface ContractArtifacts {
  sierraCode: any;
  casmCode: any;
}

// Deployment result interface
interface DeploymentResult {
  escrowAddress: string;
  resolverAddress: string;
  deployerAddress: string;
  network: string;
  timestamp: number;
}

class EscrowDeployer {
  private provider: RpcProvider;
  private account: Account;
  private config: DeployConfig;

  constructor(config: DeployConfig) {
    this.config = config;
    this.provider = new RpcProvider({ nodeUrl: config.rpcUrl });
    this.account = new Account(
      this.provider,
      config.accountAddress,
      config.privateKey
    );
  }

  /**
   * Load contract artifacts from compiled output
   */
  private async loadContractArtifacts(contractName: string): Promise<ContractArtifacts> {
    const artifactsPath = path.join(__dirname, "../target/dev");
    
    try {
      const sierraPath = path.join(artifactsPath, `${contractName}.contract_class.json`);
      const casmPath = path.join(artifactsPath, `${contractName}.compiled_contract_class.json`);
      
      const sierraCode = JSON.parse(fs.readFileSync(sierraPath, "utf8"));
      const casmCode = JSON.parse(fs.readFileSync(casmPath, "utf8"));
      
      return { sierraCode, casmCode };
    } catch (error) {
      throw new Error(`Failed to load contract artifacts for ${contractName}: ${error}`);
    }
  }

  /**
   * Deploy ESCROW contract
   */
  private async deployESCROW(tokenAddress?: string): Promise<string> {
    console.log("🔧 Deploying ESCROW contract...");
    
    try {
      const { sierraCode, casmCode } = await this.loadContractArtifacts("starknet_escrow_ESCROW");
      const callData = new CallData(sierraCode.abi);
      
      // Prepare constructor calldata
      const constructorCalldata = callData.compile("constructor", {});
      
      // Deploy contract
      const deployResponse = await this.account.declareAndDeploy({
        contract: sierraCode,
        casm: casmCode,
        constructorCalldata,
        salt: sn.randomAddress(),
      });
      
      const escrowAddress = deployResponse.deploy.contract_address;
      console.log(`✅ ESCROW deployed at: ${escrowAddress}`);
      
      return escrowAddress;
    } catch (error) {
      console.error("❌ Failed to deploy ESCROW:", error);
      throw error;
    }
  }

  /**
   * Deploy Resolver contract
   */
  private async deployResolver(escrowAddress: string): Promise<string> {
    console.log("🔧 Deploying Resolver contract...");
    
    try {
      const { sierraCode, casmCode } = await this.loadContractArtifacts("starknet_resolver_ESCROW");
      const callData = new CallData(sierraCode.abi);
      
      // Prepare constructor calldata
      const constructorCalldata = callData.compile("constructor", {
        escrow_contract: escrowAddress,
      });
      
      // Deploy contract
      const deployResponse = await this.account.declareAndDeploy({
        contract: sierraCode,
        casm: casmCode,
        constructorCalldata,
        salt: sn.randomAddress(),
      });
      
      const resolverAddress = deployResponse.deploy.contract_address;
      console.log(`✅ Resolver deployed at: ${resolverAddress}`);
      
      return resolverAddress;
    } catch (error) {
      console.error("❌ Failed to deploy Resolver:", error);
      throw error;
    }
  }

  /**
   * Verify contract deployment
   */
  private async verifyDeployment(escrowAddress: string, resolverAddress: string): Promise<void> {
    console.log("🔍 Verifying deployment...");
    
    try {
      // Verify ESCROW contract
      const escrowContract = new Contract(
        (await this.loadContractArtifacts("starknet_escrow_ESCROW")).sierraCode.abi,
        escrowAddress,
        this.provider
      );
    } catch (error) {
      console.error("❌ Contract verification failed:", error);
      throw error;
    }
  }

  /**
   * Save deployment result to file
   */
  private saveDeploymentResult(result: DeploymentResult): void {
    const deploymentsDir = path.join(__dirname, "../deployments");
    
    // Create deployments directory if it doesn't exist
    if (!fs.existsSync(deploymentsDir)) {
      fs.mkdirSync(deploymentsDir, { recursive: true });
    }
    
    const filename = `deployment-${this.config.network}-${Date.now()}.json`;
    const filepath = path.join(deploymentsDir, filename);
    
    fs.writeFileSync(filepath, JSON.stringify(result, null, 2));
    console.log(`📄 Deployment result saved to: ${filepath}`);
  }

  /**
   * Main deployment function
   */
  async deploy(): Promise<DeploymentResult> {
    console.log(`🚀 Starting deployment to ${this.config.network}...`);
    console.log(`📡 RPC URL: ${this.config.rpcUrl}`);
    console.log(`👤 Deployer: ${this.config.accountAddress}`);
    
    try {
      // Deploy ESCROW first
      const escrowAddress = await this.deployESCROW(this.config.tokenAddress);
      
      // Deploy Resolver with ESCROW address
      const resolverAddress = await this.deployResolver(escrowAddress);
      
      // Prepare deployment result
      const result: DeploymentResult = {
        escrowAddress,
        resolverAddress,
        deployerAddress: this.config.accountAddress,
        network: this.config.network,
        timestamp: Date.now(),
      };
      
      // Save deployment result
      this.saveDeploymentResult(result);
      
      console.log("\n🎉 Deployment completed successfully!");
      console.log("📋 Deployment Summary:");
      console.log(`   Network: ${result.network}`);
      console.log(`   ESCROW Address: ${result.escrowAddress}`);
      console.log(`   Resolver Address: ${result.resolverAddress}`);
      console.log(`   Deployer: ${result.deployerAddress}`);
      console.log(`   Timestamp: ${new Date(result.timestamp).toISOString()}`);
      
      return result;
    } catch (error) {
      console.error("💥 Deployment failed:", error);
      throw error;
    }
  }

  /**
   * Get contract instances for testing
   */
  async getContractInstances(escrowAddress: string, resolverAddress: string) {
    const escrowArtifacts = await this.loadContractArtifacts("starknet_escrow_ESCROW");
    const resolverArtifacts = await this.loadContractArtifacts("starknet_resolver_Resolver");
    
    const escrowContract = new Contract(
      escrowArtifacts.sierraCode.abi,
      escrowAddress,
      this.provider
    );
    
    const resolverContract = new Contract(
      resolverArtifacts.sierraCode.abi,
      resolverAddress,
      this.provider
    );
    
    return { escrowContract, resolverContract };
  }
}

// Configuration presets
const CONFIG_PRESETS = {
  devnet: {
    network: "devnet" as const,
    rpcUrl: "http://127.0.0.1:8547",
    accountAddress: "0x0260a8311b4f1092db620b923e8d7d20e76dedcc615fb4b6fdf28315b81de201",
    privateKey: "0x00000000000000000000000000000000c10662b7b247c7cecf7e8a30726cff12",
  },
  testnet: {
    network: "testnet" as const,
    rpcUrl: "https://alpha4.starknet.io",
    accountAddress: process.env.TESTNET_ACCOUNT_ADDRESS || "",
    privateKey: process.env.TESTNET_PRIVATE_KEY || "",
  },
  mainnet: {
    network: "mainnet" as const,
    rpcUrl: "https://alpha-mainnet.starknet.io",
    accountAddress: process.env.MAINNET_ACCOUNT_ADDRESS || "",
    privateKey: process.env.MAINNET_PRIVATE_KEY || "",
  },
};

// Main deployment function
async function main() {
  const network = process.argv[2] as keyof typeof CONFIG_PRESETS;
  
  if (!network || !CONFIG_PRESETS[network]) {
    console.error("❌ Invalid network. Usage: npm run deploy <devnet|testnet|mainnet>");
    process.exit(1);
  }
  
  const config = CONFIG_PRESETS[network];
  
  // Validate configuration
  if (!config.accountAddress || !config.privateKey) {
    console.error(`❌ Missing account configuration for ${network}`);
    console.error("Please set the required environment variables:");
    if (network === "testnet") {
      console.error("  TESTNET_ACCOUNT_ADDRESS");
      console.error("  TESTNET_PRIVATE_KEY");
    } else if (network === "mainnet") {
      console.error("  MAINNET_ACCOUNT_ADDRESS");
      console.error("  MAINNET_PRIVATE_KEY");
    }
    process.exit(1);
  }
  
  try {
    const deployer = new EscrowDeployer(config);
    await deployer.deploy();
  } catch (error) {
    console.error("💥 Deployment failed:", error);
    process.exit(1);
  }
}

// Export for use in tests
export { EscrowDeployer, CONFIG_PRESETS, DeployConfig, DeploymentResult };

// Run if called directly
if (require.main === module) {
  main();
} 