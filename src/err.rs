use thiserror::Error;

#[derive(Error, Debug)]
pub enum FB2Error {
    #[error("failed to open FB2 file")]
    OpenFailed,
}
