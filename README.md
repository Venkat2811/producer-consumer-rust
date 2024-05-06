# producer-consumer-rust

Evaluating `std`, `crossbeam`, `disruptor` for inter-thread communication based on [Blog Post](https://medium.com/@trunghuynh/rust-101-rust-crossbeam-vs-java-disruptor-a-wow-feeling-27eaffcda9cb.)

We use 2 producer threads and 1 consumer thread in MPSC & 1 producer thread and 1 consumer thread in SPSC setup.

- [sync module](https://doc.rust-lang.org/std/sync/mpsc/)
- [crossbeam crate](https://crates.io/crates/crossbeam)
- [disruptor create](https://crates.io/crates/disruptor) 

## Understanding Disruptor

My Blog post: https://venkat.eu/the-power-of-mechanical-sympathy-in-software-engineering#heading-lmax-disruptor

## Run

On my `13-inch, M1, 2020, 16 GB MacBook Pro`:

```bash
$ make run-all-optimized 

cargo run --release --bin main -- --lib std
    Finished `release` profile [optimized] target(s) in 1.75s
     Running `target/release/main --lib std`
Running std demo:
SPSC Sum: 10000000, processed time: 140
MPSC Sum: 20000000, processed time: 555

cargo run --release --bin main -- --lib crossbeam
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/main --lib crossbeam`
Running crossbeam demo:
SPSC Sum: 10000000, processed time: 347
MPSC Sum: 20000000, processed time: 249

cargo run --release --bin main -- --lib disruptor
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/main --lib disruptor`
Running disruptor demo:
SPSC Sum: 10000000, processed time: 183
MPSC Sum: 20000000, processed time: 686

```

## Benchmarks

**disruptor lib official benchmark:** See [disruptor results](https://github.com/nicholassm/disruptor-rs?tab=readme-ov-file#performance)

A simple throughput benchmark using [criterion create](https://crates.io/crates/criterion) as recommended [here](https://github.com/nicholassm/disruptor-rs/issues/7#issuecomment-2094345258) 

As of May 6, 2024:

**SPSC:**
Throughput: std > disruptor > crossbeam

**MPSC:**
Throughput: std > mpsc > disruptor
```bash
$ make bench

Benchmarking std_spsc: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.6s, or reduce sample count to 30.
std_spsc                time:   [126.08 ms 127.48 ms 129.76 ms]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

Benchmarking crossbeam_spsc: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 35.9s, or reduce sample count to 10.
crossbeam_spsc          time:   [319.63 ms 326.99 ms 334.20 ms]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high severe

Benchmarking disruptor_spsc: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 25.2s, or reduce sample count to 10.
disruptor_spsc          time:   [291.63 ms 314.71 ms 337.12 ms]

Benchmarking std_mpsc: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 57.4s, or reduce sample count to 10.
std_mpsc                time:   [581.07 ms 589.71 ms 598.16 ms]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

Benchmarking crossbeam_mpsc: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 29.4s, or reduce sample count to 10.
crossbeam_mpsc          time:   [264.37 ms 274.54 ms 286.64 ms]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

Benchmarking disruptor_mpsc: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 80.8s, or reduce sample count to 10.
disruptor_mpsc          time:   [782.81 ms 792.41 ms 801.15 ms]
Found 20 outliers among 100 measurements (20.00%)
  12 (12.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

```
