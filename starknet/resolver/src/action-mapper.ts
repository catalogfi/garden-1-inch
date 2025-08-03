export enum ActionType {
  DeploySrcEscrow = 'DeploySrcEscrow',
  DeployDestEscrow = 'DeployDestEscrow',
  WidthdrawSrcEscrow = 'WidthdrawSrcEscrow',
  WidthdrawDestEscrow = 'WidthdrawDestEscrow',
  NoOp = 'NoOp'
}

export interface OrderAction {
  orderId: string;
  actionType: ActionType;
  order: any; // OrderDetail type
}

export class ActionMapper {
  private chainId: number;

  constructor(chainId: number) {
    this.chainId = chainId;
  }

  determineAction(order: any): { sourceAction: ActionType; destAction: ActionType } {
    const status = order.status;
    
    let sourceAction: ActionType = ActionType.NoOp;
    let destAction: ActionType = ActionType.NoOp;

    // Source actions (only if src_chain_id matches our chain)
    if (order.src_chain_id === this.chainId) {
      switch (status) {
        case 'unmatched':
          sourceAction = ActionType.DeploySrcEscrow;
          break;
        case 'destination_filled':
        case 'source_withdraw_pending':
          sourceAction = ActionType.WidthdrawSrcEscrow;
          break;
        default:
          sourceAction = ActionType.NoOp;
      }
    }

    // Destination actions (only if dst_chain_id matches our chain)
    if (order.dst_chain_id === this.chainId) {
      switch (status) {
        case 'source_filled':
          destAction = ActionType.DeployDestEscrow;
          break;
        case 'destination_withdraw_pending':
        case 'source_settled':
          destAction = ActionType.WidthdrawDestEscrow;
          break;
        default:
          destAction = ActionType.NoOp;
      }
    }

    return { sourceAction, destAction };
  }

  getActionsForOrders(orders: any[]): OrderAction[] {
    const actions: OrderAction[] = [];

    for (const order of orders) {
      const { sourceAction, destAction } = this.determineAction(order);

      // Add source action if it's not NoOp
      if (sourceAction !== ActionType.NoOp) {
        actions.push({
          orderId: order.order_hash,
          actionType: sourceAction,
          order
        });
      }

      // Add destination action if it's not NoOp
      if (destAction !== ActionType.NoOp) {
        actions.push({
          orderId: order.order_hash,
          actionType: destAction,
          order
        });
      }
    }

    return actions;
  }

  filterStarknetOrders(orders: any[]): any[] {
    return orders.filter(order => 
      order.src_chain_id === this.chainId || order.dst_chain_id === this.chainId
    );
  }
} 