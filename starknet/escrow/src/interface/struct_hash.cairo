use starknet::ContractAddress;
use crate::interface::{IMessageHash, IStructHash};
use crate::interface::sn_domain::{StarknetDomain};
use core::poseidon::{PoseidonTrait};
use core::hash::{HashStateExTrait, HashStateTrait};
use crate::escrow::ESCROW::{
    INTENT_TYPE_HASH, NAME, VERSION, U256_TYPE_HASH,
};

#[derive(Drop, Copy, Hash, Serde, Debug)]
pub struct UserIntent {
    pub salt: u256,
    pub maker: ContractAddress,
    pub receiver: ContractAddress,
    pub maker_asset: ContractAddress,
    pub taker_asset: ContractAddress,
    pub making_amount: u256,
    pub taking_amount: u256,    
}

#[derive(Drop, Copy, Hash, Serde, Debug)]
pub struct Initiate {
    pub redeemer: ContractAddress,
    pub amount: u256,
    pub timelock: u128,
    pub secretHash: [u32; 8],
}


#[derive(Drop, Copy, Hash, Serde, Debug)]
pub struct VerifySignature {
    pub token: ContractAddress,
    pub amount: u256
}

pub impl MessageHashVerifySignature of IMessageHash<VerifySignature> {
    fn get_message_hash(self: @VerifySignature, chain_id: felt252, signer: ContractAddress) -> felt252 {
        let domain = StarknetDomain {
            name: NAME, version: VERSION, chain_id: chain_id, revision: 1,
        };
        let mut state = PoseidonTrait::new();
        state = state.update_with('StarkNet Message');
        state = state.update_with(domain.get_struct_hash());
        state = state.update_with(signer);
        state = state.update_with(self.get_struct_hash());

        state.finalize()
    }
}

pub const VERIFY_SIGNATURE_TYPE_HASH: felt252 = selector!("\"VerifySignature\"(\"token\":\"ContractAddress\",\"amount\":\"u256\")\"u256\"(\"low\":\"u128\",\"high\":\"u128\")");

pub impl StructHashVerifySignature of IStructHash<VerifySignature> {
    fn get_struct_hash(self: @VerifySignature) -> felt252 {
        let mut state = PoseidonTrait::new();
        state = state.update_with(VERIFY_SIGNATURE_TYPE_HASH);
        state = state.update_with(*self.token);
        state = state.update_with(self.amount.get_struct_hash());
        state.finalize()
    }
}

pub impl MessageHashUserIntent of IMessageHash<UserIntent> {
    fn get_message_hash(self: @UserIntent, chain_id: felt252, signer: ContractAddress) -> felt252 {
        let domain = StarknetDomain {
            name: NAME, version: VERSION, chain_id: chain_id, revision: 1,
        };
        let mut state = PoseidonTrait::new();
        state = state.update_with('StarkNet Message');
        state = state.update_with(domain.get_struct_hash());
        state = state.update_with(signer);
        state = state.update_with(self.get_struct_hash());
        state.finalize()
    }
}

pub impl StructHashUserIntent of IStructHash<UserIntent> {
    fn get_struct_hash(self: @UserIntent) -> felt252 {
        let mut state = PoseidonTrait::new();
        state = state.update_with(INTENT_TYPE_HASH);
        state = state.update_with(self.salt.get_struct_hash());
        state = state.update_with(*self.maker);
        state = state.update_with(*self.receiver);
        state = state.update_with(*self.maker_asset);
        state = state.update_with(*self.taker_asset);
        state = state.update_with(self.making_amount.get_struct_hash());
        state = state.update_with(self.taking_amount.get_struct_hash());
        state.finalize()
    }
}

pub impl StructHashU256 of IStructHash<u256> {
    fn get_struct_hash(self: @u256) -> felt252 {
        let mut state = PoseidonTrait::new();
        state = state.update_with(U256_TYPE_HASH);
        state = state.update_with(*self);
        state.finalize()
    }
}

pub impl StructHashSpanU32 of IStructHash<Span<u32>> {
    fn get_struct_hash(self: @Span<u32>) -> felt252 {
        let mut state = PoseidonTrait::new();
        for el in (*self) {
            state = state.update_with(*el);
        };
        state.finalize()
    }
}
