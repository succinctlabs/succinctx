// List of Succinct Gateway addresses for chains.
const GATEWAY_ADDRESSES: [(u32, &str); 11] = [
    // Mainnet
    (1, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Goerli
    (5, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Sepolia
    (11155111, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Holesky
    (17000, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Gnosis
    (100, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Base
    (8453, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Base Sepolia
    (84532, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Arbitrum
    (42161, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Arbitrum Sepolia
    (421614, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Scroll
    (534352, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
    // Scroll Sepolia
    (534351, "0x6c7a05e0AE641c6559fD76ac56641778B6eCd776"),
];

pub fn get_gateway_address(chain_id: u32) -> Option<&'static str> {
    GATEWAY_ADDRESSES
        .iter()
        .find(|(id, _)| *id == chain_id)
        .map(|(_, addr)| *addr)
}
