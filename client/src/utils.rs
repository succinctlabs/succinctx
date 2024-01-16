// List of Succinct Gateway addresses for chains.
const GATEWAY_ADDRESSES: [(u32, &str); 6] = [
    (1, "0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803"),
    (5, "0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803"),
    (100, "0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803"),
    (420, "0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803"),
    (17000, "0x6e4f1e9ea315ebfd69d18c2db974eef6105fb803"),
    (11155111, "0xaea9288f0b7a8c605c4d474c56e5e74f96bfd4b3"),
];

pub fn get_gateway_address(chain_id: u32) -> Option<&'static str> {
    GATEWAY_ADDRESSES
        .iter()
        .find(|(id, _)| *id == chain_id)
        .map(|(_, addr)| *addr)
}
