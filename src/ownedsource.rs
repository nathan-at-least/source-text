use crate::Source;

#[derive(Debug)]
pub struct OwnedSource {
    name: Option<String>,
    text: String,
}

/// An [OwnedSource] owns a `text` string with an optional `name` denoting the origin.
impl OwnedSource {
    /// Create a new [OwnedSource] with an optional origin name. Example: `OwnedSource::new(Some("<built-in>"), "my text")`
    pub fn new<N, T>(optname: Option<N>, text: T) -> Self
    where
        String: From<N>,
        String: From<T>,
    {
        OwnedSource {
            name: optname.map(|n| String::from(n)),
            text: String::from(text),
        }
    }

    /// Create a new [OwnedSource] with a given origin name. Example: `OwnedSource::new_named("<built-in>", "my text")`
    pub fn new_named<N, T>(name: N, text: T) -> Self
    where
        String: From<N>,
        String: From<T>,
    {
        OwnedSource::new(Some(name), text)
    }

    /// Create a new [OwnedSource] without an origin name. Example: `Source::new_unnamed("my text")`
    pub fn new_unnamed<T>(text: T) -> Self
    where
        String: From<T>,
    {
        OwnedSource::new(None, text)
    }

    /// Borrow the name, which if absent defaults to `"<string>"`.
    pub fn name(&self) -> &str {
        use crate::optname_to_str;

        optname_to_str(self.name.as_ref().map(|s| s.as_ref()))
    }

    /// Borrow the text.
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    /// Return a [Source] referring to `self`'s contents.
    pub fn source(&self) -> Source {
        Source::new(self.name.as_ref(), self.text())
    }

    /// Unbundle the optional `name` and `text` to take direct ownership.
    pub fn unwrap(self) -> (Option<String>, String) {
        (self.name, self.text)
    }
}
