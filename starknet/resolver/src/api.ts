import axios from 'axios';
import { ActiveOrdersResponse } from './types';
import { API_BASE_URL, ACTIVE_ORDERS_ENDPOINT } from './config';

export class OrderAPI {
  private baseURL: string;

  constructor(baseURL: string = API_BASE_URL) {
    this.baseURL = baseURL;
  }

  async checkHealth(): Promise<boolean> {
    try {
      const response = await axios.get(`${this.baseURL}/health`, {
        timeout: 5000 // 5 second timeout
      });

      if (response.status !== 200) {
        console.error(`Health check failed: HTTP ${response.status}`);
        return false;
      }

      const healthStatus = response.data;
      if (healthStatus === 'Online') {
        console.log('Health check passed: API is online');
        return true;
      } else {
        console.error(`Health check failed: Expected 'Online', got '${healthStatus}'`);
        return false;
      }
    } catch (error) {
      console.error('Health check failed:', error);
      return false;
    }
  }

  async getActiveOrders(page: number = 1, limit: number = 100): Promise<ActiveOrdersResponse> {
    try {
      const response = await axios.get(`${this.baseURL}${ACTIVE_ORDERS_ENDPOINT}`, {
        params: {
          page,
          limit
        },
        timeout: 10000 // 10 second timeout
      });

      if (response.status !== 200) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      // Validate response format
      if (!response.data || typeof response.data !== 'object') {
        throw new Error('Invalid response format: expected object');
      }

      if (response.data.status !== 'Ok') {
        throw new Error(`API returned non-ok status: ${response.data.status}`);
      }

      if (!response.data.result || !response.data.result.meta || !Array.isArray(response.data.result.items)) {
        throw new Error('Invalid response format: missing required fields');
      }

      return response.data;
    } catch (error) {
      console.error('Error fetching active orders:', error);
      throw error;
    }
  }

  async getAllActiveOrders(): Promise<ActiveOrdersResponse['result']['items']> {
    const allOrders: ActiveOrdersResponse['result']['items'] = [];
    let currentPage = 1;
    let hasMorePages = true;

    while (hasMorePages) {
      try {
        const response = await this.getActiveOrders(currentPage);
        
        if (response.status !== 'Ok') {
          throw new Error(`API error: ${response.status}`);
        }

        allOrders.push(...response.result.items);
        
        // Check if there are more pages
        hasMorePages = currentPage < response.result.meta.total_pages;
        currentPage++;
        
        // Add a small delay between requests to be respectful
        if (hasMorePages) {
          await new Promise(resolve => setTimeout(resolve, 100));
        }
      } catch (error) {
        console.error(`Error fetching page ${currentPage}:`, error);
        throw error;
      }
    }

    return allOrders;
  }
} 