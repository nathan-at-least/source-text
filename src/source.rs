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
    pub fn new_named<N, T>(name: N, text: T) -> Self
    where
        Cow<'a, str>: From<N>,
        Cow<'a, str>: From<T>,
    {
        Source {
            name: Some(Cow::from(name)),
            text: Cow::from(text),
        }
    }

    pub fn new_unnamed<T>(text: T) -> Self
    where
        Cow<'a, str>: From<T>,
    {
        Source {
            name: None,
            text: Cow::from(text),
        }
    }
    pub fn name(&self) -> &str {
        self.name
            .as_ref()
            .map(|cow| cow.as_ref())
            .unwrap_or("<string>")
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }
}
