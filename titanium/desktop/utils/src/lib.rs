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
#[doc(hidden)]
mod macros;

#[cfg(feature = "assets")]
#[allow(non_snake_case)]
pub fn getAsset<T: ToString>(name: T) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    load_asset_from("../../assets", name)
}

#[cfg(feature = "assets")]
#[allow(non_snake_case)]
pub fn getAssetFromRoot<T: ToString>(name: T) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    load_asset_from("./titanium/assets", name)
}

#[cfg(feature = "assets")]
pub mod assets {
    pub use libassets::*;
}
