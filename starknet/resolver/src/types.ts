export interface Order {
  salt: string;
  makerAsset: string;
  takerAsset: string;
  maker: string;
  receiver: string;
  makingAmount: string;
  takingAmount: string;
  makerTraits: string;
}

export interface Secret {
  index: number;
  secret: string | null;
  secretHash: string;
}

export interface ActiveOrder {
  orderHash: string;
  signature: string;
  deadline: number;
  auctionStartDate: string;
  auctionEndDate: string;
  remainingMakerAmount: string;
  extension: string;
  srcChainId: number;
  dstChainId: number;
  order: Order;
  orderType: string;
  secrets: Secret[];
}

export interface Meta {
  total_items: number;
  items_per_page: number;
  total_pages: number;
  current_page: number;
}

export interface ActiveOrdersResponse {
  status: string;
  result: {
    meta: Meta;
    items: ActiveOrder[];
  };
}

export interface UserIntent {
  salt: string;
  maker: string;
  receiver: string;
  makerAsset: string;
  takerAsset: string;
  makingAmount: string;
  takingAmount: string;
} 