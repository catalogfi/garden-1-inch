"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PaginationRequest = void 0;
class PaginationRequest {
    constructor(page, limit) {
        this.page = page;
        this.limit = limit;
    }
    validate() {
        if (this.limit != null && (this.limit < 1 || this.limit > 500)) {
            return 'limit should be in range between 1 and 500';
        }
        if (this.page != null && this.page < 1) {
            return `page should be >= 1`;
        }
        return null;
    }
}
exports.PaginationRequest = PaginationRequest;
//# sourceMappingURL=pagination.js.map