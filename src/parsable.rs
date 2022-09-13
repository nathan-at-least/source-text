use crate::LoadSource;

/// A type that can be parsed from a text `&str`; this provides a `parse_source` default method for
/// parsing any `LoadSource` value such as `&str` or `&Path`:
pub trait Parsable: Sized {
    /// Parse this type from a text `&str`.
    fn parse_text(text: &str) -> anyhow::Result<Self>;

    /// Parse this type from any [LoadSource] input.
    ///
    /// The default implementation loads the source and delegates to [Parsable::parse_text].
    fn parse_source<S>(source: S) -> anyhow::Result<Self>
    where
        S: LoadSource,
    {
        crate::process_text(source, Self::parse_text)
    }
}
