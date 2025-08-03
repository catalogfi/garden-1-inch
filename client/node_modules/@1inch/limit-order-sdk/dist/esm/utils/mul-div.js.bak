export var Rounding;
(function (Rounding) {
    Rounding[Rounding["Ceil"] = 0] = "Ceil";
    Rounding[Rounding["Floor"] = 1] = "Floor";
})(Rounding || (Rounding = {}));
export function mulDiv(a, b, x, rounding = Rounding.Floor) {
    const res = (a * b) / x;
    if (rounding === Rounding.Ceil && (a * b) % x > 0) {
        return res + 1n;
    }
    return res;
}
//# sourceMappingURL=mul-div.js.map