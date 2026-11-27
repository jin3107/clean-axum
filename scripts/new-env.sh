#!/usr/bin/env sh
# Creates .env from .env.example with a random JWT_SECRET and, for a server
# database, the password you type. Never overwrites. For unattended use, set the
# DB_PASSWORD environment variable instead of answering the prompt.
set -eu
cd "$(dirname "$0")/.."
[ ! -e .env ] || { echo ".env already exists, nothing changed" >&2; exit 1; }

secret=$(head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n')

# percent-encode every byte that is not URL-safe
urlencode() {
    printf '%s' "$1" | od -An -v -tx1 | tr -s ' ' '\n' | while read -r hex; do
        [ -n "$hex" ] || continue
        char=$(printf "\\$(printf '%03o' "0x$hex")")
        case $char in
            [A-Za-z0-9._~-]) printf '%s' "$char" ;;
            *) printf '%%%s' "$hex" ;;
        esac
    done
}

encoded=""
if grep -q '__DB_PASSWORD__' .env.example; then
    if [ -z "${DB_PASSWORD+set}" ]; then
        printf 'Database password (input hidden): ' >&2
        if [ -t 0 ]; then stty -echo; fi
        IFS= read -r DB_PASSWORD || true
        if [ -t 0 ]; then stty echo; echo >&2; fi
    fi
    encoded=$(LC_ALL=C urlencode "$DB_PASSWORD")
fi

umask 077
sed -e "s/^JWT_SECRET=.*/JWT_SECRET=$secret/" \
    -e "s|:__DB_PASSWORD__@|:$encoded@|" .env.example > .env
echo "Created .env"

# Docker Compose has its own database, so it gets its own random secrets
if [ ! -e .env.docker ]; then
    random() { head -c "$1" /dev/urandom | od -An -tx1 | tr -d ' \n'; }
    sed -e "s/^POSTGRES_PASSWORD=.*/POSTGRES_PASSWORD=$(random 16)/" \
        -e "s/^JWT_SECRET=.*/JWT_SECRET=$(random 32)/" .env.docker.example > .env.docker
    echo "Created .env.docker"
fi
