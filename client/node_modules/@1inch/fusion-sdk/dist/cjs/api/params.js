"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.concatQueryParams = void 0;
function concatQueryParams(params) {
    if (!params) {
        return '';
    }
    const keys = Object.keys(params);
    if (keys.length === 0) {
        return '';
    }
    return ('?' +
        keys
            .reduce((a, k) => {
            if (!params[k]) {
                return a;
            }
            const value = params[k];
            a.push(k +
                '=' +
                encodeURIComponent(Array.isArray(value) ? value.join(',') : value));
            return a;
        }, [])
            .join('&'));
}
exports.concatQueryParams = concatQueryParams;
//# sourceMappingURL=params.js.map