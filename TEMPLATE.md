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

You are asked for the PostgreSQL host, port, user and database name. Answer on the
command line to skip a question, for example `-d db_port=5433`. The password is
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
| `project-name` | package name, README, default token issuer and audience |
| `crate_name` | log filter, default database name |
| `db_host`, `db_port`, `db_user`, `db_name` | `.env.example`, README, Citus script |
| `authors` | `Cargo.toml` |
