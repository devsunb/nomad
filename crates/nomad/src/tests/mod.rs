//! Utilities for testing.

mod emoji;
mod generate;
mod generator;
mod text;

pub use emoji::Emoji;
pub use generate::*;
pub use generator::Generator;
pub use text::Text;

/// Result value for tests.
pub type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Creates a random seed.
pub fn random_seed() -> u64 {
    use rand::Rng;
    rand::thread_rng().gen()
}
