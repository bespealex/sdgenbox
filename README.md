# sdgenbox

## What it is
sdgenbox - simple web server for storing and navigating through generated images via Stable Diffusion.

## How to run
```bash
# Clone repository
git clone https://github.com/discrimy/sdgenbox.git
cd sdgenbox
# Install sqlx-cli for migrations management
cargo install sqlx-cli --no-default-features -F sqlite -F rustls
# Create sqlite database and apply migrations (needed to check SQL queries during compilation)
touch db.sqlite3 && sqlx migrate run
# Build and run project using rust toolchain
cargo run
```

## How to create development environment
```bash
# Install cargo-watch to rebuild server on files changes
cargo install cargo-watch
# Install pre-commit as git hook (you must install pre-commit beforehand, you can use nix or others package installers)
pre-commit install
# Run and autorebuild server
cargo watch -x run
```

## Why I built it
The reason I built it is I'm applying my knowledge in Rust to make something I can use. Maybe it's not so complicated as some database engine but it gives me an experience how to use rust in real world.
