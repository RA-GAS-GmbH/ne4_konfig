pub struct TreeStoreValues<'a> {
    pub reg: usize,        // Rwreg Nr. (Fcode: 0x03, 0x06)
    pub range: &'a str,    // Wertebereich
    pub value: &'a str,    // Zugeordnete Größe und Einheit
    pub property: &'a str, // Messwerteigenschaft
}
