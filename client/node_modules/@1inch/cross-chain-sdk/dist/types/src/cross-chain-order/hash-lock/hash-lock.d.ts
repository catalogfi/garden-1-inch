export declare class HashLock {
    static Web3Type: string;
    private readonly value;
    protected constructor(val: string);
    static hashSecret(secret: string): string;
    static getMerkleLeaves(secrets: string[]): MerkleLeaf[];
    static getMerkleLeavesFromSecretHashes(secretHashes: string[]): MerkleLeaf[];
    static getProof(leaves: string[], idx: number): MerkleLeaf[];
    static fromString(value: string): HashLock;
    /**
     * Create HashLock from keccak256 hash of secret
     */
    static forSingleFill(secret: string): HashLock;
    static forMultipleFills(leaves: MerkleLeaf[]): HashLock;
    /**
     * Only use if HashLockInfo is for multiple fill order
     * Otherwise garbage will be returned
     *
     */
    getPartsCount(): bigint;
    toString(): string;
    eq(other: HashLock): boolean;
}
export type MerkleLeaf = string & {
    _tag: 'MerkleLeaf';
};
