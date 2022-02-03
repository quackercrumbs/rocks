# rocks

## Setup

### Secrets (required, at least for the moment )
```
echo "[topsecrets]" >> config/private.ini
echo "DATABASE_URL = /home/calvinq/projects/rocks/data/asteroids.db" >> config/private.ini
echo "NASA_API_KEY = 123456" >> config/private.ini
```

### Dependencies
#### Linux
- Install sqlite stuff
  - `sudo apt install sqlite3`
  - `sudo apt install libsqlite3-dev`
- Install bevy dependencies
  - Refer to this doc: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

#### Windows
- Install sqlite via Chocolately
  - `choco install sqlite`
- Install VS2019 Build Tools
  - This is needed for running the game engine as well
- Launch Developer Command Prompt as Admin
- Find and change directories into the SQLite installation directory
  - It is probably `C:\ProgramData\chocolatey\lib\SQLite\tools`
- Run `lib /DEF:sqlite3.def /OUT:sqlite3.lib /MACHINE:x64` to generate `sqlite.lib` file
- Copy `sqlite.lib` file into the rust tool chain that you're using
  - ```C:\Users\User\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib)```
  - You can find out which tool chain you're using by using `rustup toolchain list`

#### Additional
- (optional) Install `diesel-cli` if you need to run db imports
  - `cargo install diesel_cli --no-default-features --features "sqlite-bundled"`

### Setting up db from scratch (optional)

If you don't have a snapshot of the database, follow these steps to setup your sqlite snapshot.
Make sure you follow the instructions for setting up your environment.

```
diesel setup --database-url=<path to db>

// example
diesel setup --database-url='/home/calvinq/projects/rocks-data/asteroids.db'

// running the migrations
diesel migration run --database-url='/home/calvinq/projects/rocks-data/asteroids.db'
diesel migration redo --database-url='/home/calvinq/projects/rocks-data/asteroids.db'
```

Then run the `importer` to populate the database. (Check quick start instructions)

## Quick Start
```
// updating the database
cargo run -p importer -- --start-date 2022-01-08
cargo run -p importer -- --help

// querying the database (you can use sqlite as well)
cargo run -p importer --bin query_responses

```

## References
- Diesel Getting Started: https://diesel.rs/guides/getting-started
- Bevy Getting Started: https://bevyengine.org/learn/book/getting-started/setup/
