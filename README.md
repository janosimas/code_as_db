# Code as DB

Code as db is a proof of concept for storing code in a database instead of a text file.

In fact, this currently already happens, many tools already parse the code en generate databases for fast and easy information retrieval.

The idea here is the go on step further and have the code already in a db. This would allow easier search and tooling of the code.

## Build

The nightly toolchain is needed to build this project.

```bash
cargo +nightly build
```

## Initialize

The [butane_cli](https://crates.io/crates/butane_cli) is required to installed before using the crates.

```bash
cargo install butane_cli
```

The butane models need to be used to initialize the database:

```bash
cd code_as_db_models
butane init sqlite ../code.db
butane makemigration init
butane migrate
cargo +nightly run
cd ..
```

## Run

The database also needs to be declared in `code_as_db_builder`:

```bash
cd code_as_db_builder
butane init sqlite ../code.db
```

Then the sample code can be fetched from the database

```bash
cargo +nightly run
cd ..
```
