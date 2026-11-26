#!/usr/bin/env sh
# Creates .env from .env.example with a random JWT_SECRET. Never overwrites.
set -eu
cd "$(dirname "$0")/.."
[ ! -e .env ] || { echo ".env already exists, nothing changed" >&2; exit 1; }

secret=$(head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n')
umask 077
sed "s/^JWT_SECRET=.*/JWT_SECRET=$secret/" .env.example > .env
echo "Created .env"
