use crate::common::*;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("cannot parse family string '{{0}}'")]
    ParseFamilyStringError(String),
}
