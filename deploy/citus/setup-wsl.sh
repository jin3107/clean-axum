#!/usr/bin/env bash
# Local development only: 1 Citus coordinator + 2 workers on one Ubuntu machine.
# Run with: sudo bash setup-wsl.sh
# TCP connections from 127.0.0.1 are trusted (no password). Never use on a server.
set -euo pipefail

[ "$(id -u)" -eq 0 ] || { echo "Run with sudo"; exit 1; }

apt-get update
apt-get install -y curl ca-certificates
repo_script=$(mktemp)
curl -fsSL https://install.citusdata.com/community/deb.sh -o "$repo_script"
bash "$repo_script"
rm -f "$repo_script"
apt-get update

package=$(apt-cache search --names-only '^postgresql-[0-9]+-citus-[0-9.]+$' | awk '{print $1}' | sort -V | tail -1)
[ -n "$package" ] || { echo "No Citus package found for this Ubuntu release"; exit 1; }
pg=$(echo "$package" | cut -d- -f2)
echo "Installing $package"
apt-get install -y "$package"

for node in coord:9700 worker1:9701 worker2:9702; do
    name=${node%:*}
    port=${node#*:}
    if ! pg_lsclusters -h | awk '{print $1" "$2}' | grep -qx "$pg $name"; then
        pg_createcluster "$pg" "$name" --port "$port"
    fi
    pg_conftool "$pg" "$name" set shared_preload_libraries citus
    pg_conftool "$pg" "$name" set listen_addresses localhost
    sed -i -E 's#^(host\s+all\s+all\s+127\.0\.0\.1/32\s+).*#\1trust#' "/etc/postgresql/$pg/$name/pg_hba.conf"
    pg_ctlcluster "$pg" "$name" restart

    psql_node=(psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -p "$port" -U postgres)
    if [ "$("${psql_node[@]}" -tAc "SELECT 1 FROM pg_database WHERE datname='clean_axum'")" != 1 ]; then
        "${psql_node[@]}" -c "CREATE DATABASE clean_axum"
    fi
    "${psql_node[@]}" -d clean_axum -c "CREATE EXTENSION IF NOT EXISTS citus"
done

coord=(psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -p 9700 -U postgres -d clean_axum)
"${coord[@]}" -c "SELECT citus_set_coordinator_host('127.0.0.1', 9700)"
"${coord[@]}" -c "SELECT citus_add_node('127.0.0.1', 9701)"
"${coord[@]}" -c "SELECT citus_add_node('127.0.0.1', 9702)"
"${coord[@]}" -c "SELECT * FROM citus_get_active_worker_nodes()"
echo "Ready: postgres://postgres@127.0.0.1:9700/clean_axum"
