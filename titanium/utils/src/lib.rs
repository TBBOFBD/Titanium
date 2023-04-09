//! # Titanium Utils

#![deny(missing_docs,missing_debug_implementations,unused,clippy::all)]

#[cfg(feature = "assets")]
fn load_asset_from<T: ToString>(path: &str, name: T) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let cd = std::env::current_dir()?;
    let item = cd.join(
        format!("{}/{}", path, name.to_string())
    );
    let item = item.canonicalize()?;
    let item = match item.to_str() {
        Some(s) => s,
        None => {
            return Err(
                Box::new(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Unable to convert icon path to string"
                    )
                )
            )
        }
    };
    return Ok(
        item.to_string()
    );
}

#[cfg(feature = "macros")]
mod macros;

/// loads an asset from the assets folder relative to the current directory (dynamic)
#[cfg(feature = "assets")]
#[allow(non_snake_case)]
pub fn getAsset<T: ToString>(name: T) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    load_asset_from("../../assets", name)
}

/// loads an asset from the assets folder starting from the root (dynamic)
#[cfg(feature = "assets")]
#[allow(non_snake_case)]
pub fn getAssetFromRoot<T: ToString>(name: T) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    load_asset_from("./titanium/assets", name)
}

/// a module containing all the assets (static)
#[cfg(feature = "assets")]
pub mod assets {
    #[cfg(feature = "assets")]
    pub use titaniumassets::*;
}
