use crate::Source;
use std::path::Path;

/// Any type that can load a [Source] synchronously.
pub trait LoadSource {
    /// Load this value synchronously into a [Source] value.
    fn load(&self) -> anyhow::Result<Source>;
}

impl<'a> LoadSource for &'a str {
    fn load(&self) -> anyhow::Result<Source> {
        Ok(Source::new_unnamed(*self))
    }
}

impl<'a> LoadSource for &'a Path {
    fn load(&self) -> anyhow::Result<Source> {
        use anyhow::Context;

        let text = std::fs::read_to_string(self)
            .with_context(|| format!("Error in {:?}:", self.display()))?;
        Ok(Source::new_named(self.display().to_string(), text))
    }
}
