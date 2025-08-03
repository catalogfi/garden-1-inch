"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.castUrl = void 0;
function castUrl(url) {
    if (url.startsWith('http')) {
        return url.replace('http', 'ws');
    }
    return url;
}
exports.castUrl = castUrl;
//# sourceMappingURL=url.js.map