"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PublicAction = exports.OrderType = exports.EscrowEventAction = exports.EscrowEventSide = exports.OrderStatus = exports.FillStatus = exports.ValidationStatus = void 0;
var ValidationStatus;
(function (ValidationStatus) {
    ValidationStatus["Valid"] = "valid";
    ValidationStatus["OrderPredicateReturnedFalse"] = "order-predicate-returned-false";
    ValidationStatus["NotEnoughBalance"] = "not-enough-balance";
    ValidationStatus["NotEnoughAllowance"] = "not-enough-allowance";
    ValidationStatus["InvalidPermitSignature"] = "invalid-permit-signature";
    ValidationStatus["InvalidPermitSpender"] = "invalid-permit-spender";
    ValidationStatus["InvalidPermitSigner"] = "invalid-permit-signer";
    ValidationStatus["InvalidSignature"] = "invalid-signature";
    ValidationStatus["FailedToParsePermitDetails"] = "failed-to-parse-permit-details";
    ValidationStatus["UnknownPermitVersion"] = "unknown-permit-version";
    ValidationStatus["WrongEpochManagerAndBitInvalidator"] = "wrong-epoch-manager-and-bit-invalidator";
    ValidationStatus["FailedToDecodeRemainingMakerAmount"] = "failed-to-decode-remaining";
    ValidationStatus["UnknownFailure"] = "unknown-failure";
})(ValidationStatus || (exports.ValidationStatus = ValidationStatus = {}));
var FillStatus;
(function (FillStatus) {
    FillStatus["Pending"] = "pending";
    FillStatus["Executed"] = "executed";
    FillStatus["Refunding"] = "refunding";
    FillStatus["Refunded"] = "refunded";
})(FillStatus || (exports.FillStatus = FillStatus = {}));
var OrderStatus;
(function (OrderStatus) {
    OrderStatus["Pending"] = "pending";
    OrderStatus["Executed"] = "executed";
    OrderStatus["Expired"] = "expired";
    OrderStatus["Cancelled"] = "cancelled";
    OrderStatus["Refunding"] = "refunding";
    OrderStatus["Refunded"] = "refunded";
})(OrderStatus || (exports.OrderStatus = OrderStatus = {}));
var EscrowEventSide;
(function (EscrowEventSide) {
    EscrowEventSide["Src"] = "src";
    EscrowEventSide["Dst"] = "dst";
})(EscrowEventSide || (exports.EscrowEventSide = EscrowEventSide = {}));
var EscrowEventAction;
(function (EscrowEventAction) {
    EscrowEventAction["SrcEscrowCreated"] = "src_escrow_created";
    EscrowEventAction["DstEscrowCreated"] = "dst_escrow_created";
    EscrowEventAction["Withdrawn"] = "withdrawn";
    EscrowEventAction["FundsRescued"] = "funds_rescued";
    EscrowEventAction["EscrowCancelled"] = "escrow_cancelled";
})(EscrowEventAction || (exports.EscrowEventAction = EscrowEventAction = {}));
var OrderType;
(function (OrderType) {
    OrderType["SingleFill"] = "SingleFill";
    OrderType["MultipleFills"] = "MultipleFills";
})(OrderType || (exports.OrderType = OrderType = {}));
var PublicAction;
(function (PublicAction) {
    PublicAction["Withdraw"] = "withdraw";
    PublicAction["Cancel"] = "cancel";
})(PublicAction || (exports.PublicAction = PublicAction = {}));
//# sourceMappingURL=types.js.map