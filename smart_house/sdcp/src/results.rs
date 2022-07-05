use crate::{ParamItem, SdcpFrame};
use std::io;
use thiserror::Error;

pub type RequestResult = Result<SdcpFrame, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid packet")]
    InvalidPacket,
    #[error("Invalid frame")]
    InvalidFrame(#[from] FrameError),
}

pub type NetResult = Result<Vec<ParamItem>, NetError>;

#[derive(Debug, Error)]
pub enum NetError {
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error(transparent)]
    Send(#[from] SendError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type ConnectResult<T> = Result<T, ConnError>;

#[derive(Debug, Error)]
pub enum ConnError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type SendResult = Result<(), SendError>;

#[derive(Debug, Error)]
pub enum SendError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type RecvResult = Result<String, RecvError>;

#[derive(Debug, Error)]
pub enum RecvError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("bad encoding")]
    BadEncoding,
    #[error("invalid packet")]
    InvalidPacket,
}

pub type FrameResult = Result<SdcpFrame, FrameError>;

#[derive(Debug, Error)]
pub enum FrameError {
    #[error(transparent)]
    Recv(#[from] RecvError),
    #[error("invalid packet")]
    InvalidPacket,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
