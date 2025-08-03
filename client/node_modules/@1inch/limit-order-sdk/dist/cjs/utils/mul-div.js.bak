"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Rounding = void 0;
exports.mulDiv = mulDiv;
var Rounding;
(function (Rounding) {
    Rounding[Rounding["Ceil"] = 0] = "Ceil";
    Rounding[Rounding["Floor"] = 1] = "Floor";
})(Rounding || (exports.Rounding = Rounding = {}));
function mulDiv(a, b, x, rounding = Rounding.Floor) {
    const res = (a * b) / x;
    if (rounding === Rounding.Ceil && (a * b) % x > 0) {
        return res + 1n;
    }
    return res;
}
//# sourceMappingURL=mul-div.js.map