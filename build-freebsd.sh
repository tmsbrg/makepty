#!/bin/sh

# from https://github.com/wezm/freebsd-cross-build

set -e

cd "$(dirname $0)"
mkdir -p target/x86_64-unknown-freebsd

project=${PWD##*/} # Use the directory's name as the name of the project.
container="$project-freebsd-build"

# Use the container itself as the cache layer for the registry.
# This implies we won't create a new container for every build,
# but just start the same container.
if ! sudo docker ps -a --format '{{.Names}}' | grep -Eq "^${container}\$"; then
	sudo docker create \
	       --name "$container" \
	       -i -a STDIN -a STDOUT -a STDERR \
	       -v "$(pwd)":/rust/project:ro \
	       freebsd-cross-rust
fi

echo "cargo build --release --target x86_64-unknown-freebsd --target-dir /rust/target" \
	| sudo docker start -i "$container"

sudo docker cp "$container":/rust/target/x86_64-unknown-freebsd/ target/
