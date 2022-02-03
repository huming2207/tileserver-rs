use std::{env, fs::File, io::Read};

use actix_web::{HttpResponse, get, web};

use crate::{model::{error::{Result, BaseError, TileError}, tile::get_tile_data}};

#[get("/{stack}/{size}")]
pub async fn serve_font(req: web::Path<(String, String)>) -> Result<HttpResponse> {
    let req_inner = req.into_inner();
    let stack_name = req_inner.0;
    let size = req_inner.1;


    let actual_stack_name = if stack_name.clone().contains(',') {
        let name = stack_name.clone();
        let vec: Vec<&str> = name.split(',').collect();
        if !vec.is_empty() {
            vec[0].to_string()
        } else {
            return Ok(HttpResponse::NotFound().body("No such font!"));
        }
    } else {
        stack_name.clone()
    };

    let font_path = format!("{}/{}/{}", env::var("TS_MAP_FONT_DIR").unwrap_or("./fonts".to_string()), actual_stack_name, size);
    let mut file = match File::open(font_path) {
        Ok(file) => {
            file
        },
        Err(_) => {
            return Ok(HttpResponse::NotFound().body("No such font!"));
        }
    };

    let mut buf: Vec<u8> = vec![];
    match file.read_to_end(&mut buf) {
        Ok(_) => {
            Ok(HttpResponse::Ok()
            .header("Content-Type", "application/x-protobuf")
            .body(buf))
        },
        Err(_) => {
            return Ok(HttpResponse::NotFound().body("Failed to read font"));
        }
    }
}