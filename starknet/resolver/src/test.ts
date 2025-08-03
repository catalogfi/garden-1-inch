import { OrderAPI } from './api';
import { StarknetClient } from './starknet-client';
import { STARKNET_CHAIN_ID } from './config';

async function testResolver() {
  console.log('Testing resolver functionality...');

  try {
    // Test health check first
    console.log('Testing API health check...');
    const orderAPI = new OrderAPI();
    const isHealthy = await orderAPI.checkHealth();
    if (!isHealthy) {
      throw new Error('API health check failed');
    }
    console.log('Health check passed');
    const orders = await orderAPI.getActiveOrders(1, 10);
    console.log(`API test successful. Found ${orders.result.items.length} orders on first page`);
    
    if (orders.result.items.length === 0) {
      console.log('No orders found in first page');
    }
    const starknetClient = new StarknetClient();
    await starknetClient.initialize();
    // Test filtering Starknet orders
    const allOrders = await orderAPI.getAllActiveOrders();
    console.log(`Total orders found: ${allOrders.length}`);

    console.log("All orders:", allOrders);
    
    if (allOrders.length === 0) {
      console.log('No orders found - skipping Starknet filtering test');
    } else {
      const starknetOrders = allOrders.filter(order => order.srcChainId === STARKNET_CHAIN_ID);
      console.log(`Found ${starknetOrders.length} starknet orders`);

      console.log("Starknet orders:", JSON.stringify(starknetOrders, null, 2));
    }
  } catch (error) {
    console.error('Test failed:', error);
  }
}

// Run the test
testResolver().catch(console.error); 