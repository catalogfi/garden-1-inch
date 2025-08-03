export var EventType;
(function (EventType) {
    EventType["OrderCreated"] = "order_created";
    EventType["OrderInvalid"] = "order_invalid";
    EventType["OrderBalanceChange"] = "order_balance_change";
    EventType["OrderAllowanceChange"] = "order_allowance_change";
    EventType["OrderFilled"] = "order_filled";
    EventType["OrderFilledPartially"] = "order_filled_partially";
    EventType["OrderCancelled"] = "order_cancelled";
    EventType["OrderSecretShared"] = "secret_shared";
})(EventType || (EventType = {}));
export var RpcMethod;
(function (RpcMethod) {
    RpcMethod["GetAllowedMethods"] = "getAllowedMethods";
    RpcMethod["Ping"] = "ping";
    RpcMethod["GetActiveOrders"] = "getActiveOrders";
    RpcMethod["GetSecrets"] = "getSecrets";
})(RpcMethod || (RpcMethod = {}));
export var WebSocketEvent;
(function (WebSocketEvent) {
    WebSocketEvent["Close"] = "close";
    WebSocketEvent["Error"] = "error";
    WebSocketEvent["Message"] = "message";
    WebSocketEvent["Open"] = "open";
})(WebSocketEvent || (WebSocketEvent = {}));
//# sourceMappingURL=types.js.map