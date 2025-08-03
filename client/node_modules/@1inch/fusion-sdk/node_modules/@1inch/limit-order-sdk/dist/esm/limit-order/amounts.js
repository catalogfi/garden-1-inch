export function calcTakingAmount(swapMakerAmount, orderMakerAmount, orderTakerAmount) {
    return ((swapMakerAmount * orderTakerAmount + orderMakerAmount - 1n) /
        orderMakerAmount);
}
export function calcMakingAmount(swapTakerAmount, orderMakerAmount, orderTakerAmount) {
    return (swapTakerAmount * orderMakerAmount) / orderTakerAmount;
}
//# sourceMappingURL=amounts.js.map