export const ABI = [
  {
    "type": "impl",
    "name": "ESCROW",
    "interface_name": "escrow::interface::IESCROW"
  },
  {
    "type": "enum",
    "name": "core::bool",
    "variants": [
      {
        "name": "False",
        "type": "()"
      },
      {
        "name": "True",
        "type": "()"
      }
    ]
  },
  {
    "type": "struct",
    "name": "core::integer::u256",
    "members": [
      {
        "name": "low",
        "type": "core::integer::u128"
      },
      {
        "name": "high",
        "type": "core::integer::u128"
      }
    ]
  },
  {
    "type": "struct",
    "name": "escrow::escrow::ESCROW::Order",
    "members": [
      {
        "name": "is_fulfilled",
        "type": "core::bool"
      },
      {
        "name": "initiator",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "redeemer",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "initiated_at",
        "type": "core::integer::u128"
      },
      {
        "name": "timelock",
        "type": "core::integer::u128"
      },
      {
        "name": "amount",
        "type": "core::integer::u256"
      },
      {
        "name": "security_deposit_paid",
        "type": "core::bool"
      },
      {
        "name": "secret_hash",
        "type": "[core::integer::u32; 8]"
      },
      {
        "name": "token",
        "type": "core::starknet::contract_address::ContractAddress"
      }
    ]
  },
  {
    "type": "struct",
    "name": "escrow::interface::struct_hash::UserIntent",
    "members": [
      {
        "name": "salt",
        "type": "core::integer::u256"
      },
      {
        "name": "maker",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "receiver",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "maker_asset",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "taker_asset",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "making_amount",
        "type": "core::integer::u256"
      },
      {
        "name": "taking_amount",
        "type": "core::integer::u256"
      }
    ]
  },
  {
    "type": "struct",
    "name": "escrow::interface::OutboundOrderInput",
    "members": [
      {
        "name": "user_intent",
        "type": "escrow::interface::struct_hash::UserIntent"
      },
      {
        "name": "signature",
        "type": "core::array::Array::<core::felt252>"
      },
      {
        "name": "token",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "order_hash",
        "type": "core::felt252"
      },
      {
        "name": "user_address",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "resolver_address",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "timelock",
        "type": "core::integer::u128"
      },
      {
        "name": "secret_hash",
        "type": "[core::integer::u32; 8]"
      },
      {
        "name": "amount",
        "type": "core::integer::u256"
      }
    ]
  },
  {
    "type": "struct",
    "name": "escrow::interface::InboundOrderInput",
    "members": [
      {
        "name": "token",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "order_hash",
        "type": "core::felt252"
      },
      {
        "name": "resolver_address",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "user_address",
        "type": "core::starknet::contract_address::ContractAddress"
      },
      {
        "name": "timelock",
        "type": "core::integer::u128"
      },
      {
        "name": "secret_hash",
        "type": "[core::integer::u32; 8]"
      },
      {
        "name": "amount",
        "type": "core::integer::u256"
      }
    ]
  },
  {
    "type": "interface",
    "name": "escrow::interface::IESCROW",
    "items": [
      {
        "type": "function",
        "name": "get_order",
        "inputs": [
          {
            "name": "token",
            "type": "core::starknet::contract_address::ContractAddress"
          },
          {
            "name": "order_hash",
            "type": "core::felt252"
          }
        ],
        "outputs": [
          {
            "type": "escrow::escrow::ESCROW::Order"
          }
        ],
        "state_mutability": "view"
      },
      {
        "type": "function",
        "name": "transfer_funds",
        "inputs": [
          {
            "name": "token",
            "type": "core::starknet::contract_address::ContractAddress"
          },
          {
            "name": "amount",
            "type": "core::integer::u256"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "verify_signature",
        "inputs": [
          {
            "name": "token",
            "type": "core::starknet::contract_address::ContractAddress"
          },
          {
            "name": "amount",
            "type": "core::integer::u256"
          },
          {
            "name": "signature",
            "type": "core::array::Array::<core::felt252>"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "create_outbound_order",
        "inputs": [
          {
            "name": "input",
            "type": "escrow::interface::OutboundOrderInput"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "create_inbound_order",
        "inputs": [
          {
            "name": "input",
            "type": "escrow::interface::InboundOrderInput"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "withdraw",
        "inputs": [
          {
            "name": "token",
            "type": "core::starknet::contract_address::ContractAddress"
          },
          {
            "name": "order_hash",
            "type": "core::felt252"
          },
          {
            "name": "secret",
            "type": "core::array::Array::<core::integer::u32>"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "withdraw_public",
        "inputs": [
          {
            "name": "token",
            "type": "core::starknet::contract_address::ContractAddress"
          },
          {
            "name": "order_hash",
            "type": "core::felt252"
          },
          {
            "name": "secret",
            "type": "core::array::Array::<core::integer::u32>"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "rescue",
        "inputs": [
          {
            "name": "token",
            "type": "core::starknet::contract_address::ContractAddress"
          },
          {
            "name": "order_hash",
            "type": "core::felt252"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      },
      {
        "type": "function",
        "name": "rescue_public",
        "inputs": [
          {
            "name": "token",
            "type": "core::starknet::contract_address::ContractAddress"
          },
          {
            "name": "order_hash",
            "type": "core::felt252"
          }
        ],
        "outputs": [],
        "state_mutability": "external"
      }
    ]
  },
  {
    "type": "constructor",
    "name": "constructor",
    "inputs": []
  },
  {
    "type": "event",
    "name": "escrow::interface::events::Created",
    "kind": "struct",
    "members": [
      {
        "name": "order_hash",
        "type": "core::felt252",
        "kind": "key"
      },
      {
        "name": "secret_hash",
        "type": "[core::integer::u32; 8]",
        "kind": "data"
      },
      {
        "name": "amount",
        "type": "core::integer::u256",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "escrow::interface::events::Withdraw",
    "kind": "struct",
    "members": [
      {
        "name": "order_hash",
        "type": "core::felt252",
        "kind": "key"
      },
      {
        "name": "secret_hash",
        "type": "[core::integer::u32; 8]",
        "kind": "data"
      },
      {
        "name": "secret",
        "type": "core::array::Array::<core::integer::u32>",
        "kind": "data"
      },
      {
        "name": "is_public",
        "type": "core::bool",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "escrow::interface::events::Rescue",
    "kind": "struct",
    "members": [
      {
        "name": "order_hash",
        "type": "core::felt252",
        "kind": "key"
      },
      {
        "name": "is_public",
        "type": "core::bool",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "escrow::interface::events::TransferFunds",
    "kind": "struct",
    "members": [
      {
        "name": "token",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "key"
      },
      {
        "name": "amount",
        "type": "core::integer::u256",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "escrow::interface::events::SignatureVerified",
    "kind": "struct",
    "members": [
      {
        "name": "token",
        "type": "core::starknet::contract_address::ContractAddress",
        "kind": "data"
      },
      {
        "name": "amount",
        "type": "core::integer::u256",
        "kind": "data"
      }
    ]
  },
  {
    "type": "event",
    "name": "escrow::escrow::ESCROW::Event",
    "kind": "enum",
    "variants": [
      {
        "name": "Created",
        "type": "escrow::interface::events::Created",
        "kind": "nested"
      },
      {
        "name": "Withdraw",
        "type": "escrow::interface::events::Withdraw",
        "kind": "nested"
      },
      {
        "name": "Rescue",
        "type": "escrow::interface::events::Rescue",
        "kind": "nested"
      },
      {
        "name": "TransferFunds",
        "type": "escrow::interface::events::TransferFunds",
        "kind": "nested"
      },
      {
        "name": "SignatureVerified",
        "type": "escrow::interface::events::SignatureVerified",
        "kind": "nested"
      }
    ]
  }
] as const;
