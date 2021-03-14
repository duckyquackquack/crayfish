cargo fmt;
cargo clippy;
cargo build --release;
Copy-Item "scene_config.json" -Destination "target\release"
cargo run --release;