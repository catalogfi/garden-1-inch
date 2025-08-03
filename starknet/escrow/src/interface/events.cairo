#[derive(Drop, starknet::Event)]
pub struct Created {
    #[key]
    pub order_hash: felt252,
    pub secret_hash: [u32; 8],
    pub amount: u256,
}

#[derive(Drop, starknet::Event)]
pub struct Withdraw {
    #[key]
    pub order_hash: felt252,
    pub secret_hash: [u32; 8],
    pub secret: Array<u32>,
    pub is_public: bool,
}

#[derive(Drop, starknet::Event)]
pub struct Rescue {
    #[key]
    pub order_hash: felt252,
    pub is_public: bool,
}