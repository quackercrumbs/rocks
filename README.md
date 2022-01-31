# rocks

## Setup

### Secrets
```
echo "[topsecrets]" >> config/private.ini
echo "DATABASE_URL = /home/calvinq/projects/rocks/data/asteroids.db" >> config/private.ini
echo "NASA_API_KEY = 123456" >> config/private.ini
```

### Libraries

Diesel Getting Started: https://diesel.rs/guides/getting-started
> By default diesel depends on the following client libraries:
>
>    ... 
>    libsqlite3 for SQlite backend


Also might be worth installed `diesel-cli`. Note: we are only using sqlite3 in this project
```
sudo apt install sqlite3
sudo apt install libsqlite3-dev
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

### Setting up db

After installing the tools and libraries for the db, run this diesel command
```
diesel setup --database-url=<path to db>

// example
diesel setup --database-url='/home/calvinq/projects/rocks-data/asteroids.db'

// running the migrations
diesel migration run --database-url='/home/calvinq/projects/rocks-data/asteroids.db'
diesel migration redo --database-url='/home/calvinq/projects/rocks-data/asteroids.db'
```

### Quick Start
```
cargo run --bin importer -- --start-date 2022-01-08
cargo run --bin importer -- --help
```
