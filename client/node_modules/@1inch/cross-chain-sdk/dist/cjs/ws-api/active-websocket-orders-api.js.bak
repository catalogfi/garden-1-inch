"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ActiveOrdersWebSocketApi = void 0;
const constants_1 = require("./constants");
const types_1 = require("./types");
class ActiveOrdersWebSocketApi {
    constructor(provider) {
        this.provider = provider;
    }
    onOrder(cb) {
        this.provider.onMessage((data) => {
            if (constants_1.orderEvents.includes(data.event)) {
                cb(data);
            }
        });
    }
    onOrderCreated(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderCreated) {
                cb(data);
            }
        });
    }
    onOrderInvalid(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderInvalid) {
                cb(data);
            }
        });
    }
    onOrderBalanceChange(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderBalanceChange) {
                cb(data);
            }
        });
    }
    onOrderAllowanceChange(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderAllowanceChange) {
                cb(data);
            }
        });
    }
    onOrderFilled(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderFilled) {
                cb(data);
            }
        });
    }
    onOrderCancelled(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderCancelled) {
                cb(data);
            }
        });
    }
    onOrderFilledPartially(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderFilledPartially) {
                cb(data);
            }
        });
    }
    onOrderSecretShared(cb) {
        this.provider.onMessage((data) => {
            if (data.event === types_1.EventType.OrderSecretShared) {
                cb(data);
            }
        });
    }
}
exports.ActiveOrdersWebSocketApi = ActiveOrdersWebSocketApi;
//# sourceMappingURL=active-websocket-orders-api.js.map