#!/bin/bash

set -euf -o pipefail

if ! echo "$1" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+(-rc\.[0-9]+)?$'; then
  echo "${1} is not in MAJOR.MINOR.PATCH format"
  exit 1
fi

# Create a new signed annotated tag for the version
git tag "v${1}" -as -m "release v${1}"

# Create or update the "latest" tag
git tag -af "latest" -m "latest release" "v${1}^{}"

# Create or update the "nightly" tag
git tag -af "nightly" -m "nightly release" "v${1}^{}"

# Push all tags, which triggers the Release workflow
git push --atomic origin "v${1}" "latest" "nightly" --force
