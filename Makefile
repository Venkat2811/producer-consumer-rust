clean:
	cargo clean

build:
	cargo build

build-prod:
	cargo build --release

run-std-demo-optimized:
	cargo run --release --bin main -- --lib std

run-crossbeam-demo-optimized:
	cargo run --release --bin main -- --lib crossbeam

run-disruptor-demo-optimized:
	cargo run --release --bin main -- --lib disruptor

run-all-optimized: run-std-demo-optimized run-crossbeam-demo-optimized run-disruptor-demo-optimized

bench:
	cargo bench
	
