# producer-consumer-rust

Evaluating `crossbeam` & `disruptor` based on Blog Post: https://medium.com/@trunghuynh/rust-101-rust-crossbeam-vs-java-disruptor-a-wow-feeling-27eaffcda9cb.

We use 2 producer threads and 1 consumer thread in MPSC & 1 producer thread and 1 consumer thread in SPSC setup.

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

Benchmark using [criterion create](https://crates.io/crates/criterion) as recommended [here](https://github.com/nicholassm/disruptor-rs/issues/7#issuecomment-2094345258) 

```bash
$ make bench
```
