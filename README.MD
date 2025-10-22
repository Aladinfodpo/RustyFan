cargo ndk -t x86_64 -o ./bin build --release
cargo ndk -t arm64-v8a -o ./bin build --release
cargo build --release --target-dir ./bin/windows --lib  
