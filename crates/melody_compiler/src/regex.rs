pub mod ast_to_regex;
// This table is a standalone reference for a possible escape preflight.
mod escape_cache;
mod utils;

pub use self::ast_to_regex::ast_to_regex;
