
#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    use std::path::Path;
    let mut res = winres::WindowsResource::new();
    let icon_path = Path::new("resources").join("ne4_konfig.ico").display().to_string();
    res.set_icon(&icon_path);
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}
