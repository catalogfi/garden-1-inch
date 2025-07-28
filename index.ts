import {
  HashLock,
  NetworkEnum,
  OrderStatus,
  PresetEnum,
  PrivateKeyProviderConnector,
  SDK,
  type Quote,
  type QuoteParams,
  type PreparedOrder
} from '@1inch/cross-chain-sdk';
import { ethers } from 'ethers';
import { randomBytes } from 'node:crypto';
import { writeFileSync } from 'fs';
import { join } from 'path';

// Configuration
const CONFIG = {
  privateKey: "0xprivate_key",
  rpcUrl: "https://eth-mainnet.g.alchemy.com/v2/key", 
  authKey: "auth_key",
  source: "FusionPlusBot",
  makerAddress: "0xYourMakerAddress", 
};

class FusionPlusSwap {
  private sdk: SDK;
  private provider: ethers.JsonRpcProvider;
  private walletAddress: string;

  constructor() {
    this.provider = new ethers.JsonRpcProvider(CONFIG.rpcUrl);
    this.walletAddress = new ethers.Wallet(CONFIG.privateKey, this.provider).address;

    // Create Web3-like adapter for ethers
    const web3Like = {
      eth: {
        call: async (transactionConfig: any) => {
          return await this.provider.call(transactionConfig);
        }
      },
      extend: () => { } // Required by Web3Like interface
    };

    this.sdk = new SDK({
      url: "https://api.1inch.dev/fusion-plus",
      authKey: CONFIG.authKey,
      blockchainProvider: new PrivateKeyProviderConnector(
        CONFIG.privateKey,
        web3Like as any
      )
    });
  }

  private saveToJsonFile(filename: string, data: any): void {
    try {
      const filePath = join(process.cwd(), filename);
      const jsonString = JSON.stringify(data, (key, value) => {
        if (typeof value === 'bigint') {
          return value.toString();
        }
        return value;
      }, 2);
      writeFileSync(filePath, jsonString);
      console.log(`💾 Data saved to ${filename}`);
    } catch (error) {
      console.error(`Error saving to ${filename}:`, error);
    }
  }

  private async sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  async getQuote(params: QuoteParams): Promise<Quote> {
    console.log("🔍 Getting quote...");
    const quote = await this.sdk.getQuote(params);
    this.saveToJsonFile("quote_response.json", quote);
    return quote;
  }

  async createAndSubmitOrder(quote: Quote, preset: PresetEnum = PresetEnum.fast): Promise<PreparedOrder> {
    console.log("📝 Creating order...");

    // Generate secrets
    const secrets = Array.from({ length: quote.presets[preset]!.secretsCount })
      .map(() => '0x' + randomBytes(32).toString('hex'));

    const hashLock = secrets.length === 1
      ? HashLock.forSingleFill(secrets[0] as string)
      : HashLock.forMultipleFills(HashLock.getMerkleLeaves(secrets));

    const secretHashes = secrets.map((s) => HashLock.hashSecret(s));

    // Create order
    const orderResponse = await this.sdk.createOrder(quote, {
      walletAddress: this.walletAddress,
      hashLock,
      preset,
      source: CONFIG.source,
      secretHashes,
      receiver: this.walletAddress,
    });
    console.log(orderResponse.order.getTypedData(NetworkEnum.ETHEREUM));

    this.saveToJsonFile("create_order_response.json", orderResponse);
    console.log(`✅ Order created! Hash: ${orderResponse.hash}`);

    // const { order, orderHash, signature, extension, quoteId } = await this.sdk.submitOrder(
    //   quote.srcChainId,
    //   orderResponse.order,
    //   orderResponse.quoteId,
    //   secretHashes
    // );
    // console.log(`📤 Order submitted! Hash: ${orderHash}`);
    // console.log(`✍️ Signature: ${signature}`);
    // console.log(`🔗 Quote ID: ${quoteId}`)
    // this.saveToJsonFile("submit_order_response.json", {
    //   order,
    //   orderHash,
    //   signature,
    //   extension,
    //   quoteId
    // });

    return orderResponse;
  }

  async monitorOrder(orderHash: string, secrets: string[]): Promise<void> {
    console.log("👀 Starting order monitoring...");

    while (true) {
      const statusResponse = await this.sdk.getOrderStatus(orderHash);
      this.saveToJsonFile("order_status.json", statusResponse);

      console.log(`📊 Current status: ${statusResponse.status}`);

      // Check if order is completed
      if ([
        OrderStatus.Executed,
        OrderStatus.Expired,
        OrderStatus.Refunded
      ].includes(statusResponse.status)) {
        console.log("🏁 Order completed with status:", statusResponse.status);
        break;
      }

      // Submit secrets for ready fills
      const readyFills = await this.sdk.getReadyToAcceptSecretFills(orderHash);
      this.saveToJsonFile("ready_fills.json", readyFills);

      if (readyFills.fills.length > 0) {
        for (const { idx } of readyFills.fills) {
          await this.sdk.submitSecret(orderHash, secrets[idx] as string);
          console.log(`🔓 Submitted secret for fill ${idx}`);
        }
      }

      await this.sleep(5000); // Poll every 5 seconds
    }
  }

  async executeSwap(params: QuoteParams): Promise<void> {
    try {
      console.log("🚀 Starting swap process...");

      // Get quote
      const quote = await this.getQuote(params);
      console.log("✅ Quote received");

      // Create and submit order
      const orderResponse = await this.createAndSubmitOrder(quote);

      // Generate secrets again for monitoring (in a real app, you would store these securely)
      const secrets = Array.from({ length: quote.presets[PresetEnum.fast].secretsCount })
        .map(() => '0x' + randomBytes(32).toString('hex'));

      // Monitor order status
      // await this.monitorOrder(orderResponse.hash, secrets);

      console.log("🎉 Swap completed successfully!");
    } catch (error) {
      console.error("💥 Swap failed:", error);
      this.saveToJsonFile("swap_error.json", {
        error: error instanceof Error ? error.message : String(error),
        timestamp: new Date().toISOString()
      });
      throw error;
    }
  }
}

async function main(): Promise<void> {
  const swapper = new FusionPlusSwap();

  const swapParams: QuoteParams = {
    srcChainId: NetworkEnum.ETHEREUM,
    dstChainId: NetworkEnum.POLYGON,
    srcTokenAddress: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", // WETH
    dstTokenAddress: "0x2791bca1f2de4661ed88a30c99a7a9449aa84174", // USDC
    amount: "100000000000000000", // 0.1 ETH
    walletAddress: CONFIG.makerAddress,
    enableEstimate: true,
    source: CONFIG.source
  };

  await swapper.executeSwap(swapParams);

  // await swapper.getQuote(swapParams);
}

main()