#[macro_use]
extern crate log;

pub mod gui {
    pub mod gtk3;
}

pub mod sensors {
    pub mod ne4;
}

mod serial_thread;
