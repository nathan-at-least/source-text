//! Enable API's that consume `&str` to retrieve the string from any `LoadSource` source while tracking the origin.
//!
//! # Example
//!
//! Suppose we have a config file parser and want to be able to either parse strings in memory or
//! load and parse config files from disk, with error messages indicating the source:
//!
//! ```
//! use indoc::indoc; // For cleanly formatting test assertions.
//! use source_text::Parsable;
//!
//! #[derive(Debug)]
//! pub struct Config {
//!     // ...
//! }
//!
//! impl Parsable for Config {
//!     fn parse_text(text: &str) -> anyhow::Result<Self> {
//!         if text.is_empty() {
//!             Err(anyhow::Error::msg("empty input\n"))
//!         } else {
//!             todo!("implement an `&str` parser...");
//!         }
//!     }
//! }
//!
//! fn parse_empty_config() -> anyhow::Result<()> {
//!     let config = Config::parse_source("")?;
//!     panic!("Unexpectedly parsed config: {:?}", config);
//! }
//!
//! fn parse_non_existent_file() -> anyhow::Result<()> {
//!     let configpath = std::path::Path::new("/__this_path_should_not_exist__");
//!     let config = Config::parse_source(configpath)?;
//!     panic!("Unexpectedly parsed config: {:?}", config);
//! }
//!
//! fn main() {
//!     let err1 = parse_empty_config().err().unwrap();
//!
//!     assert_eq!(
//!         format!("{:?}", err1).trim_end(),
//!         indoc! {
//!         r#"
//!            Error in <string>:
//!
//!            Caused by:
//!                empty input
//!         "#
//!         }.trim_end()
//!     );
//!
//!     let err2 = parse_non_existent_file().err().unwrap();
//!
//!     assert_eq!(
//!         format!("{:?}", err2).trim_end(),
//!         indoc! { r#"
//!             Error in "/__this_path_should_not_exist__":
//!
//!             Caused by:
//!                 No such file or directory (os error 2)
//!         "# }.trim_end(),
//!     );
//! }
//! ```

mod load;
mod process;
mod source;
mod parsable;

pub use self::load::LoadSource;
pub use self::process::{process, process_text};
pub use self::source::Source;
pub use self::parsable::Parsable;
