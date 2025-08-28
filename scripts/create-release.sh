#!/bin/sh

if [ "$1" = "" ]; then
    echo "No version number specified. Usage: ./create-release.sh VERSION_NUMBER"
else
    cargo audit
    cargo test
    cargo build --release
    git tag $1
    echo "Tagged $1. Run 'git push --tags' to publish."
fi
