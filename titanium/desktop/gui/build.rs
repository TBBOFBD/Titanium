#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_language(0x0009);
    match titaniumutils::getAsset("icon-simple.ico") {
        Ok(s) => {
            let path = s.as_str();
            res.set_icon(path);
        }
        Err(_) => {}
    }
    res.compile().expect("Compilation Failed");
}

#[cfg(not(windows))]
fn main() {}