#!/usr/bin/env bash
# Turn the template into a real project: rename the placeholder crate, swap
# in the description, promote the project README, and delete this script.
set -euo pipefail
cd "$(dirname "$0")"

die() {
    echo "init.sh: $1" >&2
    exit 1
}

[ $# -ge 2 ] || die "usage: ./init.sh <name> \"<description>\" [owner]"

NEW_NAME=$1
NEW_DESC=$2
NEW_OWNER=${3:-eljpsm}

printf '%s' "$NEW_NAME" | grep -Eq '^[a-z][a-z0-9_-]*$' \
    || die "name must match ^[a-z][a-z0-9_-]*$"
[ -n "$NEW_DESC" ] || die "description must not be empty"
grep -q renameme Cargo.toml || die "already initialized"

export NEW_NAME NEW_DESC NEW_OWNER

# The fixed list of files that carry the placeholder name or description.
FILES="Cargo.toml Cargo.lock flake.nix Makefile AGENTS.md README.template.md \
src/main.rs src/cli.rs src/app.rs tests/cli.rs"

perl -pi -e '
    s/renameme/$ENV{NEW_NAME}/g;
    s/\QA placeholder command line tool.\E/$ENV{NEW_DESC}/g;
' $FILES

if [ "$NEW_OWNER" != "eljpsm" ]; then
    perl -pi -e '
        s{github\.com/eljpsm/}{github.com/$ENV{NEW_OWNER}/}g;
        s{github:eljpsm/}{github:$ENV{NEW_OWNER}/}g;
    ' $FILES
    echo "note: .github/renovate.json still extends eljpsm/renovate-config; review it."
fi

mv README.template.md README.md
rm -- "$0"

echo "Initialized $NEW_NAME."
echo "Next: review with 'git diff', then:"
echo "  git add -A && git commit -m 'Initialize from rust-cli-template'"
echo "  nix build"
