# Code as DB

Code as db is a proof of concept for storing code in a database instead of a text file.

In fact, this currently already hapens, many tools already parse the code en generate databases for fast and easy information retrival.

The ideia here is the go on step further and have the code already in a db. This would allow easier search and tooling of the code.

# Executing

The [butane_cli](https://crates.io/crates/butane_cli) is required to initialize the database before using the crates. Each create has to be initialized:

```bash
cd code_as_db_*
butane init sqlite ../code.db
butane makemigration init
butane migrate
```
