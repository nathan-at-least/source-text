//! Enable API's that consume `&str` to retrieve the text string from any `LoadSource` source while tracking the origin.
//!
//! Terminology:
//!
//! This crate uses some terminology to help in disambiguation:
//!
//! - "text" - an `&str` to be "processed" by application code; in contrast to `&str` used for
//!   other purposes such as naming the origin of a "text" or used in error messages.
//! - "source" - a [Source] value which tracks both a `text` and the `name` of its origin.
//!
//! # Example: [Parsable]
//!
//! Suppose we have a config file parser which parses a configuration into a `Config` struct. We
//! want to be able to either parse strings in memory or load and parse config files from disk,
//! with error messages indicating the source:
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
//! fn main() {
//!     let err1 = Config::parse_source("").err().unwrap();
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
//!     let configpath = std::path::Path::new("/__this_path_should_not_exist__");
//!     let err2 = Config::parse_source(configpath).err().unwrap();
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
//!
//! # Example: `process_text`
//!
//! If [Parsable] does not fit your usage, you can wrap any `fn(&str) -> anyhow::Result<T>` with
//! [process_text]:
//!
//! ```
//! use indoc::indoc; // For cleanly formatting test assertions.
//!
//! fn count_lines<S>(source: S) -> anyhow::Result<usize>
//!     where S: source_text::LoadSource,
//! {
//!     source_text::process_text(source, |text: &str| {
//!         Ok(text.lines().count())
//!     })
//! }
//!
//! fn main() {
//!     let linecount = count_lines("Hello\nWorld!").unwrap();
//!     assert_eq!(2, linecount);
//!
//!     let path = std::path::Path::new("/__this_path_should_not_exist__");
//!     let err = count_lines(path).err().unwrap();
//!
//!     assert_eq!(
//!         format!("{:?}", err).trim_end(),
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
pub use self::process::{process_source, process_text};
pub use self::source::Source;
pub use self::parsable::Parsable;
