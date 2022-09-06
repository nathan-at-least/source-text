use crate::LoadSource;

pub trait Parsable: Sized {
    fn parse_text(text: &str) -> anyhow::Result<Self>;

    fn parse_source<S>(source: S) -> anyhow::Result<Self>
        where S: LoadSource,
    {
        crate::process_text(source, Self::parse_text)
    }
}
