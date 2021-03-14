cargo fmt;
cargo clippy;
cargo build;
Copy-Item "scene_config.json" -Destination "target\debug"
cargo run;