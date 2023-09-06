# sdgenbox

## What it is
sdgenbox - simple web server for storing and navigating through generated images via Stable Diffusion.

## How to run
```bash
# Clone repository
git clone https://github.com/discrimy/sdgenbox.git
cd sdgenbox
# Setup default .env
cp example.env .env
# Install sqlx-cli for migrations management
cargo install sqlx-cli --no-default-features -F sqlite -F rustls
# Create sqlite database and apply migrations (needed to check SQL queries during compilation)
touch db.sqlite3 && sqlx migrate run
# Build and run project using rust toolchain
# Install exiftool to extract metadata from images
nix-env -Ai nixpkgs.exiftool  # or your prefered package manager
cargo run
```

## How to create development environment
```bash
# Install `task` task runner and pre-commit
nix-env -Ai nixpkgs.pre-commit  # or use other package manager
# Install cargo-watch to rebuild server on files changes
cargo install cargo-watch
# Install pre-commit as git hook
pre-commit install
# Run tests to validate the installation is alright
./Taskfile.sh test
# Run and autorebuild server
./Taskfile.sh watch-run
```

## How to build docker image
```bash
docker build . -t discrimy/sdgenbox
```

## Why I built it
The reason I built it is I'm applying my knowledge in Rust to make something I can use. Maybe it's not so complicated as some database engine but it gives me an experience how to use rust in real world.
