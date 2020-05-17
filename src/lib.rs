#![warn(missing_debug_implementations)]
#![warn(clippy::result_unwrap_used)]
#![warn(clippy::option_unwrap_used)]

mod parser;
pub use parser::*;

#[cfg(test)]
mod tests;
