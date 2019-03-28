#!/bin/bash

command -v hyperfine > /dev/null || (echo "You need to install 'hyperfine' with cargo or brew." && exit 1)
test -e logs/example.log.gz || mkdir logs; curl -L "https://github.com/rubytogether/kirby/releases/download/log-sample/example.log.gz" -o logs/example.log.gz && cp logs/example.log.gz logs/example_2.log.gz && cp logs/example.log.gz logs/example_3.log.gz && cp logs/example.log.gz logs/example_4.log.gz
test -e logs/example.log || gunzip -c logs/example.log.gz  >  logs/example.log && cp logs/example.log logs/example_2.log && cp logs/example.log logs/example_3.log && cp logs/example.log logs/example_4.log

cargo build --release

echo
echo "commit $(git log -1 --pretty=format:%H)"
hyperfine --warmup 3 'target/release/mongo-import-rs logs/example.log.gz' 'mongoimport --drop -d kirby -c bench logs/example.log'
hyperfine --warmup 3 'target/release/mongo-import-rs logs/example*.log.gz' 'mongoimport --drop -d kirby -c bench logs/example.log && mongoimport -d kirby -c bench logs/example_2.log && mongoimport -d kirby -c bench logs/example_3.log && mongoimport -d kirby -c bench logs/example_4.log'
