

use super::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmitterError {
    #[error("processing error: `{0}`")]
    Processing(String),
    #[error("emitting error: `{0}`")]
    Emitting(String),
    #[error("packing error: `{0}`")]
    Packing(String),
}