use std::sync::PoisonError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Concurency(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::Concurency("poisoned mutex lock".into())
    }
}
