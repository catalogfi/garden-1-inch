"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ActiveOrdersWebSocketApi = void 0;
const constants_1 = require("./constants");
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
            if (data.event === 'order_created') {
                cb(data);
            }
        });
    }
    onOrderInvalid(cb) {
        this.provider.onMessage((data) => {
            if (data.event === 'order_invalid') {
                cb(data);
            }
        });
    }
    onOrderBalanceOrAllowanceChange(cb) {
        this.provider.onMessage((data) => {
            if (data.event === 'order_balance_or_allowance_change') {
                cb(data);
            }
        });
    }
    onOrderFilled(cb) {
        this.provider.onMessage((data) => {
            if (data.event === 'order_filled') {
                cb(data);
            }
        });
    }
    onOrderCancelled(cb) {
        this.provider.onMessage((data) => {
            if (data.event === 'order_cancelled') {
                cb(data);
            }
        });
    }
    onOrderFilledPartially(cb) {
        this.provider.onMessage((data) => {
            if (data.event === 'order_filled_partially') {
                cb(data);
            }
        });
    }
}
exports.ActiveOrdersWebSocketApi = ActiveOrdersWebSocketApi;
//# sourceMappingURL=active-websocket-orders-api.js.map