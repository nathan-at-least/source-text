use crate::{LoadSource, Source};

pub fn process<S, F, R>(source: S, f: F) -> anyhow::Result<R>
where
    S: LoadSource,
    F: FnOnce(&Source) -> anyhow::Result<R>,
{
    use anyhow::Context;

    let srcval = source.load()?;
    let name = srcval.name();
    f(&srcval).with_context(|| format!("Error in {}:", name))
}

pub fn process_text<S, F, R>(source: S, f: F) -> anyhow::Result<R>
where
    S: LoadSource,
    F: FnOnce(&str) -> anyhow::Result<R>,
{
    process(source, |s: &Source| f(s.text()))
}
