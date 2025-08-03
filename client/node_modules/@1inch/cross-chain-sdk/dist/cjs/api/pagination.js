"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PaginationRequest = void 0;
class PaginationRequest {
    constructor(page, limit) {
        if (limit != undefined && (limit < 1 || limit > 500)) {
            throw Error('limit should be in range between 1 and 500');
        }
        if (page != undefined && page < 1) {
            throw Error(`page should be >= 1`);
        }
        this.page = page;
        this.limit = limit;
    }
}
exports.PaginationRequest = PaginationRequest;
//# sourceMappingURL=pagination.js.map