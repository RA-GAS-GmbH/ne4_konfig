use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

struct RwregStore {
    store: gtk::TreeStore,
}

impl RwregStore {
    fn new() -> Self {
        let store = gtk::TreeStore::new(&[
            glib::Type::I32,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
        ]);

        RwregStore { store }
    }

    fn fill_treestore(&self) {
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &0,
                &"0 .. 65535 [0]",
                &"",
                &"Kundencode: zur freien Belegung z.B. Raumcode *",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &2,
                &"0 … 10000 [11111]",
                &"0 … 10000 ppm",
                &"Messwertvorgabe für Testzwecke",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &3,
                &"0 … 2500 [11111]",
                &"0 … 25,00 mA",
                &"Ausgangsstrom vorgeben für Testzwecke",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &4,
                &"-200 … 600 [11111]",
                &"-20,0 … 60,0 °C",
                &"Temperatur vorgeben für Testzwecke",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&10, &"0 … 16383", &"", &"Sensorspannung im Nullpunkt	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&11, &0, &0, &"Sensorwert Nullpunkt = 0	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &12,
                &"0 … 16383",
                &"",
                &"Sensorspannung im Kalibrierpunkt ( bei Endwert) 	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &13,
                &"0 … 10000",
                &"0 … 10000 ppm",
                &"Sensorwert im Kalibrierpunkt (bei Endwert) 	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &15,
                &"0 … 10000 [0]",
                &"0 … 10000 ppm [0 ppm]",
                &"Messwert unten für Ausgangsstrom unten	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &16,
                &"0 … 2500 [400]",
                &"0 … 25,00 mA [4 mA]",
                &"Ausgangsstrom im unteren Punkt	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &17,
                &"0 … 10000 [1000]",
                &"0 … 10000 ppm [1000ppm]",
                &"Messwert oben für Ausgangsstrom oben	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &16,
                &"0 … 2500 [2000]",
                &"0 … 25,00 mA [20 mA]",
                &"Ausgangsstrom im oberen Punkt	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &20,
                &"0 / 1",
                &"",
                &"Status (Auswerte IC) (keine Eingabemöglichkeit)",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&21, &"0 / 1", &"", &"Lock (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&22, &"0 … 7", &"", &"TIA_GAIN (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&23, &"0 … 3", &"", &"RLOAD (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&24, &"0 / 1", &"", &"REF_Source (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&25, &"0 … 3", &"", &"INT_Z (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&26, &"0 / 1", &"", &"BIAS_Sign (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&27, &"0 … 13", &"", &"BIAS (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&28, &"0 / 1", &"", &"FET_Short (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&29, &"0 … 7", &"", &"OP_Mode (Auswerte IC) 	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &30,
                &"50 ... 200",
                &"0,50 … 2,00",
                &"Kennlinie vom Sensorhersteller bei -20°C	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &31,
                &"50 ... 200",
                &"0,50 … 2,00",
                &"Kennlinie vom Sensorhersteller bei 0°C	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &32,
                &"50 ... 200",
                &"0,50 … 2,00",
                &"Kennlinie vom Sensorhersteller bei 10°C	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &33,
                &"50 ... 200",
                &"0,50 … 2,00",
                &"Kennlinie vom Sensorhersteller bei 20°C	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &34,
                &"50 ... 200",
                &"0,50 … 2,00",
                &"Kennlinie vom Sensorhersteller bei 30°C	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &35,
                &"50 ... 200",
                &"0,50 … 2,00",
                &"Kennlinie vom Sensorhersteller bei 40°C	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &36,
                &"50 ... 200",
                &"0,50 … 2,00",
                &"Kennlinie vom Sensorhersteller bei 60°C	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &42,
                &"0 … 16000 [11111]",
                &"",
                &"Sensor AD-Wert vorgeben für Testzwecke",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &49,
                &"0 … 65535",
                &"",
                &"Neustart / Grunddaten / entsichern",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&50, &"1 … 247 [1]", &"", &"Modbus-Geräteadresse	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&51, &"0 … 3 [1]", &"", &"Modbus Baudrate	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&52, &"0 … 4 [0]", &"", &"Modbus Mode	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &53,
                &"10 .. 1000 [180]",
                &"",
                &"Kalibrierwert Ausgangsstrom 4mA	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &54,
                &"10 … 1000 [900]",
                &"",
                &"Kalibrierwert Ausgangsstrom 20mA	*",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[
                &95,
                &"0, 129 … 256 [90]",
                &"",
                &"Sensornummer für MCS4000 - Mode",
            ],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&96, &"0 … 65535", &"", &"Einschaltzähler	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&97, &"0 … 65535", &"", &"Betriebsstunden	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&98, &"0 … 65535", &"", &"Gerätekennung vom Werk	*"],
        );
        self.store.insert_with_values(
            None,
            None,
            &[0, 1, 2, 3],
            &[&99, &"0 … 65535", &"", &"Arbeitsweise vom Werk	*"],
        );
    }

    fn build_ui(&self) -> gtk::ScrolledWindow {
        self.fill_treestore();

        let sortable_store = gtk::TreeModelSort::new(&self.store);
        let treeview = gtk::TreeView::with_model(&sortable_store);

        // Renderer Column 0
        let column_reg = gtk::TreeViewColumn::new();
        column_reg.set_title("Rwreg Nr.");
        column_reg.set_clickable(false);
        column_reg.set_sort_indicator(true);
        column_reg.set_sort_column_id(0);

        let renderer = gtk::CellRendererText::new();
        column_reg.pack_end(&renderer, true);
        column_reg.add_attribute(&renderer, "text", 0);

        treeview.append_column(&column_reg);

        // Renderer Column 1
        let column_range = gtk::TreeViewColumn::new();
        column_range.set_title("Wertebereich");

        let renderer = gtk::CellRendererText::new();
        column_range.pack_end(&renderer, true);
        column_range.add_attribute(&renderer, "text", 1);

        treeview.append_column(&column_range);

        // Renderer Column 2
        let column_value = gtk::TreeViewColumn::new();
        column_value.set_title("Zugeordnete Größe und Einheit");

        let renderer = gtk::CellRendererText::new();
        renderer.set_property_editable(true);
        column_value.pack_end(&renderer, true);
        column_value.add_attribute(&renderer, "text", 2);

        // let store = self.store.clone();
        // renderer.connect_editing_started(move |widget, path, text| {
        //     debug!("Edit started:\nwidget: {:?}\npath: {:?}\ntext: {:?}\n", widget, path, text);
        // });
        let store = self.store.clone();
        renderer.connect_edited(move |widget, path, text| {
            // debug!("Edited:\nwidget: {:?}\npath: {:?}\ntext: {:?}\n", widget, path, text);
            edit_cell(&widget, &path, text, &store);
        });

        treeview.append_column(&column_value);

        // Renderer Column 3
        let column_property = gtk::TreeViewColumn::new();
        column_property.set_title("Messwerteigenschaft");

        let renderer = gtk::CellRendererText::new();
        column_property.pack_end(&renderer, true);
        column_property.add_attribute(&renderer, "text", 3);

        treeview.append_column(&column_property);

        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let box_main = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        scrolled_window.add(&treeview);

        scrolled_window
    }
}

fn edit_cell(
    cell: &gtk::CellRendererText,
    path: &gtk::TreePath,
    new_text: &str,
    model: &gtk::TreeStore,
) {
    if let Some(iter) = model.get_iter(&path) {
        let old_value = model.get_value(&iter, 2);
        println!("{:?}", old_value.get::<String>());
        model.set_value(&iter, 2, &new_text.to_value());
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tree View/ Model Test");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1024, 600);

    let rwreg_store = RwregStore::new();
    let rwreg_window = rwreg_store.build_ui();
    window.add(&rwreg_window);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.gaswarnanlagen.example.treestore"),
        gio::ApplicationFlags::empty(),
    )
    .expect("Initialization failed....");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
