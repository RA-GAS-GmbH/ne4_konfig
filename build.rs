extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("ne4_konfig.ico");
    res.compile().unwrap();
  }
}
