#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

year=$1
day=$2

padded_day=$(printf "%02d" "$day")

input_file="inputs/year${year}/day${padded_day}.txt"

# Check if input file exists
if [ ! -f "$input_file" ]; then
    echo "Input file not found, fetching..."
    cargo run --bin fetch_input -- "$year" "$day"
    
    if [ $? -ne 0 ]; then
        echo "Failed to fetch input file"
        exit 1
    fi
fi

cargo run --bin advent_of_code -- run "$year" "$day"
