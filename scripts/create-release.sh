#!/bin/sh

if [ "$1" = "" ]; then
    echo "No version number specified. Usage: ./create-version.sh VERSION_NUMBER"
else
    ./bump-version.sh $1
    cargo audit
    cargo test
    cargo build --release
    git tag $1
    git push --tags
fi
