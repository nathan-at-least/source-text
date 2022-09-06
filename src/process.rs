use crate::{LoadSource, Source};

/// Process any [Source] with a callback, annotating the error with the source's origin `name`.
pub fn process_source<S, F, R>(source: S, f: F) -> anyhow::Result<R>
where
    S: LoadSource,
    F: FnOnce(&Source) -> anyhow::Result<R>,
{
    use anyhow::Context;

    let srcval = source.load()?;
    let name = srcval.name();
    f(&srcval).with_context(|| format!("Error in {}:", name))
}

/// Process any text `&str` with a callback, annotating the error with the source's origin `name`.
pub fn process_text<S, F, R>(source: S, f: F) -> anyhow::Result<R>
where
    S: LoadSource,
    F: FnOnce(&str) -> anyhow::Result<R>,
{
    process_source(source, |s: &Source| f(s.text()))
}
