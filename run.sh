#!/bin/bash

cargo build --release

bin=./target/release/shared-memory-ipc

$bin consume &
consumer_pid=$!

producer_pids=()
for i in {1..5}; do
    $bin produce &
    producer_pids+=($!)
done

for pid in "${producer_pids[@]}"; do
    wait $pid
done

kill $consumer_pid
