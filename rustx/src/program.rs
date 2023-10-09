use core::fmt::Debug;

/// A trait for a program that can be verified on-chain via a trusted relayer.
pub trait Program: Debug + Clone {
    /// The logic of the function in the form `f(inputBytes) -> outputBytes`.
    fn run(input_bytes: Vec<u8>) -> Vec<u8>;

    /// The address of the trusted relayer. Inside the contract, we verify that
    /// `tx.origin == tx_origin`.
    fn tx_origin() -> String {
        "0xDEd0000E32f8F40414d3ab3a830f735a3553E18e".to_string()
    }
}
