/// Diese Beispiel zeit den Einsatz des `thread_local` macros.
/// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use std::cell::RefCell;

thread_local!(
    pub static GLOBAL: RefCell<Option<String>> = RefCell::new(None)
);

mod gui {
    pub mod gtk3 {
        use crate::GLOBAL;

        pub fn test_thread_local() {
            GLOBAL.with(|global| {
                *global.borrow_mut() = Some(
                    "string".into()
                );
            })
        }

    }
}

fn main() {
    gui::gtk3::test_thread_local();

    println!("Thread Local Var: {:?}", GLOBAL.with(|global| global.borrow().clone()));
}
