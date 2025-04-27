// src/bin/sys_monitor.rs
use clap::Parser;
use inotify::{Inotify, WatchMask};
use std::fs::File;
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Monitor memory
    let mut meminfo = File::open("/proc/meminfo")?;
    let mut buffer = String::new();
    meminfo.read_line(&mut buffer)?;
    println!("Memory: {}", buffer.trim());

    // Watch directory
    let mut inotify = Inotify::init()?;
    inotify.add_watch(&args.path, WatchMask::MODIFY)?;

    let mut buffer = [0; 1024];
    loop {
        let events = inotify.read_events_blocking(&mut buffer)?;
        for event in events {
            println!("File changed: {:?}", event.name);
        }
        thread::sleep(Duration::from_secs(1));
    }
}
