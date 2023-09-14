use std::sync::Once;
pub mod eth;
pub mod lido;
pub mod poseidon;
pub mod proof;
pub mod serde;
pub mod stream;

use log::LevelFilter;

pub macro bytes32($hex_literal:expr) {
    $hex_literal.parse::<ethers::types::H256>().unwrap()
}

pub macro address($hex_literal:expr) {
    $hex_literal.parse::<ethers::types::Address>().unwrap()
}

pub macro bytes($hex_literal:expr) {{
    let hex_string = $hex_literal;
    let stripped = if let Some(stripped) = hex_string.strip_prefix("0x") {
        stripped
    } else {
        &hex_string
    };
    hex::decode(stripped)
        .expect("Invalid hex string")
        .try_into()
        .expect(&format!(
            "Wrong byte length {} for hex string {}",
            stripped.len(),
            hex_string
        ))
}}

pub macro hex($bytes:expr) {{
    let bytes = $bytes;
    let mut hex_string = String::from("0x");
    hex_string.push_str(&hex::encode(bytes));
    hex_string
}}

pub fn byte_to_bits_be(input: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[7 - i] = (input & (1 << i)) != 0;
    }
    bits
}

static INIT: Once = Once::new();

pub fn setup_logger() {
    INIT.call_once(|| {
        if std::env::args().any(|arg| arg == "--show-output") {
            let mut builder_logger = env_logger::Builder::from_default_env();
            builder_logger.format_timestamp(None);
            builder_logger.filter_level(LevelFilter::Trace);
            builder_logger.init();
        }
    });
}
