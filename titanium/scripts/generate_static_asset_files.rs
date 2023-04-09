//! USE THIS SCRIPT TO GENERATE THE STATIC ASSET FILES
//! This script will generate a file called lib.rs in "titanium/assets/"

use std::io::Write;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Asset {
    pub original_name: String,
    pub name: String,
    pub data: Vec<u8>
}

pub fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("../assets")?;

    let mut assets: Vec<Asset> = Vec::new();

    for path in paths {
        match path {
            Ok(p) => {
                let asset_file = p.path().to_str().unwrap().to_string();
                if asset_file.ends_with(".rs") || asset_file.ends_with(".toml") { continue; }
                assets.push(entry_parser(asset_file)?);
            },
            Err(_) => {}
        }
    }

    let mut file = fs::File::create("../assets/lib.rs")?;
    let mut output = String::new();
    for asset in assets {
        output.push_str(
            const_builder(&asset)
            .as_str()
        );
    }
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn entry_parser(entry: String) -> std::io::Result<Asset> {
    let asset_file = entry.replace("\\", "/");
    let data = fs::read(&asset_file)?;
    let name = asset_file
        .split("/")
        .last()
        .expect("Could not get last element of split")
        .to_string();
    let original_name = name.clone();
    let name = name.
        replace(".", "_")
        .replace("-", "_")
        .replace(" ", "_")
        .split("/")
        .last()
        .expect("Could not get last element of split")
        .to_string();
    Ok(
        Asset {
            original_name,
            name,
            data
        }
    )
}

fn const_builder(asset: &Asset) -> String {
    let data = format!(
        "{:?}",
        asset.data.as_slice()
    ).replace(" ", "");
    format!(
        "/// ORIGNAL FILE: {}\n#[doc(hidden)]pub const {}: [u8;{}] = {};\n",
        asset.original_name,
        asset.name.to_uppercase(),
        asset.data.len(),
        data
    )
}