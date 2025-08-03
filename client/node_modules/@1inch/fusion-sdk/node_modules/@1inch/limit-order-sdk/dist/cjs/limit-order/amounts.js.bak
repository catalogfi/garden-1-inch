"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.calcTakingAmount = calcTakingAmount;
exports.calcMakingAmount = calcMakingAmount;
function calcTakingAmount(swapMakerAmount, orderMakerAmount, orderTakerAmount) {
    return ((swapMakerAmount * orderTakerAmount + orderMakerAmount - 1n) /
        orderMakerAmount);
}
function calcMakingAmount(swapTakerAmount, orderMakerAmount, orderTakerAmount) {
    return (swapTakerAmount * orderMakerAmount) / orderTakerAmount;
}
//# sourceMappingURL=amounts.js.map