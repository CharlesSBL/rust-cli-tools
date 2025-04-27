use clap::Parser;
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let total_size = WalkDir::new(&args.path)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().unwrap().len())
        .sum::<u64>();

    println!("Total size: {} bytes", total_size);
    Ok(())
}
