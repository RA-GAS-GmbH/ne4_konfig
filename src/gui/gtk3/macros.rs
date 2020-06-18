/// Build gtk objects with gtk::Builder
/// ```compile_fail
/// let glade_src = include_str!("main.ui");
/// let builder = gtk::Builder::new_from_string(glade_src);
/// // old build
/// let window_main: gtk::Window = builder.get_object("application_window").expect("Couldn't find 'window_main' in glade ui file");
/// // this macro build!
/// let window_main: gtk::Window = build!(builder, "application_window");
/// ```
#[macro_export]
macro_rules! build {
    ($builder:ident, $e:expr) => {
        $builder
            .get_object($e)
            .expect(&format!("Couldn't find '{}' in glade ui file", $e))
    };
}
