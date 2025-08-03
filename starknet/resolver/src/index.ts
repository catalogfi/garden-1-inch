import { OrderAPI } from './api';
import { StarknetClient } from './starknet-client';
import { ActionMapper } from './action-mapper';
import { STARKNET_CHAIN_ID, POLLING_INTERVAL } from './config';

class Resolver {
  private orderAPI: OrderAPI;
  private starknetClient: StarknetClient;
  private actionMapper: ActionMapper;
  private isRunning: boolean = false;

  constructor() {
    this.orderAPI = new OrderAPI();
    this.starknetClient = new StarknetClient();
    this.actionMapper = new ActionMapper(STARKNET_CHAIN_ID);
  }

  async initialize() {
    try {
      const isHealthy = await this.orderAPI.checkHealth();
      if (!isHealthy) {
        throw new Error('API health check failed');
      }
      await this.starknetClient.initialize();
    } catch (error) {
      console.error('Failed to initialize resolver:', error);
      throw error;
    }
  }

  async processOrders() {
    try {
      console.log('Fetching active orders...');
      const orders = await this.orderAPI.getAllActiveOrders();
      console.log(`Found ${orders.length} total active orders`);

      if (orders.length === 0) {
        console.log('No active orders found');
        return;
      }

      // Filter orders that involve Starknet (either as source or destination)
      const starknetOrders = this.actionMapper.filterStarknetOrders(orders);
      console.log(`Found ${starknetOrders.length} Starknet orders`);

      if (starknetOrders.length === 0) {
        console.log('No Starknet orders to process');
        return;
      }

      // Get actions for all Starknet orders
      const actions = this.actionMapper.getActionsForOrders(starknetOrders);
      console.log(`Generated ${actions.length} actions for Starknet orders`);

      if (actions.length === 0) {
        console.log('No actions needed for Starknet orders');
        return;
      }

      // Process each action
      let processedCount = 0;
      for (const action of actions) {
        console.log(`Processing action: ${action.actionType} for order: ${action.orderId}`);
        const success = await this.starknetClient.processOrderAction(action);
        if (success) {
          processedCount++;
        }
        
        // Add a small delay between actions
        await new Promise(resolve => setTimeout(resolve, 2000));
      }

      console.log(`Successfully processed ${processedCount} out of ${actions.length} actions`);

    } catch (error) {
      console.error('Error processing orders:', error);
    }
  }

  async start() {
    if (this.isRunning) {
      console.log('Resolver is already running');
      return;
    }

    this.isRunning = true;
    console.log('Starting resolver...');

    // Initial processing
    await this.processOrders();

    // Set up polling
    const poll = async () => {
      if (!this.isRunning) return;
      
      try {
        await this.processOrders();
      } catch (error) {
        console.error('Error in polling cycle:', error);
      }

      // Schedule next poll
      setTimeout(poll, POLLING_INTERVAL);
    };

    // Start polling
    setTimeout(poll, POLLING_INTERVAL);
  }

  stop() {
    this.isRunning = false;
    console.log('Resolver stopped');
  }
}

// Main execution
async function main() {
  const resolver = new Resolver();

  // Handle graceful shutdown
  process.on('SIGINT', () => {
    console.log('\nReceived SIGINT, shutting down gracefully...');
    resolver.stop();
    process.exit(0);
  });

  process.on('SIGTERM', () => {
    console.log('\nReceived SIGTERM, shutting down gracefully...');
    resolver.stop();
    process.exit(0);
  });

  try {
    await resolver.initialize();
    await resolver.start();
  } catch (error) {
    console.error('Failed to start resolver:', error);
    process.exit(1);
  }
}

// Run the resolver
if (require.main === module) {
  main().catch(console.error);
}

export { Resolver }; 