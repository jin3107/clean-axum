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

You are asked for the roles and the PostgreSQL host, port, user and database name. Answer on the
command line to skip a question, for example
`-d roles=doctor,receptionist -d db_port=5433`. The password is
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

`client/src` and `client/public` are copied without placeholder processing. To use
a placeholder there, remove the folder from `exclude` in `cargo-generate.toml`.

| Placeholder | Used in |
|---|---|
| `project-name` | package names, README, page title, default token issuer and audience |
| `crate_name` | log filter, default database name |
| `db_host`, `db_port`, `db_user`, `db_name` | `.env.example`, README, Citus script |
| `roles` | `models/src/domains/role.rs`, README |
| `authors` | `Cargo.toml` |
