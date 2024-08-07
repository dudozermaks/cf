#!/usr/bin/env bash

# Check if an argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <problem_name>"
    exit 1
fi

echo -e "\n[[bin]]\nname = \"$1\"\npath = \"src/$1.rs\"" >> Cargo.toml
touch "src/$1.rs"

nvim "src/$1.rs"
