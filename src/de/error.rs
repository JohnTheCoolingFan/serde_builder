use std::fmt::Debug;

use serde::Deserializer;
use thiserror::Error;

#[derive(Error)]
pub enum Error<'de, D: Deserializer<'de>> {
    Deserialization(D::Error),
    Validation(String),
}

impl<'de, D> Debug for Error<'de, D>
where
    D: Deserializer<'de>,
    D::Error: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deserialization(deerr) => f.debug_tuple("Deserialization").field(&deerr).finish(),
            Self::Validation(vaerr) => f.debug_tuple("Validation").field(&vaerr).finish(),
        }
    }
}
