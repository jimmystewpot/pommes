#![warn(missing_debug_implementations)]
#![warn(clippy::result_unwrap_used)]
#![warn(clippy::option_unwrap_used)]

pub mod parser;
pub mod simpler;

#[cfg(test)]
mod tests;
