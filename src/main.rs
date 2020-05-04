#[macro_use]
extern crate log;

mod gui {
    pub mod gtk3;
}

mod serial_thread;

fn main() {
    env_logger::init();

    gui::gtk3::launch();
}
