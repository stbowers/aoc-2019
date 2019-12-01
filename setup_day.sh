#!/bin/sh

YEAR="2019"
PROJECT=$(echo "day$1$2")
AOC_SESSION=".aocsession"

if [ -d $PROJECT ]; then
    echo "$PROJECT already set up"
else
    echo "Setting up $PROJECT..."

    # Create cargo crate
    cargo new $PROJECT

    # Append dependencies to the crate
    echo "aoc_utils = { path = \"../aoc_utils\", features=[\"text_utils\", \"file_utils\"] }" >> $PROJECT/Cargo.toml

    # Copy template.rs as main.rs
    rm $PROJECT/src/main.rs
    cp ./template.rs $PROJECT/src/main.rs

    # Get input file
    if [ -e $AOC_SESSION ]; then
    	curl -b session=$(cat .aocsession) https://adventofcode.com/$YEAR/day/$1/input > $PROJECT/input.txt
    else
    	echo "File $AOC_SESSION not found... Skipping downloading of puzzle input."
    fi

    # Create a test input file
    touch $PROJECT/test_input.txt

    # Move to project dir and build
    cd $PROJECT
    cargo build
fi
