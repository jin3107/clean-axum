# {{project-name}}

REST API built with Axum, SeaORM, PostgreSQL and utoipa, organised as a clean
architecture workspace, with JWT authentication, roles and tenant isolation, and
a React, TypeScript and Vite client in `client/`.

## Requirements

| Tool | Notes |
|---|---|
| Rust (stable) | on Windows also the Visual Studio C++ build tools |
| PostgreSQL | running, with a user that may create databases |
| Node.js 22.12 or newer | for the client |
| Docker with Compose | optional, replaces the local PostgreSQL, see [Docker](#docker) |

## Run

All commands start in the project folder.

Create `.env`. The script generates `JWT_SECRET` and asks for the database
password:

```powershell
./scripts/new-env.ps1
```

On Linux or macOS use `sh scripts/new-env.sh`.

Run it before the first commit: it also names the new repository's branch `main`.

Start the API. The first build takes several minutes:

```bash
cargo run
```

The app connects to `{{db_user}}@{{db_host}}:{{db_port}}` and creates the database
`{{ db_name | default: crate_name }}` on first run if it is missing. That needs the right to
create databases. Change `DATABASE_URL` in `.env` to point somewhere else.

Migrations run at startup. With `APP_ENV=development` the API docs are at
<http://localhost:3000/docs/> (Swagger UI) and <http://localhost:3000/scalar>.
With `OPEN_DOCS=1`, set in a new `.env`, Swagger UI opens in the default browser
once the API is ready.

## Client

With the API running, in a second terminal:

```bash
cd client
npm install
npm run dev
```

Open <http://localhost:5173>. The dev server forwards `/api` to
`http://127.0.0.1:3000`; change `client/vite.config.ts` if you change `PORT`.
`npm run lint` and `npm run build` check the client.

## Try the API

1. Open <http://localhost:3000/docs/>, if it has not opened by itself.
2. Run `POST /auth/register`. It creates a tenant and its admin.
3. Copy `access_token` from the response, press **Authorize** and paste it.
4. Run `POST /users` to add a user with another role, then `GET /users`.

Press `Ctrl+C` in a terminal to stop the API or the client.

## Docker

`scripts/new-env` also creates `.env.docker` with random secrets. Then:

```bash
docker compose --env-file .env.docker up -d --build
```

| Address | Content |
|---|---|
| <http://localhost:8080> | web client; `/api` is forwarded to the API |
| <http://localhost:8081/docs/> | API docs, with `APP_ENV=development` |

Compose runs its own PostgreSQL, kept in the `pgdata` volume. Both ports listen on
this machine only. `docker compose --env-file .env.docker down` stops the stack;
add `-v` to delete its data as well.

With Docker inside WSL, run these commands in the Ubuntu shell, after `cd` to the
project folder: `D:\work\{{project-name}}` on Windows is
`/mnt/d/work/{{project-name}}` there.

## Sessions

`POST /auth/register` creates a tenant and its admin. It and `POST /auth/login`
return an access token (15 minutes) and a refresh token (7 days). In Swagger UI,
press **Authorize** and paste `access_token`.

`POST /auth/refresh` returns a new pair. Each refresh token works once: store the
new one, and send only one refresh request at a time. `POST /auth/logout` ends
the session.

## Roles

`admin`{% assign names = roles | split: "," | uniq %}{% for name in names %}{% if name != "admin" %}, `{{ name }}`{% endif %}{% endfor %}.
Only `admin` creates users. To change the list, edit `models/src/domains/role.rs`;
at least one role besides `admin` is needed by the tests.

## Settings

| Variable | Meaning |
|---|---|
| `APP_ENV` | API docs are served only for `development` |
| `OPEN_DOCS` | `1` opens Swagger UI in the default browser at startup; set `0` to turn it off |
| `CORS_ORIGINS` | browser origins allowed to call the API, comma separated |
| `AUTH_RATE_LIMIT` | requests per minute and address for register and login |
| `JWT_ISSUER`, `JWT_AUDIENCE` | values tokens must carry |

## Test

```bash
cargo test --workspace
```

Tests need PostgreSQL. They use `TEST_DATABASE_URL`, or else the database in
`DATABASE_URL` with a `_test` suffix, which is created when missing. Rows are left
behind.

## Citus (optional, local development only)

1. `sudo bash deploy/citus/setup-wsl.sh` installs a coordinator and two workers.
   It trusts every connection from 127.0.0.1.
2. Set `DATABASE_URL="postgres://postgres@127.0.0.1:9700/{{ db_name | default: crate_name }}"` and
   start the app once so that migrations run.
3. Run `deploy/citus/distribute.sql` on the coordinator, before adding data.

Every new tenant table needs `tenant_id` in its primary key and unique indexes,
plus a line in `distribute.sql`.

## Layout

| Crate | Content |
|---|---|
| `api` | routers, extractors, middleware, error mapping |
| `app` | use cases, persistence, auth, sessions, config |
| `models` | entities, parameters, queries, schemas |
| `migration` | SeaORM migrations |
| `doc` | OpenAPI document |
| `utils` | database and test helpers |
| `client` | web client |

## Credits

Generated from [jin3107/clean-axum](https://github.com/jin3107/clean-axum), a fork
of [kigawas/clean-axum](https://github.com/kigawas/clean-axum) (MIT). Keep
`third-party/clean-axum-LICENSE`.
