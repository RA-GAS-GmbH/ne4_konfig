mod gui {
    pub mod gtk3;
}

fn main() {
    env_logger::init();

    gui::gtk3::launch();
}
