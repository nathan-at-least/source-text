use crate::OwnedSource;
use std::borrow::Cow;

/// A [Source] owns or refers to a `text` string with an optional `name` denoting the origin.
///
/// The `name` and `text` are both `Cow<'a, str>` allowing a [Source] to either own or refer to
/// these.
#[derive(Debug)]
pub struct Source<'a> {
    name: Option<Cow<'a, str>>,
    text: Cow<'a, str>,
}

impl<'a> Source<'a> {
    /// Create a new [Source] with an optional origin name. Example: `Source::new(Some("<built-in>"), "my text")`
    pub fn new<N, T>(optname: Option<N>, text: T) -> Self
    where
        Cow<'a, str>: From<N>,
        Cow<'a, str>: From<T>,
    {
        Source {
            name: optname.map(|n| Cow::from(n)),
            text: Cow::from(text),
        }
    }

    /// Create a new [Source] with a given origin name. Example: `Source::new_named("<built-in>", "my text")`
    pub fn new_named<N, T>(name: N, text: T) -> Self
    where
        Cow<'a, str>: From<N>,
        Cow<'a, str>: From<T>,
    {
        Source::new(Some(name), text)
    }

    /// Create a new [Source] without an origin name. Example: `Source::new_unnamed("my text")`
    pub fn new_unnamed<T>(text: T) -> Self
    where
        Cow<'a, str>: From<T>,
    {
        Source::new(None, text)
    }

    /// Borrow the name of this [Source], which if absent defaults to `"<string>"`.
    pub fn name(&self) -> &str {
        use crate::optname_to_str;

        optname_to_str(self.name.as_ref().map(|cow| cow.as_ref()))
    }

    /// Borrow the text of this [Source].
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }
}

impl<'a> From<Source<'a>> for OwnedSource {
    fn from(s: Source<'a>) -> OwnedSource {
        OwnedSource::new(s.name, s.text)
    }
}

impl<'a> From<OwnedSource> for Source<'a> {
    fn from(s: OwnedSource) -> Source<'a> {
        let (optname, text) = s.unwrap();
        Source::new(optname, text)
    }
}
