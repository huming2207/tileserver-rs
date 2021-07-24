use thiserror::Error;

#[derive(Error, Debug)]
pub enum TileError {
    #[error("Invalid data format")]
    InvalidDataFormat(String),

    #[error("Database error")]
    Database(#[from] rusqlite::Error),

    #[error("Connection pool error")]
    ConnPool(#[from] r2d2::Error),

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

pub type Result<T> = std::result::Result<T, BaseError>;