MONGO IMPORT RS
===============

Inspired by [kirby](https://github.com/rubytogether/kirby/issues/13), this tool imports multiples gzip JSON logs files into MongoDB.

The objective of this tool is not to replace the official [`mongoimport` tool](https://docs.mongodb.com/manual/reference/program/mongoimport/) (See benchmarks). It was created only to test the kirby and MongoDB in an easy way, that is importing multiples gzip files in a unique command.


INSTALLATION
------------

```
sudo apt install musl-tools
rustup target add x86_64-unknown-linux-musl
rustup component add rustfmt
rustup component add clippy
make release
```

BENCHMARKS
----------
See `bench.sh`.

```
Benchmark #1: target/release/mongo-import-rs logs/example.log.gz
  Time (mean ± σ):     74.794 s ±  0.286 s    [User: 60.137 s, System: 1.617 s]
  Range (min … max):   74.395 s … 75.296 s    10 runs

Benchmark #2: mongoimport --drop -d kirby -c bench logs/example.log
  Time (mean ± σ):     18.854 s ±  0.234 s    [User: 33.608 s, System: 0.985 s]
  Range (min … max):   18.543 s … 19.351 s    10 runs

Summary
  'mongoimport --drop -d kirby -c bench logs/example.log' ran
    3.97 ± 0.05 times faster than 'target/release/mongo-import-rs logs/example.log.gz'
```

Results for a 4 core processor (i5-4590 CPU @ 3.30GHz). `mongo-import-rs` only uses all the cores with multiples input files. Benchmarks for 4 input files:

 * mongo-import-rs command: `target/release/mongo-import-rs logs/example*.log.gz`
 * mongoimport command: `mongoimport --drop -d kirby -c bench logs/example.log && mongoimport -d kirby -c bench logs/example_2.log && mongoimport -d kirby -c bench logs/example_3.log && mongoimport -d kirby -c bench logs/example_4.log`

```
Benchmark #1: mongo-import-rs
  Time (mean ± σ):     88.415 s ±  0.982 s    [User: 276.616 s, System: 7.947 s]
  Range (min … max):   87.624 s … 90.990 s    10 runs

Benchmark #2: mongoimport
  Time (mean ± σ):     75.534 s ±  0.429 s    [User: 134.216 s, System: 3.886 s]
  Range (min … max):   74.953 s … 76.247 s    10 runs

Summary
  'mongoimport' ran
    1.17 ± 0.01 times faster than 'mongo-import-rs'
```
