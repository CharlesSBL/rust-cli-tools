use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser)]
struct Args {
    source: String,
    destination: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut reader = File::open(&args.source)?;
    let mut writer = File::create(&args.destination)?;
    let mut buffer = [0; 8192];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        writer.write_all(&buffer[..n])?;
    }
    Ok(())
}
