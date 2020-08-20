// use ne4_konfig::gui::gtk3::treestore_values::TreeStoreValues;
use serde::Deserialize;
use std::error::Error;
use std::io;
use std::process;

#[derive(Debug, Deserialize)]
struct Record {
    reg: i32,
    range: String,
    value: String,
    description: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .has_headers(true)
        .from_reader(io::stdin());
    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running example: {}", err);
        process::exit(1)
    }
}
