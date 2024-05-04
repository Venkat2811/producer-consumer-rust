use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Instant;
use disruptor::{build_multi_producer, build_single_producer, BusySpin, Producer};

struct Event {
    val: i32
}

//spsc
fn spsc_example() {
    let buf_size = 32_768;
    let producer_msg_no = 10_000_000;

    let factory = || { Event { val: 0 }}; //to initialize disruptor

    let sink = Arc::new(AtomicI32::new(0)); //bcos we read and print value from main thread
    // Consumer
    let processor = {
        let sink = Arc::clone(&sink);
        move |event: &Event, _sequence: i64, _end_of_batch: bool| {
            sink.fetch_add(event.val, Ordering::Release);
        }
    };

    let mut producer = build_single_producer(buf_size, factory, BusySpin)
        .handle_events_with(
            processor
        )
        .build();

    let start_time = Instant::now();

    // Publish into the Disruptor.
    thread::scope(|s| {
        s.spawn(move || {
            for _ in 0..producer_msg_no {
                producer.publish(|e| {
                    e.val = 1 as i32;
                });
            }
        });
    });

    let d = Instant::now().duration_since(start_time);
    let delta = d.as_millis();

    let sum = sink.load(Ordering::Acquire);
    println!("SPSC Sum: {}, processed time: {}", sum, delta);
}

//mpsc
fn mpsc_example() {
    let buf_size = 32_768;
    let producer_msg_no = 10_000_000;

    let factory = || { Event { val: 0 }}; //to initialize disruptor

    let sink = Arc::new(AtomicI32::new(0)); //bcos we read and print value from main thread
    // Consumer
    let processor = {
        let sink = Arc::clone(&sink);
        move |event: &Event, _sequence: i64, _end_of_batch: bool| {
            sink.fetch_add(event.val, Ordering::Release);
        }
    };

    let mut producer1 = build_multi_producer(buf_size, factory, BusySpin)
        .handle_events_with(
            processor
        )
        .build();

    let mut producer2 = producer1.clone();

    let start_time = Instant::now();

    // Publish into the Disruptor.
    thread::scope(|s| {
        s.spawn(move || {
            for _ in 0..producer_msg_no {
                producer1.publish(|e| {
                    e.val = 1 as i32;
                });
            }
        });

        s.spawn(move || {
            for _ in 0..producer_msg_no {
                producer2.publish(|e| {
                    e.val = 1 as i32;
                });
            }
        });
    });

    let d = Instant::now().duration_since(start_time);
    let delta = d.as_millis();

    let sum = sink.load(Ordering::Acquire);
    println!("MPSC Sum: {}, processed time: {}", sum, delta);
}

fn main() {
    spsc_example();
    mpsc_example();
}