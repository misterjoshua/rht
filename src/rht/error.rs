use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Clone, Serialize, Deserialize, Debug)]
pub enum RhtError {
    #[error("{0}")]
    Error(String),
}
