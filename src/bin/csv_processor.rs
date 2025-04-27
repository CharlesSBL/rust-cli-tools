use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
}

#[derive(Deserialize, Serialize)]
struct Record {
    #[serde(rename = "column1")]
    value: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = File::open(args.input)?;
    let reader = io::BufReader::new(file);
    let mut writer = csv::Writer::from_path(args.output)?;

    for result in csv::Reader::from_reader(reader).deserialize() {
        let record: Record = result?;
        if record.value > 50 {
            writer.serialize(record)?;
        }
    }
    Ok(())
}
