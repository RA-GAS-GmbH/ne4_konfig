use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tree View/ Model Test");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1024, 600);

    let store = gtk::TreeStore::new(&[
        glib::Type::I32,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
    ]);
    store.insert_with_values(
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
    store.insert_with_values(
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
    store.insert_with_values(
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
    store.insert_with_values(
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
    store.insert_with_values(
        None,
        None,
        &[0, 1, 2, 3],
        &[&10, &"0 … 16383", &"", &"Sensorspannung im Nullpunkt	*"],
    );
    store.insert_with_values(
        None,
        None,
        &[0, 1, 2, 3],
        &[&11, &"0", &"0", &"Sensorwert Nullpunkt = 0	*"],
    );
    store.insert_with_values(
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
    store.insert_with_values(
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
    store.insert_with_values(
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
    store.insert_with_values(
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
    store.insert_with_values(
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
    store.insert_with_values(
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
    // 20, &0 / 1, &, &Status (Auswerte IC) (keine Eingabemöglichkeit)
    // 21, &0 / 1, &, &Lock (Auswerte IC) 	*
    // 22, &0 … 7, &, &TIA_GAIN (Auswerte IC) 	*
    // 23, &0 … 3, &, &RLOAD (Auswerte IC) 	*
    // 24, &0 / 1, &, &REF_Source (Auswerte IC) 	*
    // 25, &0 … 3, &, &INT_Z (Auswerte IC) 	*
    // 26, &0 / 1, &, &BIAS_Sign (Auswerte IC) 	*
    // 27, &0 … 13, &, &BIAS (Auswerte IC) 	*
    // 28, &0 / 1, &, &FET_Short (Auswerte IC) 	*
    // 29, &0 … 7, &, &OP_Mode (Auswerte IC) 	*
    // 30, &50 ... 200, &0,50 … 2,00, &Kennlinie vom Sensorhersteller bei -20°C	*
    // 31, &50 ... 200, &0,50 … 2,00, &Kennlinie vom Sensorhersteller bei 0°C	*
    // 32, &50 ... 200, &0,50 … 2,00, &Kennlinie vom Sensorhersteller bei 10°C	*
    // 33, &50 ... 200, &0,50 … 2,00, &Kennlinie vom Sensorhersteller bei 20°C	*
    // 34, &50 ... 200, &0,50 … 2,00, &Kennlinie vom Sensorhersteller bei 30°C	*
    // 35, &50 ... 200, &0,50 … 2,00, &Kennlinie vom Sensorhersteller bei 40°C	*
    // 36, &50 ... 200, &0,50 … 2,00, &Kennlinie vom Sensorhersteller bei 60°C	*
    // 42, &0 … 16000 [11111], &, &Sensor AD-Wert vorgeben für Testzwecke
    // 49, &0 … 65535, &, &Neustart / Grunddaten / entsichern
    // 50, &1 … 247 [1], &, &Modbus-Geräteadresse	*
    // 51, &0 … 3 [1], &, &Modbus Baudrate	*
    // 52, &0 … 4 [0], &, &Modbus Mode	*
    // 53, &10 .. 1000 [180], &, &Kalibrierwert Ausgangsstrom 4mA	*
    // 54, &10 … 1000 [900], &, &Kalibrierwert Ausgangsstrom 20mA	*
    // 95, &0, 129 … 256 [90], &, &Sensornummer für MCS4000 - Mode
    // 96, &0 … 65535, &, &Einschaltzähler	*
    // 97, &0 … 65535, &, &Betriebsstunden	*
    // 98, &0 … 65535, &, &Gerätekennung vom Werk	*
    // 99, &0 … 65535, &, &Arbeitsweise vom Werk	*

    let sortable_store = gtk::TreeModelSort::new(&store);
    let treeview = gtk::TreeView::new_with_model(&sortable_store);

    let column_reg = gtk::TreeViewColumn::new();
    column_reg.set_title("Rwreg Nr.");
    column_reg.set_clickable(true);
    column_reg.set_sort_indicator(true);
    column_reg.set_sort_column_id(0);

    let renderer = gtk::CellRendererText::new();
    column_reg.pack_end(&renderer, true);
    column_reg.add_attribute(&renderer, "text", 0);

    treeview.append_column(&column_reg);

    let column_range = gtk::TreeViewColumn::new();
    column_range.set_title("Wertebereich");
    column_range.set_clickable(true);
    column_range.set_sort_indicator(true);
    column_range.set_sort_column_id(0);

    let renderer = gtk::CellRendererText::new();
    column_range.pack_end(&renderer, true);
    column_range.add_attribute(&renderer, "text", 1);

    treeview.append_column(&column_range);

    let column_value = gtk::TreeViewColumn::new();
    column_value.set_title("Zugeordnete Größe und Einheit");
    column_value.set_clickable(true);
    column_value.set_sort_indicator(true);
    column_value.set_sort_column_id(0);

    let renderer = gtk::CellRendererText::new();
    column_value.pack_end(&renderer, true);
    column_value.add_attribute(&renderer, "text", 2);

    treeview.append_column(&column_value);

    let column_property = gtk::TreeViewColumn::new();
    column_property.set_title("Messwerteigenschaft");
    column_property.set_clickable(true);
    column_property.set_sort_indicator(true);
    column_property.set_sort_column_id(0);

    let renderer = gtk::CellRendererText::new();
    column_property.pack_end(&renderer, true);
    column_property.add_attribute(&renderer, "text", 3);

    treeview.append_column(&column_property);

    window.add(&treeview);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.gaswarnanlagen.ne4"),
        gio::ApplicationFlags::empty(),
    )
    .expect("Initialization failed....");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
