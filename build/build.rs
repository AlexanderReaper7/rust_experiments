use std::{io::prelude::*, format};
fn main() {
    // add file icon
    set_icon();
    proccess_assets();

    println!("cargo:rerun-if-changed=build/build.rs");
    println!("cargo:rerun-if-changed=build/asset_list.toml");
    println!("cargo:rerun-if-changed=assets");

}

#[cfg(target_os = "windows")]
fn set_icon() { // TODO: fix adding icon
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico").compile().unwrap();
}
#[cfg(not(target_os = "windows"))]
fn set_icon() {
    println!("cargo:warning=Setting icon is not implemented for this platform");
}

use serde::Deserialize;

#[derive(Deserialize)]
struct Assets {
    asset: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    path: String,
    description: Option<String>,
}

const ZST_EXSTENSION: &str = ".zst";
const COMPRESSION_LEVEL: i32 = 19;

fn proccess_assets() {
    // parse assets list
    let include_list = include_str!("asset_list.toml");
    let include_list: Assets = toml::from_str(include_list).unwrap();

    // compress and incude assets
    std::env::set_current_dir("assets").unwrap();
    for entry in &include_list.asset {
        let path = &entry.path;
        let file = std::fs::File::open(&path).unwrap();
        let buffer = zstd::encode_all(file, COMPRESSION_LEVEL).unwrap();
        std::env::set_current_dir("../build/assets").unwrap();
        let mut new_file = std::fs::File::create(path.clone() + ZST_EXSTENSION).unwrap();
        new_file.write_all(&buffer).unwrap();
        std::env::set_current_dir("../../assets").unwrap();
    }
    // return to root
    std::env::set_current_dir("..").unwrap();
    // create assets.rs
    let mut assets_rs = std::fs::File::create("src/assets.rs").unwrap();
    assets_rs.write_all(format!("//! Contains all the assets. \n//! Auto generated on {}\n\n", chrono::Utc::now().to_string()).as_bytes()).unwrap();
    for asset in &include_list.asset {
        // description
        if let Some(description) = &asset.description {
            description.split("\n").for_each(|x| { assets_rs.write_all(format!("/// {}\n", x).as_bytes()).unwrap();});
        }
        // the actual asset
        assets_rs.write_all(format!("pub static {name}: &'static [u8] = include_bytes!(\"../build/assets/{path}{ext}\");\n", name = asset.name, path = asset.path, ext = ZST_EXSTENSION).as_bytes()).unwrap();
    }
    // create decompression function
    assets_rs.write_all(
        format!(
"\n
use std::{{io::prelude::*, fs::{{File, OpenOptions}}}};
/// Decompresses the given asset.
pub fn decompress(asset: &[u8]) -> Vec<u8> {{
    zstd::decode_all(asset).unwrap()
}}

/// Unpacks the given asset to the given path.
pub fn unpack(asset: &[u8], path: &std::path::Path) -> std::io::Result<File> {{
    let mut file = OpenOptions::new().write(true).read(true).create(true).truncate(true).open(path)?;
    file.write_all(decompress(asset).as_slice())?;
    file.rewind()?;
    Ok(file)
}}"
        ).as_bytes()).unwrap();
}
