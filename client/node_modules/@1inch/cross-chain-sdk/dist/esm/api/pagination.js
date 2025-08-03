export class PaginationRequest {
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
//# sourceMappingURL=pagination.js.map