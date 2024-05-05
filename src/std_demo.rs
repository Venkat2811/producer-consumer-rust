use std::thread::{self, JoinHandle};
use std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::mpsc::{self, SyncSender, Receiver};

fn std_spsc(should_print: bool) {
    let buf_size = 32_768;
    let producer_msg_no = 10_000_000;
    let (s, r): (SyncSender<i32>, Receiver<i32>) = mpsc::sync_channel(buf_size);

    let start_time = Instant::now();
    // Producer 
    let t1 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            s.send(1).unwrap();
        }
    });

    let sink = Arc::new(AtomicI32::new(0));
    let sink_clone = Arc::clone(&sink);
    // Consumer
    let c1: JoinHandle<()> = thread::spawn(move || {
        for msg in r {
            let tmp = msg;
            sink_clone.fetch_add(tmp, Ordering::Release);
        }
    });

    let _ = t1.join();
    let _ = c1.join();

    if should_print {
        let d = Instant::now().duration_since(start_time);
        let delta = d.as_millis();

        let sum = sink.load(Ordering::Acquire);
        println!("SPSC Sum: {}, processed time: {}", sum, delta);
    } else {
        sink.load(Ordering::Acquire);
    }
}

fn std_mpsc(should_print: bool) {
    let buf_size = 32_768;
    let producer_msg_no = 10_000_000;
    let (s, r): (SyncSender<i32>, Receiver<i32>) = mpsc::sync_channel(buf_size);
    let s2 = s.clone();

    let start_time = Instant::now();
    // Producer 1
    let t1 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            s.send(1).unwrap();
        }
    });

    // Producer 2
    let t2 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            s2.send(1).unwrap();
        }
    });

    let sink = Arc::new(AtomicI32::new(0));
    let sink_clone = Arc::clone(&sink);
    // Consumer
    let c1: JoinHandle<()> = thread::spawn(move || {
        for msg in r {
            let tmp = msg;
            sink_clone.fetch_add(tmp, Ordering::Release);
        }
    });

    let _ = t1.join();
    let _ = t2.join();
    let _ = c1.join();

    if should_print {
        let d = Instant::now().duration_since(start_time);
        let delta = d.as_millis();

        let sum = sink.load(Ordering::Acquire);
        println!("MPSC Sum: {}, processed time: {}", sum, delta);
    } else {
        sink.load(Ordering::Acquire);
    }
}

fn main() {
    std_spsc(true);
    std_mpsc(true);
}
