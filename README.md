# {{project-name}}

REST API built with Axum, SeaORM and utoipa, organised as a clean architecture
workspace, with JWT authentication, roles and tenant isolation.

Database chosen at generation time: **{{database}}**.

## Run

Create `.env` with a random `JWT_SECRET`{% if database != "sqlite" %}; it also asks for the
database password{% endif %}:

```powershell
./scripts/new-env.ps1
```

On Linux or macOS use `sh scripts/new-env.sh`.
{%- if database != "sqlite" %}

The app connects to `{{db_user}}@{{db_host}}:{{db_port}}` and creates the database
`{{ db_name | default: crate_name }}` on first run if it is missing. That needs the right to
create databases. Change `DATABASE_URL` in `.env` to point somewhere else.
{%- endif %}

```bash
cargo run
```

Migrations run at startup. API docs: <http://localhost:3000/docs/> (Swagger UI)
and <http://localhost:3000/scalar>.

## First calls

`POST /auth/register` creates a tenant and its admin and returns a token. In
Swagger UI, press **Authorize** and paste `access_token`.

## Test

```bash
cargo test --workspace
```

Tests use in-memory SQLite. To run them against another server:
`TEST_DATABASE_URL=<url> cargo test --workspace -- --test-threads=1`.
Use a database made for tests, because rows are left behind.
{%- if database == "postgres" %}

## Citus (optional, local development only)

1. `sudo bash deploy/citus/setup-wsl.sh` installs a coordinator and two workers.
   It trusts every connection from 127.0.0.1.
2. Set `DATABASE_URL="postgres://postgres@127.0.0.1:9700/{{ db_name | default: crate_name }}"` and
   start the app once so that migrations run.
3. Run `deploy/citus/distribute.sql` on the coordinator, before adding data.

Every new tenant table needs `tenant_id` in its primary key and unique indexes,
plus a line in `distribute.sql`.
{%- endif %}

## Layout

| Crate | Content |
|---|---|
| `api` | routers, extractors, error mapping |
| `app` | use cases, persistence, auth, config |
| `models` | entities, parameters, queries, schemas |
| `migration` | SeaORM migrations |
| `doc` | OpenAPI document |
| `utils` | database and test helpers |

## Credits

Generated from [jin3107/clean-axum](https://github.com/jin3107/clean-axum), a fork
of [kigawas/clean-axum](https://github.com/kigawas/clean-axum) (MIT). Keep
`third-party/clean-axum-LICENSE`.
