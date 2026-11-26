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

You are asked for the database (`sqlite`, `postgres` or `mysql`). To skip the
question add `-d database=postgres`.

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
| `crate_name` | log filter, database names |
| `database` | `.env.example`, README, whether `deploy/` is included |
| `authors` | `Cargo.toml` |
