#[starknet::contract]
pub mod ESCROW {
    use starknet::SyscallResultTrait;
use core::num::traits::Zero;
    use starknet::{ContractAddress, get_block_info, get_contract_address};
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerWriteAccess,
        StoragePointerReadAccess
    };
    use core::traits::Into;
    use core::sha256::compute_sha256_u32_array;
    use openzeppelin::token::erc20::interface::{IERC20Dispatcher, IERC20DispatcherTrait};
    use openzeppelin::account::interface::{ISRC6Dispatcher, ISRC6DispatcherTrait};
    use crate::interface::{IESCROW, IMessageHash};
    use crate::interface::struct_hash::{MessageHashUserIntent};
    use crate::interface::events::{Created, Withdraw, Rescue};
    use starknet::event::EventEmitter;
    use starknet::syscalls::get_execution_info_v2_syscall;
    use core::array::Array;
    use core::integer::u256;


    pub const NAME: felt252 = 'ESCROW';
    pub const VERSION: felt252 = '1';
    pub const SECURITY_DEPOSIT: u256 = u256 { low: 1000000000000000000, high: 0 }; // 1 token
    pub const WITHDRAW_TIMELOCK: u128 = 10;
    pub const RESCUE_TIMELOCK: u128 = 240;

    pub const INTENT_TYPE_HASH: felt252 = selector!(
        "\"UserIntent\"(\"salt\":\"u256\",\"maker\":\"ContractAddress\",\"receiver\":\"ContractAddress\",\"maker_asset\":\"ContractAddress\",\"taker_asset\":\"ContractAddress\",\"making_amount\":\"u256\",\"taking_amount\":\"u256\")\"u256\"(\"low\":\"u128\",\"high\":\"u128\")",
    );
    pub const U256_TYPE_HASH: felt252 = selector!("\"u256\"(\"low\":\"u128\",\"high\":\"u128\")");

    #[storage]
    struct Storage {
        pub orders: Map::<(ContractAddress, felt252), Order>,
        pub chain_id: felt252,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        Created: Created,
        Withdraw: Withdraw,
        Rescue: Rescue
    }

    #[derive(Drop, Serde, starknet::Store, Debug)]
    pub struct Order {
        // ESCROW-specific storage fields
        is_fulfilled: bool,
        initiator: ContractAddress,
        redeemer: ContractAddress,
        initiated_at: u128,
        timelock: u128,
        amount: u256,
        security_deposit_paid: bool,
        secret_hash: [u32; 8],
        token: ContractAddress,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {
        let tx_info = get_execution_info_v2_syscall().unwrap_syscall().unbox().tx_info.unbox();
        self.chain_id.write(tx_info.chain_id);
    }

    #[abi(embed_v0)]
    pub impl ESCROW of IESCROW<ContractState> {
        fn get_order(self: @ContractState, token: ContractAddress, order_hash: felt252) -> Order {
            self.orders.read((token, order_hash))
        }
        /// @notice  Creates an outbound order (Starknet -> Something) with user signature verification.
        /// @dev     User must sign the UserIntent, resolver will execute the order.
        /// @param   input  OutboundOrderInput struct containing user intent and ESCROW parameters.
        fn create_outbound_order(ref self: ContractState, input: crate::interface::OutboundOrderInput) {
            self.safe_params(input.resolver_address, input.timelock, input.amount);

            let chain_id = self.chain_id.read();
            let message_hash = input.user_intent.get_message_hash(chain_id, input.user_address);

            let is_valid = ISRC6Dispatcher { contract_address: input.user_address }
                .is_valid_signature(message_hash, input.signature);
            let is_valid_signature = is_valid == starknet::VALIDATED || is_valid == 1;
            assert!(is_valid_signature, "ESCROW: invalid user signature");

            self._initiate(
                input.token, 
                input.order_hash, 
                input.user_address, 
                input.user_address, 
                input.resolver_address, 
                input.timelock, 
                input.amount, 
                input.secret_hash
            );
        }

        /// @notice  Creates an inbound order (Something -> Starknet) without signature verification.
        /// @dev     Resolver creates the order, user will receive the funds.
        /// @param   input  InboundOrderInput struct containing ESCROW parameters.
        fn create_inbound_order(ref self: ContractState, input: crate::interface::InboundOrderInput) {
            self.safe_params(input.user_address, input.timelock, input.amount);
            
            self._initiate(
                input.token, 
                input.order_hash, 
                input.resolver_address, 
                input.resolver_address, 
                input.user_address, 
                input.timelock, 
                input.amount, 
                input.secret_hash
            );
        }

        /// @notice  Redeemer can withdraw funds with correct secret within timelock.
        /// @dev     Only the redeemer can call this function.
        /// @param   token  Contract address of the token to withdraw.
        /// @param   order_hash  Order hash of the ESCROW order.
        /// @param   secret  Secret used to redeem an order.
        fn withdraw(ref self: ContractState, token: ContractAddress, order_hash: felt252, secret: Array<u32>) {
            let order = self.orders.read((token, order_hash));
            assert!(order.redeemer.is_non_zero(), "ESCROW: order not initiated");
            assert!(!order.is_fulfilled, "ESCROW: order fulfilled");

            // Verify that the provided secret matches the stored secret hash
            let computed_secret_hash = compute_sha256_u32_array(secret.clone(), 0, 0);
            assert!(computed_secret_hash == order.secret_hash, "ESCROW: incorrect secret");

            self.orders.write((token, order_hash), Order { is_fulfilled: true, ..order });

            // Transfer amount + security deposit to redeemer
            let total_amount = order.amount + SECURITY_DEPOSIT;
            let token_dispatcher = IERC20Dispatcher { contract_address: token };
            token_dispatcher.transfer(order.redeemer, total_amount);
            self.emit(Event::Withdraw(Withdraw { order_hash, secret_hash: computed_secret_hash, secret, is_public: false }));
        }

        /// @notice  Anyone can withdraw funds with correct secret within timelock.
        /// @dev     Security deposit goes to the caller.
        ///          Can only be called after WITHDRAW_TIMELOCK blocks have passed since order creation.
        /// @param   token  Contract address of the token to withdraw.
        /// @param   order_hash  Order hash of the ESCROW order.
        /// @param   secret  Secret used to redeem an order.
        fn withdraw_public(ref self: ContractState, token: ContractAddress, order_hash: felt252, secret: Array<u32>) {
            let order = self.orders.read((token, order_hash));
            assert!(order.redeemer.is_non_zero(), "ESCROW: order not initiated");
            assert!(!order.is_fulfilled, "ESCROW: order fulfilled");

            // Check that enough time has passed for public access
            let block_info = get_block_info().unbox();
            let current_block = block_info.block_number;
            assert!(
                (order.initiated_at + WITHDRAW_TIMELOCK) <= current_block.into(),
                "ESCROW: public access not yet available",
            );

            // Verify that the provided secret matches the stored secret hash
            let computed_secret_hash = compute_sha256_u32_array(secret.clone(), 0, 0);
            assert!(computed_secret_hash == order.secret_hash, "ESCROW: incorrect secret");

            self.orders.write((token, order_hash), Order { is_fulfilled: true, ..order });

            let caller = starknet::get_caller_address();
            let token_dispatcher = IERC20Dispatcher { contract_address: token };
            
            // Transfer amount to redeemer, security deposit to caller
            token_dispatcher.transfer(order.redeemer, order.amount);
            token_dispatcher.transfer(caller, SECURITY_DEPOSIT);
            self.emit(Event::Withdraw(Withdraw { order_hash, secret_hash: computed_secret_hash, secret, is_public: true }));
        }

        /// @notice  Redeemer can rescue funds after timelock expires.
        /// @dev     Only the redeemer can call this function.
        /// @param   token  Contract address of the token to rescue.
        /// @param   order_hash  Order hash of the ESCROW order.
        fn rescue(ref self: ContractState, token: ContractAddress, order_hash: felt252) {
            let order = self.orders.read((token, order_hash));

            assert!(order.redeemer.is_non_zero(), "ESCROW: order not initiated");
            assert!(!order.is_fulfilled, "ESCROW: order fulfilled");

            let block_info = get_block_info().unbox();
            let current_block = block_info.block_number;
            assert!(
                (order.initiated_at + order.timelock) < current_block.into(),
                "ESCROW: order not expired",
            );

            self.orders.write((token, order_hash), Order { is_fulfilled: true, ..order });

            // Transfer amount + security deposit to initiator
            let total_amount = order.amount + SECURITY_DEPOSIT;
            let token_dispatcher = IERC20Dispatcher { contract_address: token };
            token_dispatcher.transfer(order.initiator, total_amount);

            self.emit(Event::Rescue(Rescue { order_hash, is_public: false }));
        }

        /// @notice  Anyone can rescue funds after timelock expires.
        /// @dev     Security deposit goes to the caller.
        ///          Can only be called after RESCUE_TIMELOCK blocks have passed since order creation.
        /// @param   token  Contract address of the token to rescue.
        /// @param   order_hash  Order hash of the ESCROW order.
        fn rescue_public(ref self: ContractState, token: ContractAddress, order_hash: felt252) {
            let order = self.orders.read((token, order_hash));

            assert!(order.redeemer.is_non_zero(), "ESCROW: order not initiated");
            assert!(!order.is_fulfilled, "ESCROW: order fulfilled");

            let block_info = get_block_info().unbox();
            let current_block = block_info.block_number;
            
            // Check that order timelock has expired
            assert!(
                (order.initiated_at + order.timelock) < current_block.into(),
                "ESCROW: order not expired",
            );
            
            // Check that enough time has passed for public rescue access
            assert!(
                (order.initiated_at + RESCUE_TIMELOCK) <= current_block.into(),
                "ESCROW: public rescue not yet available",
            );

            self.orders.write((token, order_hash), Order { is_fulfilled: true, ..order });

            let caller = starknet::get_caller_address();
            let token_dispatcher = IERC20Dispatcher { contract_address: token };
            
            // Transfer amount to initiator, security deposit to caller
            token_dispatcher.transfer(order.initiator, order.amount);
            token_dispatcher.transfer(caller, SECURITY_DEPOSIT);

            self.emit(Event::Rescue(Rescue { order_hash, is_public: true }));
        }
    }

    #[generate_trait]
    pub impl InternalFunctions of InternalFunctionsTrait {
        /// @notice  Internal function to initiate an order for an atomic swap.
        /// @dev     This function is called internally to create a new order for an atomic swap.
        ///          It checks that the initiator and redeemer addresses are different and that
        ///          there is no duplicate order.
        ///          It creates a new order with the provided parameters and stores it in the
        ///          'orders' mapping.
        ///          It emits a 'Created' event with the order hash, secret hash, and amount.
        ///          It transfers the specified amount of tokens plus security deposit from the initiator to the contract
        ///          address.
        /// @param   token  Contract address of the token to trade.
        /// @param   order_hash  Order hash for the ESCROW order.
        /// @param   funder_  Address of the funder of the atomic swap.
        /// @param   initiator_  Address of the initiator of the atomic swap.
        /// @param   redeemer_  Address of the redeemer of the atomic swap.
        /// @param   secret_hash_  Hash of the secret used for redemption.
        /// @param   timelock_  Timelock block number for the atomic swap.
        /// @param   amount_  Amount of tokens to be traded in the atomic swap.
        fn _initiate(
            ref self: ContractState,
            token: ContractAddress,
            order_hash: felt252,
            funder_: ContractAddress,
            initiator_: ContractAddress,
            redeemer_: ContractAddress,
            timelock_: u128,
            amount_: u256,
            secret_hash_: [u32; 8],
        ) {
            assert!(initiator_ != redeemer_, "ESCROW: same initiator & redeemer");

            let order: Order = self.orders.read((token, order_hash));
            assert!(!order.redeemer.is_non_zero(), "ESCROW: duplicate order");

            let block_info = get_block_info().unbox();
            let current_block = block_info.block_number;

            let create_order = Order {
                is_fulfilled: false,
                initiator: initiator_,
                redeemer: redeemer_,
                initiated_at: current_block.into(),
                timelock: timelock_,
                amount: amount_,
                security_deposit_paid: true,
                secret_hash: secret_hash_,
                token: token,
            };
            self.orders.write((token, order_hash), create_order);

            // let token_dispatcher = IERC20Dispatcher { contract_address: token };
            // let balance = token_dispatcher.balance_of(funder_);
            // assert!(balance >= amount_, "ERC20: Insufficient balance");

            // let allowance = token_dispatcher.allowance(funder_, get_contract_address());
            // assert!(allowance >= amount_, "ERC20: insufficient allowance");

            // Transfer amount + security deposit from funder to contract

            let total_amount = amount_;
            let token_dispatcher = IERC20Dispatcher { contract_address: token };
            let transfer_result = token_dispatcher.transfer_from(funder_, get_contract_address(), total_amount);
            assert!(transfer_result, "ERC20: Transfer failed");

            self
                .emit(
                    Event::Created(
                        Created { order_hash, secret_hash: secret_hash_, amount: amount_ },
                    ),
                );
        }
    }

    #[generate_trait]
    impl AssertsImpl of AssertsTrait {
        /// @notice  .
        /// @dev     Provides checks to ensure:
        ///              1. Redeemer is not the null address.
        ///              3. Timelock is greater than 0.
        ///              4. Amount is not zero.
        /// @param   redeemer  Contract address of the redeemer.
        /// @param   timelock  Timelock period for the ESCROW order.
        /// @param   amount  Amount of tokens to trade.
        #[inline]
        fn safe_params(
            self: @ContractState, redeemer: ContractAddress, timelock: u128, amount: u256,
        ) {
            assert!(redeemer.is_non_zero(), "ESCROW: zero address redeemer");
            assert!(timelock > 0, "ESCROW: zero timelock");
            assert!(amount > 0, "ESCROW: zero amount");
        }
    }
}