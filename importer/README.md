# Nasa Data importer

Originally I had the idea of importing the nasa data onto SQLite. And then having the application query that data. But at the time I couldn't figure out how to connect the DB Connection into the Bevy ECS system.

In hindsight I could've used the same setup I have now for the http calls to the NASA API. But also another point is that this importer was never needed.

Regardless, good learning experience on Diesel and it's apis.

## Setup

### Confg
```
# first make directory of the db
mkdir -p /home/calvinq/projects/rocks/data/

# add to config
echo "DATABASE_URL = /home/calvinq/projects/rocks/data/asteroids.db" >> config/private.ini
```

### System Dependencies
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

**NOTE**: run the migration stuff at project root directory (thats where i'm keeping the migration files)
```
diesel setup --database-url=<path to db>

// example
diesel setup --database-url='/home/calvinq/projects/rocks-data/asteroids.db'

// running the migrations
diesel migration run --database-url='/home/calvinq/projects/rocks-data/asteroids.db'
diesel migration redo --database-url='/home/calvinq/projects/rocks-data/asteroids.db'
```

Then run the `importer` to populate the database

## To Run
```
// updating the database
cargo run -p importer -- --start-date 2022-01-08
cargo run -p importer -- --help

// querying the database (you can use sqlite as well)
cargo run -p importer --bin query_responses
```

## References
- Diesel Getting Started: https://diesel.rs/guides/getting-started
