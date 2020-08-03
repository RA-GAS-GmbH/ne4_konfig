// use ne4_konfig::gui::gtk3::treestore_values::TreeStoreValues;
use std::error::Error;
use std::io;
use std::process;

fn run() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(io::stdin());
    for result in reader.records() {
        let record = result?;
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
