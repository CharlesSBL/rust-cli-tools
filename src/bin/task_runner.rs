// src/bin/task_runner.rs
use clap::Parser;
use tokio::process::Command;
use std::process::Stdio;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    commands: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut handles = vec![];

    for cmd in args.commands {
        let handle = tokio::spawn(async move {
            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .stdout(Stdio::piped())
                .output()
                .await?;
            println!("Command {} exited with {}", cmd, output.status);
            Ok::<_, std::io::Error>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }
    Ok(())
}
