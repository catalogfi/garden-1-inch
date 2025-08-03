"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AuthError = void 0;
class AuthError extends Error {
    constructor() {
        super('Auth error, please use token from https://portal.1inch.dev/');
    }
}
exports.AuthError = AuthError;
//# sourceMappingURL=errors.js.map