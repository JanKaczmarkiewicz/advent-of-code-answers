#!/usr/bin/env bash

set -Eeuo pipefail
IFS=$'\n\t'

if [ "$#" -ne 1 ]; then
  echo "usage: ./download <year>" >&2
  exit 1
fi

year=$1

if [ -z "${AOC_COOKIE:-}" ]; then
  echo "error: Missing AOC_COOKIE" >&2
  exit 1
fi

if [ ! -e "static/style.css" ]; then
  mkdir -p "static"

  echo "Downloading static/style.css"
  curl "https://adventofcode.com/static/style.css" -o "static/style.css"
fi

if [ ! -e "$year/index.html" ]; then
  mkdir -p "$year"

  echo "Downloading $year/index.html"
  curl "https://adventofcode.com/$year" -o "$year/index.html" --cookie "$AOC_COOKIE"
fi

for day in $(seq 25); do
  path="$year/day/$day"

  mkdir -p "$path"

  if [ ! -e "$path/index.html" ]; then
    echo "Downloading $path/index.html"
    curl "https://adventofcode.com/$path" -o "$path/index.html" --cookie "$AOC_COOKIE"
  fi

  if [ ! -e "$path/input" ]; then
    echo "Downloading $path/input"
    curl "https://adventofcode.com/$path/input" -o "$path/input" --cookie "$AOC_COOKIE"
  fi
done

echo "Done"