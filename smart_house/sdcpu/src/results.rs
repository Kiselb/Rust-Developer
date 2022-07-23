use std::io;
use thiserror::Error;

use crate::SdcpuFrame;

pub type FrameResult = Result<SdcpuFrame, FrameError>;

#[derive(Debug, Error)]
pub enum RecvError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("bad encoding")]
    BadEncoding,
}

#[derive(Debug, Error)]
pub enum FrameError {
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error("Invalid frame structure")]
    InvalidStructure,
    #[error("Encoding UTF8 error")]
    EncodingError(#[from] std::str::Utf8Error),
}
