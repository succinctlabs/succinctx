use std::sync::Once;
pub mod eth;
pub mod lido;
pub mod poseidon;
pub mod proof;
pub mod reqwest;
pub mod serde;
pub mod stream;
use std::sync::atomic::{AtomicUsize, Ordering};

use log::{set_max_level, LevelFilter};

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

pub fn to_be_bits(msg: &[u8]) -> Vec<bool> {
    let mut res = Vec::new();
    msg.iter().for_each(|char| {
        for j in 0..8 {
            if (char & (1 << (7 - j))) != 0 {
                res.push(true);
            } else {
                res.push(false);
            }
        }
    });
    res
}

static INIT: Once = Once::new();

pub fn setup_logger() {
    INIT.call_once(|| {
        env_logger::Builder::from_default_env()
            .format_timestamp(None)
            .filter_level(LevelFilter::Trace)
            .init();
    });
}

static ORIGINAL_LEVEL: AtomicUsize = AtomicUsize::new(LevelFilter::Info as usize);

pub fn disable_logging() {
    let current_level = log::max_level() as usize;
    ORIGINAL_LEVEL.store(current_level, Ordering::SeqCst);
    set_max_level(LevelFilter::Off);
}

pub fn enable_logging() {
    let original_level = ORIGINAL_LEVEL.load(Ordering::SeqCst);
    set_max_level(unsafe { std::mem::transmute(original_level) });
}
