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

export interface Signature {
  r: string;
  vs: string;
}

export interface Extension {
  customData: string;
  makerAssetSuffix: string;
  makerPermit: string;
  makingAmountData: string;
  postInteraction: string;
  preInteraction: string;
  predicate: string;
  takerAssetSuffix: string;
  takingAmountData: string;
}

export interface OrderData {
  salt: string;
  maker_asset: string;
  taker_asset: string;
  maker: string;
  receiver: string;
  making_amount: string;
  taking_amount: string;
  maker_traits: string;
}

export interface ActiveOrder {
  order_hash: string;
  signature: any;
  deadline: number;
  auction_start_date: string | null;
  auction_end_date: string | null;
  remaining_maker_amount: string;
  extension: Extension;
  src_chain_id: string;
  dst_chain_id: string;
  order: OrderData;
  taker: string;
  timelock: string;
  taker_traits: string;
  args: string;
  order_type: string;
  secrets: Secret[];
  src_deploy_immutables: string | null;
  dst_deploy_immutables: string | null;
  src_withdraw_immutables: string | null;
  dst_withdraw_immutables: string | null;
  src_event: string | null;
  dest_event: string | null;
  src_withdraw: string | null;
  dst_withdraw: string | null;
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

export interface SingleOrderResponse {
  status: string;
  result: ActiveOrder;
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