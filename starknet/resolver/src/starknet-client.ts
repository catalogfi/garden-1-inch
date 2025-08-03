import { 
  Account, 
  RpcProvider, 
  Contract, 
  CallData, 
  cairo,
  TypedData,
  TypedDataRevision,
  WeierstrassSignatureType,
  shortString
} from 'starknet';
import { 
  RESOLVER_PRIVATE_KEY, 
  RESOLVER_ADDRESS, 
  RESOLVER_CONTRACT_ADDRESS, 
  STARKNET_RPC_URL,
  STARKNET_CHAIN_ID,
  TIMELOCK
} from './config';
import { ActiveOrder, UserIntent } from './types';
import { ActionType, OrderAction } from './action-mapper';

export class StarknetClient {
  private provider: RpcProvider;
  private account: Account;
  private resolverContract!: Contract;
  private callData!: CallData;

  constructor() {
    this.provider = new RpcProvider({
      nodeUrl: STARKNET_RPC_URL,
    });

    this.account = new Account(
      this.provider,
      RESOLVER_ADDRESS,
      RESOLVER_PRIVATE_KEY,
      "1",
      "0x3"
    );
  }

  async initialize() {
    try {
      // Get the resolver contract class
      const contractData = await this.provider.getClassAt(RESOLVER_CONTRACT_ADDRESS);
      this.resolverContract = new Contract(contractData.abi, RESOLVER_CONTRACT_ADDRESS, this.provider);
      this.callData = new CallData(contractData.abi);
      
      console.log('Starknet client initialized successfully');
    } catch (error) {
      console.error('Error initializing Starknet client:', error);
      throw error;
    }
  }

  private generateOrderHash(): string {
    // Generate a random felt252-compatible value
    const randomValue = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
    return "0x" + randomValue.toString(16);
  }

  private hexToU32Array(hex: string): number[] {
    const bytes = Buffer.from(hex.slice(2), 'hex');
    const u32Array = [];
    for (let i = 0; i < bytes.length; i += 4) {
      const chunk = bytes.slice(i, i + 4);
      const value = chunk.readUInt32BE(0);
      u32Array.push(value);
    }
    return u32Array;
  }

  async createSourceOrder(order: ActiveOrder): Promise<boolean> {
    try {
      console.log(`Processing order ${order.orderHash} from Starknet`);

      // Convert the order to UserIntent format
      const userIntent: UserIntent = {
        salt: order.order.salt,
        maker: order.order.maker,
        receiver: order.order.receiver,
        makerAsset: order.order.makerAsset,
        takerAsset: order.order.takerAsset,
        makingAmount: order.order.makingAmount,
        takingAmount: order.order.takingAmount,
      };

      // Create typed data for signature verification
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
        chainId: STARKNET_CHAIN_ID.toString(),
        revision: TypedDataRevision.ACTIVE,
      };

      const typedData: TypedData = {
        domain: DOMAIN,
        primaryType: "UserIntent",
        types: USER_INTENT_TYPE,
        message: {
          salt: cairo.uint256(BigInt(userIntent.salt)),
          maker: userIntent.maker,
          receiver: userIntent.receiver,
          maker_asset: userIntent.makerAsset,
          taker_asset: userIntent.takerAsset,
          making_amount: cairo.uint256(BigInt(userIntent.makingAmount)),
          taking_amount: cairo.uint256(BigInt(userIntent.takingAmount)),
        },
      };

      // Parse the signature from the order
      const signature = order.signature;
      const signatureArray = this.parseSignature(signature);

      // Generate a new order hash for the escrow
      const escrowOrderHash = this.generateOrderHash();

      // Get the first secret hash from the order
      const secretHash = this.hexToU32Array(order.secrets[0]?.secretHash || "0x0000000000000000000000000000000000000000000000000000000000000000");

      const input = {
        user_address: userIntent.maker,
        resolver_address: RESOLVER_ADDRESS,
        user_intent: {
          salt: cairo.uint256(BigInt(userIntent.salt)),
          maker: userIntent.maker,
          receiver: userIntent.receiver,
          maker_asset: userIntent.makerAsset,
          taker_asset: userIntent.takerAsset,
          making_amount: cairo.uint256(BigInt(userIntent.makingAmount)),
          taking_amount: cairo.uint256(BigInt(userIntent.takingAmount)),
        },
        signature: signatureArray,
        order_hash: escrowOrderHash,
        timelock: TIMELOCK,
        secret_hash: secretHash,
        amount: cairo.uint256(BigInt(userIntent.makingAmount))
      };

      // Execute the create_source function
      const result = await this.account.execute({
        contractAddress: RESOLVER_CONTRACT_ADDRESS,
        entrypoint: "create_source",
        calldata: this.callData.compile("create_source", input),
      });

      console.log(`Successfully created source order for ${order.orderHash}`);
      console.log('Transaction hash:', result.transaction_hash);
      
      return true;
    } catch (error) {
      console.error(`Error creating source order for ${order.orderHash}:`, error);
      return false;
    }
  }

  private parseSignature(signature: string): [string, string] {
    // Remove 0x prefix if present
    const cleanSignature = signature.startsWith('0x') ? signature.slice(2) : signature;
    
    // Split into r and s components (each 32 bytes = 64 hex chars)
    const r = '0x' + cleanSignature.slice(0, 64);
    const s = '0x' + cleanSignature.slice(64, 128);
    
    return [r, s];
  }

  async processOrderAction(action: OrderAction): Promise<boolean> {
    try {
      console.log(`Processing action ${action.actionType} for order ${action.orderId}`);
      
      switch (action.actionType) {
        case ActionType.DeploySrcEscrow:
          return await this.createSourceOrder(action.order);
        case ActionType.DeployDestEscrow:
          return await this.createDestinationOrder(action.order);
        case ActionType.WidthdrawSrcEscrow:
          return await this.withdrawFromSourceEscrow(action.order);
        case ActionType.WidthdrawDestEscrow:
          return await this.withdrawFromDestEscrow(action.order);
        default:
          console.log(`No action needed for ${action.actionType}`);
          return true;
      }
    } catch (error) {
      console.error(`Error processing action ${action.actionType} for order ${action.orderId}:`, error);
      return false;
    }
  }

  async processStarknetOrders(orders: any[]): Promise<number> {
    let processedCount = 0;
    
    for (const order of orders) {
      // Check if this order involves Starknet (either as source or destination)
      if (order.src_chain_id === STARKNET_CHAIN_ID || order.dst_chain_id === STARKNET_CHAIN_ID) {
        console.log(`Processing Starknet order ${order.order_hash} with status: ${order.status}`);
        console.log(`Source chain: ${order.src_chain_id}, Destination chain: ${order.dst_chain_id}`);
        
        // TODO: Implement actual action processing based on the action mapper
        processedCount++;
        
        // Add a small delay between transactions
        await new Promise(resolve => setTimeout(resolve, 2000));
      }
    }
    
    return processedCount;
  }

  // Placeholder methods for other actions
  private async createDestinationOrder(order: any): Promise<boolean> {
    console.log(`Creating destination order for ${order.order_hash}`);
    // TODO: Implement create_destination logic
    return true;
  }

  private async withdrawFromSourceEscrow(order: any): Promise<boolean> {
    console.log(`Withdrawing from source escrow for ${order.order_hash}`);
    // TODO: Implement withdraw logic
    return true;
  }

  private async withdrawFromDestEscrow(order: any): Promise<boolean> {
    console.log(`Withdrawing from destination escrow for ${order.order_hash}`);
    // TODO: Implement withdraw logic
    return true;
  }
} 