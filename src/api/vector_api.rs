use std::io::Read;

use actix_web::{HttpResponse, get, web};
use flate2::read::GzDecoder;

use crate::{model::{error::{Result, BaseError, TileError}, tile::get_tile_data}};

#[get("/{zoom}/{x}/{y}.pbf")]
pub async fn serve_vector(req: web::Path<(String, String, String)>) -> Result<HttpResponse> {
    let req_inner = req.into_inner();
    let zoom = req_inner.0;
    let x = req_inner.1;
    let y = req_inner.2;

    let tile = get_tile_data(&zoom, &x, &y)?;
    let mut decoder = GzDecoder::new(&tile[..]);
    let mut tile_unzipped: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut tile_unzipped).unwrap();


    Ok(HttpResponse::Ok()
        .header("Content-Type", "application/x-protobuf")
        .body(tile_unzipped))
}