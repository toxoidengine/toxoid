use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")] 
    pub object_type: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub gid: Option<u32>,
    pub visible: bool,
    pub properties: Option<Vec<Property>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub data: Option<Vec<u32>>,
    pub height: Option<u32>,
    pub id: u32,
    pub name: String,
    pub opacity: f32,
    #[serde(rename = "type")]
    pub layer_type: String,
    pub visible: bool,
    pub width: Option<u32>,
    pub layers: Option<Vec<Layer>>,
    pub x: u32,
    pub y: u32,
    pub properties: Option<Vec<Property>>,
    pub objects: Option<Vec<Object>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub name: String,
    #[serde(rename = "type")]
    pub property_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tileset {
    pub columns: u32,
    pub firstgid: u32,
    pub image: String,
    pub imageheight: u32,
    pub imagewidth: u32,
    pub margin: u32,
    pub name: String,
    pub spacing: u32,
    pub tilecount: u32,
    pub tileheight: u32,
    pub tilewidth: u32,
    #[serde(skip)]
    pub sprite: Option<*mut std::ffi::c_void>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TiledCell {
    pub compressionlevel: i32,
    pub height: u32,
    pub infinite: bool,
    pub layers: Vec<Layer>,
    pub nextlayerid: u32,
    pub nextobjectid: u32,
    #[serde(rename = "orientation")]
    pub orientation: String,
    #[serde(rename = "renderorder")]
    pub render_order: String,
    #[serde(rename = "tiledversion")]
    pub tiled_version: String,
    pub tileheight: u32,
    pub tilesets: Vec<Tileset>,
    pub tilewidth: u32,
    #[serde(rename = "type")]
    pub cell_type: String,
    pub version: String,
    pub width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditorSettings {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TiledCellData {
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub height: u32,
    pub width: u32,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TiledWorld {
    pub maps: Option<Vec<TiledCellData>>,
    #[serde(rename = "onlyShowAdjacentMaps")]
    pub only_show_adjacent_maps: bool,
    #[serde(rename = "type")]
    pub world_type: String,
}

pub fn parse_world(world: &str) -> TiledWorld {
    let world: TiledWorld = serde_json::from_str(world).unwrap();
    world
}

pub fn parse_cell(cell: &str) -> TiledCell {
    let cell: TiledCell = serde_json::from_str(cell).unwrap();
    cell
}