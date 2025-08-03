// Shared configuration for tests
export const STARKNET_DEVNET_URL = "https://starknet-sepolia.public.blastapi.io/rpc/v0_8";

// Prefund accounts from devnet
export const accounts = [
  {
    address: "0x06873a9bbb239716b533f49ccb6775551f329263c3e13838d8f7b4788643983a",
    privateKey: "0x1e4d7e5232bdad0abe3b4e87941f2a6d3dd2620c900751e17339c71878e09b"
  },
  {
    address: "0x014923a0e03ec4f7484f600eab5ecf3e4eacba20ffd92d517b213193ea991502",
    privateKey: "0x00000000000000000000000000000000e5852452e0757e16b127975024ade3eb"
  },
  {
    address: "0x07c3ddf1d8b12ca535493becae82782e537884172a20ffc239b9c859e0280052",
    privateKey: "0x014b647de5269b2e0069f3c1ef93c1c8e64ae8d842181df21afb0b32b3db081a"
  },
];

// Token addresses
export const STARK = "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d";
export const ETH = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
export const ZERO_ADDRESS = "0x000000000000000000000000000000000000000000000000000000000000000";

// Test constants
export const TIMELOCK = 10n;
export const AMOUNT = "100000000000000000"; // 0.1 ETH in wei 