import { orderEvents } from './constants';
import { EventType } from './types';
export class ActiveOrdersWebSocketApi {
    constructor(provider) {
        this.provider = provider;
    }
    onOrder(cb) {
        this.provider.onMessage((data) => {
            if (orderEvents.includes(data.event)) {
                cb(data);
            }
        });
    }
    onOrderCreated(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderCreated) {
                cb(data);
            }
        });
    }
    onOrderInvalid(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderInvalid) {
                cb(data);
            }
        });
    }
    onOrderBalanceChange(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderBalanceChange) {
                cb(data);
            }
        });
    }
    onOrderAllowanceChange(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderAllowanceChange) {
                cb(data);
            }
        });
    }
    onOrderFilled(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderFilled) {
                cb(data);
            }
        });
    }
    onOrderCancelled(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderCancelled) {
                cb(data);
            }
        });
    }
    onOrderFilledPartially(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderFilledPartially) {
                cb(data);
            }
        });
    }
    onOrderSecretShared(cb) {
        this.provider.onMessage((data) => {
            if (data.event === EventType.OrderSecretShared) {
                cb(data);
            }
        });
    }
}
//# sourceMappingURL=active-websocket-orders-api.js.map