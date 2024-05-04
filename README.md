# producer-consumer-rust

Evaluating `crossbeam` & `disruptor` based on Blog Post: https://medium.com/@trunghuynh/rust-101-rust-crossbeam-vs-java-disruptor-a-wow-feeling-27eaffcda9cb.

We use 2 producer threads and 1 consumer thread in MPSC & 1 producer thread and 1 consumer thread in SPSC setup.

- [crossbeam crate](https://crates.io/crates/crossbeam)
- [disruptor create](https://crates.io/crates/disruptor) 

## Understanding Disruptor

My Blog post: https://venkat.eu/the-power-of-mechanical-sympathy-in-software-engineering#heading-lmax-disruptor

## Benchmarks

### disruptor lib official benchmark

See [disruptor results](https://github.com/nicholassm/disruptor-rs?tab=readme-ov-file#performance)

On my `13-inch, M1, 2020, 16 GB MacBook Pro`:
- disruptor takes, ~150ms in SPSC & ~700ms in MPSC
- crossbeam takes, ~230ms in SPSC & ~550ms in MPSC

```bash
$ make run-disruptor-demo-optimized
cargo run --release --bin disruptor_demo
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/disruptor_demo`
SPSC Sum: 10000000, processed time: 141
MPSC Sum: 20000000, processed time: 694
```

```bash
$ make run-crossbeam-demo-optimized
cargo run --release --bin crossbeam_demo
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/crossbeam_demo`
SPSC Sum: 10000000, processed time: 242
MPSC Sum: 20000000, processed time: 555
```