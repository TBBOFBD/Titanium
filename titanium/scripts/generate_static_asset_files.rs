//! USE THIS SCRIPT TO GENERATE THE STATIC ASSET FILES
//! This script will generate a file called lib.rs in "titanium/assets/"

use std::io::Write;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Asset {
    pub name: String,
    pub data: Vec<u8>
}

fn main() -> std::io::Result<()> {
    let paths = fs::read_dir("../assets")?;

    let mut assets: Vec<Asset> = Vec::new();

    for path in paths {
        match path {
            Ok(p) => {
                let asset_file = p.path().to_str().unwrap().to_string();

                if asset_file.ends_with(".rs") || asset_file.ends_with(".toml") {
                    continue;
                }

                let asset_file = asset_file.replace("\\", "/");
                let name = asset_file.split("/").last().unwrap().to_string();
                let name = name.replace(".", "_");
                let name = name.replace("-", "_");
                let name = name.replace(" ", "_");
                let name = name.split("/").last().unwrap().to_string();
                let data = fs::read(&asset_file)?;
                assets.push(
                    Asset {
                        name,
                        data
                    }
                );
            },
            Err(_) => {}
        }
    }

    let mut file = fs::File::create("../assets/lib.rs")?;
    let mut output = String::new();
    for asset in assets {
        output.push_str(const_builder(&asset).as_str());
    }
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn const_builder(asset: &Asset) -> String {
    let data = format!("{:?}",asset.data.as_slice());
    let data = data.replace(" ", "");
    format!(
        "#[doc(hidden)]pub const {}: [u8;{}] = {};\n",
        asset.name.to_uppercase(),
        asset.data.len(),
        data
    )
}