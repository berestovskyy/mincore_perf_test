#!/bin/bash

# Script that runs a parameter sweep for the rust program defined in
# this project. The script returns a csv file with the results of the
# following type:
# region_size, percentage_pages, time

# The output of the rust program is something like:
# Time taken: 88.30505ms.

cargo build --release

REGION_SIZES=(1 2 4 8 10 12 16 20 24 30)
PERCENTAGES=(1 5 10 20 25 50 75 80 90 100)

echo "region_size,percentage_pages,time" > results.csv

for region_size in ${REGION_SIZES[@]}; do
    for percentage in ${PERCENTAGES[@]}; do
        echo "Running with region_size: $region_size, percentage: $percentage"
        output_result=`./target/release/pagemap_perf_test --region-size=${region_size} --percentage-pages=${percentage}`
        result_time=$(echo $output_result | grep -oP 'Time taken: \K[0-9.]+')
        echo "$region_size,$percentage,$result_time" >> results.csv
    done
done