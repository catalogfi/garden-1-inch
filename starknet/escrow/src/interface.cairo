pub mod sn_domain;
pub mod struct_hash;
pub mod events;
use starknet::ContractAddress;
use crate::escrow::ESCROW::Order;
use core::array::Array;
use core::integer::u256;

// For outbound orders (Starknet -> Something) - requires signature
#[derive(Drop, Serde, Debug)]
pub struct OutboundOrderInput {
    // User intent for signature verification
    pub user_intent: crate::interface::struct_hash::UserIntent,
    pub signature: Array<felt252>,
    
    // ESCROW parameters
    pub token: ContractAddress,
    pub order_hash: felt252,
    pub user_address: ContractAddress,    // who signs (initiator)
    pub resolver_address: ContractAddress, // who executes (redeemer)
    pub timelock: u128,
    pub secret_hash: [u32; 8],
    pub amount: u256
}

// For inbound orders (Something -> Starknet) - no signature needed
#[derive(Drop, Serde, Debug)]
pub struct InboundOrderInput {
    // ESCROW parameters only
    pub token: ContractAddress,
    pub order_hash: felt252,
    pub resolver_address: ContractAddress, // who creates (initiator)
    pub user_address: ContractAddress,     // who receives (redeemer)
    pub timelock: u128,
    pub secret_hash: [u32; 8],
    pub amount: u256
}

#[starknet::interface]
pub trait IESCROW<TContractState> {
    fn get_order(self: @TContractState, token: ContractAddress, order_hash: felt252) -> Order;
    
    fn create_outbound_order(ref self: TContractState, input: OutboundOrderInput);

    fn create_inbound_order(ref self: TContractState, input: InboundOrderInput);

    fn withdraw(ref self: TContractState, token: ContractAddress, order_hash: felt252, secret: Array<u32>);

    fn withdraw_public(ref self: TContractState, token: ContractAddress, order_hash: felt252, secret: Array<u32>);

    fn rescue(ref self: TContractState, token: ContractAddress, order_hash: felt252);

    fn rescue_public(ref self: TContractState, token: ContractAddress, order_hash: felt252);
}

#[starknet::interface]
pub trait IResolver<TContractState> {

    fn create_source(
        ref self: TContractState,
        user_address: ContractAddress,
        resolver_address: ContractAddress,
        user_intent: crate::interface::struct_hash::UserIntent,
        signature: Array<felt252>,
        order_hash: felt252,
        timelock: u128,
        secret_hash: [u32; 8],
        amount: u256
    );

    fn create_destination(
        ref self: TContractState,
        user_address: ContractAddress,
        resolver_address: ContractAddress,
        order_hash: felt252,
        timelock: u128,
        token: ContractAddress,
        secret_hash: [u32; 8],
        amount: u256
    );

    fn withdraw(
        ref self: TContractState,
        token: ContractAddress,
        order_hash: felt252,
        secret: Array<u32>,
    );
}

pub trait IMessageHash<T> {
    fn get_message_hash(self: @T, chain_id: felt252, signer: ContractAddress) -> felt252;
}

pub trait IStructHash<T> {
    fn get_struct_hash(self: @T) -> felt252;
}