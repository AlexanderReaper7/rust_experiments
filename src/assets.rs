//! Contains all the assets. 
//! Auto generated on 2021-12-11 19:39:53.576029800 UTC

/// an example embeded file
pub static EMBEDED_FILE: &'static [u8] = include_bytes!("../build/assets/embededfile.txt.zst");
pub static EMBEDED_FILE2: &'static [u8] = include_bytes!("../build/assets/another_file.txt.zst");
/// the icon for the application
pub static ICON: &'static [u8] = include_bytes!("../build/assets/icon.ico.zst");


use std::{io::prelude::*, fs::{File, OpenOptions}};
/// Decompresses the given asset.
pub fn decompress(asset: &[u8]) -> Vec<u8> {
    zstd::decode_all(asset).unwrap()
}

/// Unpacks the given asset to the given path.
pub fn unpack(asset: &[u8], path: &std::path::Path) -> std::io::Result<File> {
    let mut file = OpenOptions::new().write(true).read(true).create(true).truncate(true).open(path)?;
    file.write_all(decompress(asset).as_slice())?;
    file.rewind()?;
    Ok(file)
}