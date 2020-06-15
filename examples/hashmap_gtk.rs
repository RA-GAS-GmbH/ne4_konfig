pub(crate) mod gui {
    pub mod gtk3 {
        pub fn launch() {}
    }
}

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    info!("Launch GUI");
    gui::gtk3::launch();
}
