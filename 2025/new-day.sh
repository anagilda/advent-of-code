#!/bin/bash

set -e

day=$(printf "%02d" $1)
folder="day-$day"
template="templates/main.rs.template"

if [ ! -f "$template" ]; then
    echo "Template not found: $template"
    exit 1
fi

echo "Creating $folder..."

# Create crate
cargo new $folder

# Copy template
cp "$template" "$folder/src/main.rs"

# Create input files
touch "$folder/input.txt"
touch "$folder/example.txt"

echo "✔ $folder created using template"
