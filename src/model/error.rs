use std::io;

use actix_web::{HttpResponse, dev::HttpResponseBuilder, error, http::{StatusCode, header}};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TileError {
    #[error("Invalid data format")]
    InvalidDataFormat(String),

    #[error("Database error")]
    Database(#[from] rusqlite::Error),

    #[error("Connection pool error")]
    ConnPool(#[from] r2d2::Error),

    #[error("IO Error")]
    IOError(#[from] io::Error),

    #[error("Wrong configuration: {0}")]
    Config(String),

    #[error("Unknown tile format: {0}")]
    UnknownTileFormat(String),

    #[error("Missing table: {0}")]
    MissingTable(String)
}

#[derive(Error, Debug)]
pub enum BaseError {
    #[error(transparent)]
    TileData(#[from] TileError)
}

impl error::ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            BaseError::TileData(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type Result<T> = std::result::Result<T, BaseError>;