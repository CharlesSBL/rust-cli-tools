# rust-cli-tools

To compile all tools:
  cargo build --release

To compile individual tools: 
  cargo build --release --bin safe_cp
  cargo build --release --bin csv_processor


Copy a file:  cargo run --bin safe_cp -- source.txt dest.txt
Process CSV:  cargo run --bin csv_processor -- input.csv output.csv
Scan directory:  cargo run --bin dir_scanner -- /path/to/dir
Monitor system:  cargo run --bin sys_monitor -- /path/to/watch
Run tasks:  cargo run --bin task_runner -- "echo Hello" "sleep 1"
     
