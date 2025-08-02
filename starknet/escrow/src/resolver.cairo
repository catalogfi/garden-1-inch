#[starknet::contract]
pub mod Resolver {
    use crate::interface::IESCROWDispatcherTrait;
use starknet::SyscallResultTrait;
use starknet::ContractAddress;
    use starknet::storage::{StoragePointerWriteAccess, StoragePointerReadAccess};
    use core::array::Array;
    use core::integer::u256;
    use core::traits::Into;
    use crate::interface::{OutboundOrderInput, InboundOrderInput};
    use crate::interface::struct_hash::UserIntent;
    use crate::interface::IMessageHash;
    use openzeppelin::account::interface::{ISRC6Dispatcher, ISRC6DispatcherTrait};
    use crate::interface::{IESCROWDispatcher,IResolver};
    use starknet::syscalls::get_execution_info_v2_syscall;

    #[storage]
    struct Storage {
        // ESCROW contract address
        pub escrow_contract: ContractAddress,        
        // Owner/admin of the resolver
        pub owner: ContractAddress,

        pub chain_id: felt252
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        SrcEscrowCreated: SrcEscrowCreated,
        DstEscrowCreated: DstEscrowCreated,
    }

    #[derive(Drop, starknet::Event)]
    pub struct SrcEscrowCreated {
        pub order_hash: felt252,
        pub user_address: ContractAddress,
        pub resolver_address: ContractAddress,
        pub amount: u256,
    }

    #[derive(Drop, starknet::Event)]
    pub struct DstEscrowCreated {
        pub order_hash: felt252,
        pub user_address: ContractAddress,
        pub resolver_address: ContractAddress,
        pub amount: u256,
    }

    #[constructor]
    fn constructor(ref self: ContractState, escrow_contract: ContractAddress) {
        self.escrow_contract.write(escrow_contract);
        self.owner.write(starknet::get_caller_address());
        let tx_info = get_execution_info_v2_syscall().unwrap_syscall().unbox().tx_info.unbox();
        self.chain_id.write(tx_info.chain_id);
    }

    #[abi(embed_v0)]
    pub impl ResolverImpl of IResolver<ContractState> {
        /// @notice Creates a source order (Starknet -> Something)
        /// @dev Called by resolver when user wants to send funds from Starknet to another chain
        /// @param user_address The address of the user who signed the intent
        /// @param user_intent The user's intent for the cross-chain transfer
        /// @param signature The user's signature
        /// @param resolver_address The resolver address (caller)
        /// @param order_hash The order hash for the ESCROW
        /// @param timelock Timelock for the ESCROW
        /// @param amount Amount to transfer
        /// @param secret_hash Hash of the secret for redemption
        fn create_source(
            ref self: ContractState,
            user_address: ContractAddress,
            user_intent: UserIntent,
            signature: Array<felt252>,
            resolver_address: ContractAddress,
            order_hash: felt252,
            timelock: u128,
            amount: u256,
            secret_hash: [u32; 8],
        ) {
            // Verify resolver is authorized
            let caller = starknet::get_caller_address();
            assert!(caller == resolver_address, "Resolver: caller mismatch");

            // Verify user signature
            let chain_id = self.chain_id.read();
            let message_hash = user_intent.get_message_hash(chain_id, user_address);
            
            let is_valid = ISRC6Dispatcher { contract_address: user_address }
                .is_valid_signature(message_hash, signature.clone());
            let is_valid_signature = is_valid == starknet::VALIDATED || is_valid == 1;
            assert!(is_valid_signature, "Resolver: invalid user signature");

            // Create outbound order input
            let outbound_input = OutboundOrderInput {
                user_intent,
                signature,
                token: user_intent.maker_asset,
                order_hash,
                user_address,
                resolver_address,
                timelock,
                amount,
                secret_hash,
            };

            IESCROWDispatcher { contract_address: self.escrow_contract.read() }
                .create_outbound_order(outbound_input);

            self.emit(Event::SrcEscrowCreated(SrcEscrowCreated {
                order_hash,
                user_address,
                resolver_address,
                amount,
            }));
        }

        /// @notice Creates a destination order (Something -> Starknet)
        /// @dev Called by resolver when funds are coming from another chain to Starknet
        /// @param user_address The address that will receive the funds
        /// @param resolver_address The resolver address (caller)
        /// @param order_hash The order hash for the ESCROW
        /// @param timelock Timelock for the ESCROW
        /// @param amount Amount to transfer
        /// @param secret_hash Hash of the secret for redemption
        /// @param token Token address for the order
        fn create_destination(
            ref self: ContractState,
            user_address: ContractAddress,
            resolver_address: ContractAddress,
            order_hash: felt252,
            timelock: u128,
            amount: u256,
            secret_hash: [u32; 8],
            token: ContractAddress,
        ) {
            // Verify resolver is authorized
            let caller = starknet::get_caller_address();
            assert!(caller == resolver_address, "Resolver: caller mismatch");

            // Create inbound order input
            let inbound_input = InboundOrderInput {
                token,
                order_hash,
                resolver_address,
                user_address,
                timelock,
                amount,
                secret_hash,
            };

            IESCROWDispatcher { contract_address: self.escrow_contract.read() }
                .create_inbound_order(inbound_input);

            self.emit(Event::DstEscrowCreated(DstEscrowCreated {
                order_hash,
                user_address,
                resolver_address,
                amount,
            }));
        }
    }
} 