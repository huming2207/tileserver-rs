use actix_web::{HttpResponse, get, web};

use crate::{model::{error::Result, tile::get_tile_data}};

#[get("/{zoom}/{x}/{y}.pbf")]
pub async fn serve_vector(req: web::Path<(String, String, String)>) -> Result<HttpResponse> {
    let req_inner = req.into_inner();
    let zoom = req_inner.0;
    let x = req_inner.1;
    let y = req_inner.2;

    let tile = get_tile_data(&zoom, &x, &y).unwrap();
    Ok(HttpResponse::Ok()
        .header("Content-Type", "application/x-protobuf")
        .body(tile))
}