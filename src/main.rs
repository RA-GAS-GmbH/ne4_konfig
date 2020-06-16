#![windows_subsystem = "windows"]
use ne4_konfig;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    info!("Launch GUI");
    ne4_konfig::gui::gtk3::launch();
}
