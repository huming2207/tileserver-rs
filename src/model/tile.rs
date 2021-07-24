use std::{collections::HashMap, env};
use std::path::{Path, PathBuf};

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OpenFlags};

use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;

use crate::util::tile_utils::{DataFormat, decode, get_data_format};

use super::error::{BaseError, Result, TileError};

type Connection = r2d2::PooledConnection<SqliteConnectionManager>;

lazy_static! {
    static ref MBTILES_POOL: r2d2::Pool<SqliteConnectionManager> = {
        let path = env::var("TS_MBTILE_PATH").unwrap_or("data.mbtile".to_string());
        let manager = SqliteConnectionManager::file(Path::new(&path)).with_flags(OpenFlags::SQLITE_OPEN_READ_ONLY);
        r2d2::Pool::new(manager).unwrap()
    };

}

#[derive(Clone, Debug)]
pub struct TileMeta {
    pub name: Option<String>,
    pub version: Option<String>,
    pub tilejson: String,
    pub scheme: String,
    pub id: String,
    pub tile_format: DataFormat,
    pub grid_format: Option<DataFormat>,
    pub bounds: Option<Vec<f64>>,
    pub center: Option<Vec<f64>>,
    pub minzoom: Option<u32>,
    pub maxzoom: Option<u32>,
    pub description: Option<String>,
    pub attribution: Option<String>,
    pub layer_type: Option<String>,
    pub legend: Option<String>,
    pub template: Option<String>,
    pub json: Option<JSONValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileSummaryJSON {
    pub image_type: DataFormat,
    pub url: String,
}

#[derive(Deserialize)]
struct UTFGridKeys {
    pub grid: Vec<String>,
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UTFGrid {
    pub data: HashMap<String, JSONValue>,
    pub grid: Vec<String>,
    pub keys: Vec<String>,
}

pub fn get_data_format_via_query(
    tile_name: &str,
    connection: &Connection,
    category: &str,
) -> Result<DataFormat> {
    let query = match category {
        "tile" => r#"SELECT tile_data FROM tiles LIMIT 1"#,
        "grid" => r#"SELECT grid_utfgrid FROM grid_utfgrid LIMIT 1"#,
        _ => {
            return Err(BaseError::TileData(TileError::InvalidDataFormat(String::from(
                tile_name,
            ))))
        }
    };
    let mut statement = match connection.prepare(query) {
        Ok(s) => s,
        Err(err) => return Err(BaseError::TileData(TileError::Database(err))),
    };
    let data_format: DataFormat = statement
        .query_row([], |row| {
            Ok(get_data_format(&row.get::<_, Vec<u8>>(0).unwrap()))
        })
        .unwrap_or(DataFormat::UNKNOWN);
    Ok(data_format)
}

pub fn get_tile_details(path: &PathBuf, tile_name: &str) -> Result<TileMeta> {
    
    // 'tiles', 'metadata' tables or views must be present
    let query = r#"SELECT count(*) FROM sqlite_master WHERE name IN ('tiles', 'metadata')"#;
    let connection = match MBTILES_POOL.get() {
        Ok(conn) => conn,
        Err(err) => {
            return Err(BaseError::TileData(TileError::ConnPool(err)))
        },
    };

    let mut statement = match connection.prepare(query) {
        Ok(s) => s,
        Err(err) => return Err(BaseError::TileData(TileError::Database(err))),
    };
    match statement.query_row([], |row| Ok(row.get::<_, i8>(0).unwrap_or(0))) {
        Ok(count) => {
            if count < 2 {
                return Err(BaseError::TileData(TileError::MissingTable(String::from(tile_name))));
            }
        }
        Err(err) => return Err(BaseError::TileData(TileError::Database(err))),
    };

    let tile_format = match get_data_format_via_query(&tile_name, &connection, "tile") {
        Ok(tile_format) => match tile_format {
            DataFormat::UNKNOWN => return Err(BaseError::TileData(TileError::UnknownTileFormat(String::from(tile_name)))),
            DataFormat::GZIP => DataFormat::PBF, // GZIP masks PBF format too
            _ => tile_format,
        },
        Err(err) => return Err(err),
    };

    let mut metadata = TileMeta {
        name: None,
        version: None,
        tilejson: String::from("2.1.0"),
        scheme: String::from("xyz"),
        id: String::from(tile_name),
        tile_format,
        grid_format: get_grid_info(&connection, tile_name),
        bounds: None,
        center: None,
        minzoom: None,
        maxzoom: None,
        description: None,
        attribution: None,
        layer_type: None,
        legend: None,
        template: None,
        json: None,
    };

    let mut statement = connection
        .prepare(r#"SELECT name, value FROM metadata WHERE value IS NOT ''"#)
        .unwrap();
    let mut metadata_rows = statement.query([]).unwrap();

    while let Some(row) = metadata_rows.next().unwrap() {
        let label: String = row.get(0).unwrap();
        let value: String = row.get(1).unwrap();
        match label.as_ref() {
            "name" => metadata.name = Some(value),
            "version" => metadata.version = Some(value),
            "bounds" => {
                metadata.bounds = Some(value.split(",").filter_map(|s| s.parse().ok()).collect())
            }
            "center" => {
                metadata.center = Some(value.split(",").filter_map(|s| s.parse().ok()).collect())
            }
            "minzoom" => metadata.minzoom = Some(value.parse().unwrap()),
            "maxzoom" => metadata.maxzoom = Some(value.parse().unwrap()),
            "description" => metadata.description = Some(value),
            "attribution" => metadata.attribution = Some(value),
            "type" => metadata.layer_type = Some(value),
            "legend" => metadata.legend = Some(value),
            "template" => metadata.template = Some(value),
            "json" => metadata.json = Some(serde_json::from_str(&value).unwrap()),
            _ => (),
        }
    }

    Ok(metadata)
}

fn get_grid_info(connection: &Connection, tile_name: &str) -> Option<DataFormat> {
    let mut statement = connection.prepare(r#"SELECT count(*) FROM sqlite_master WHERE name IN ('grids', 'grid_data', 'grid_utfgrid', 'keymap', 'grid_key')"#).unwrap();
    let count: u8 = statement
        .query_row([], |row| Ok(row.get(0).unwrap()))
        .unwrap();
    if count == 5 {
        match get_data_format_via_query(&tile_name, &connection, "grid") {
            Ok(grid_format) => return Some(grid_format),
            Err(err) => {
                println!("{}", err);
                return None;
            }
        };
    }
    None
}

pub fn get_grid_data(
    data_format: DataFormat,
    z: u32,
    x: u32,
    y: u32,
) -> Result<UTFGrid> {
    let connection = match MBTILES_POOL.get() {
        Ok(conn) => conn,
        Err(err) => {
            return Err(BaseError::TileData(TileError::ConnPool(err)))
        },
    };

    let mut statement = connection
        .prepare(
            r#"SELECT grid
                 FROM grids
                WHERE zoom_level = ?1
                  AND tile_column = ?2
                  AND tile_row = ?3
            "#,
        )
        .unwrap();
    let grid_data = match statement.query_row(params![z, x, y], |row| {
        Ok(row.get::<_, Vec<u8>>(0).unwrap())
    }) {
        Ok(d) => d,
        Err(err) => return Err(BaseError::TileData(TileError::Database(err))),
    };
    let grid_key_json: UTFGridKeys =
        serde_json::from_str(&decode(grid_data, data_format).unwrap()).unwrap();
    let mut grid_data = UTFGrid {
        data: HashMap::new(),
        grid: grid_key_json.grid,
        keys: grid_key_json.keys,
    };

    let mut statement = connection
        .prepare(
            r#"SELECT key_name, key_json
                 FROM grid_data
                WHERE zoom_level = ?1
                  AND tile_column = ?2
                  AND tile_row = ?3
            "#,
        )
        .unwrap(); // TODO handle error
    let grid_data_iter = statement
        .query_map(params![z, x, y], |row| {
            Ok((
                row.get::<_, String>(0).unwrap(),
                row.get::<_, String>(1).unwrap(),
            ))
        })
        .unwrap();
    for gd in grid_data_iter {
        let (k, v) = gd.unwrap();
        let v: JSONValue = serde_json::from_str(&v).unwrap();
        grid_data.data.insert(k, v);
    }

    Ok(grid_data)
}

pub fn get_tile_data(z: u32, x: u32, y: u32) -> Result<Vec<u8>> {
    let connection = match MBTILES_POOL.get() {
        Ok(conn) => conn,
        Err(err) => {
            return Err(BaseError::TileData(TileError::ConnPool(err)))
        },
    };
    
    let mut statement = connection
        .prepare(
            r#"SELECT tile_data
                 FROM tiles
                WHERE zoom_level = ?1
                  AND tile_column = ?2
                  AND tile_row = ?3
            "#,
        )
        .unwrap(); // TODO handle error
    match statement.query_row(params![z, x, y], |row| Ok(row.get(0).unwrap())) {
        Ok(data) => Ok(data),
        Err(err) => Err(BaseError::TileData(TileError::Database(err))),
    }
}