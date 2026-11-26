# Using this branch as a project template

This branch holds placeholders, so it does not build on its own. Change code on
`main`, then merge `main` into `template`.

## Install the generator once

```bash
cargo install cargo-generate --locked
```

## Create a project

```bash
cargo generate --git https://github.com/jin3107/clean-axum.git --branch template --name my-app
```

You are asked for the database (`sqlite`, `postgres` or `mysql`) and, for a server
database, its host, port, user and name. Answer on the command line to skip a
question, for example `-d database=postgres -d db_port=5433`. The password is
asked later by `scripts/new-env`.

## Short alias

Add to `cargo-generate.toml` in your `CARGO_HOME` folder:

```toml
[favorites.rust-api]
git = "https://github.com/jin3107/clean-axum.git"
branch = "template"
```

Then:

```bash
cargo generate rust-api --name my-app
```

## Placeholders

| Placeholder | Used in |
|---|---|
| `project-name` | package name, README |
| `crate_name` | log filter, default database name |
| `db_host`, `db_port`, `db_user`, `db_name` | `.env.example`, README, Citus script; `postgres` and `mysql` only |
| `database` | `.env.example`, README, whether `deploy/` is included |
| `authors` | `Cargo.toml` |
