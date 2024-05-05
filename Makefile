run-std-demo-optimized:
	cargo run --release --bin std_demo

run-crossbeam-demo-optimized:
	cargo run --release --bin crossbeam_demo

run-disruptor-demo-optimized:
	cargo run --release --bin disruptor_demo

run-all-optimized: run-std-demo-optimized run-crossbeam-demo-optimized run-disruptor-demo-optimized
	
