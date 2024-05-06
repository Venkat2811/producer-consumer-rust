use std::thread::{self, JoinHandle};
use std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use crate::event::Event;

pub fn std_spsc(should_print: bool) {
    let buf_size = 32_768;
    let producer_msg_no = 10_000_000;
    let (s, r): (SyncSender<Event>, Receiver<Event>) = sync_channel(buf_size); // Update the channel type

    let start_time = Instant::now();
    // Producer 
    let t1 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            let event = Event { val: 1 }; // Send Event instances
            s.send(event).unwrap();
        }
    });

    let sink = Arc::new(AtomicI32::new(0));
    let sink_clone = Arc::clone(&sink);
    // Consumer
    let c1: JoinHandle<()> = thread::spawn(move || {
        for msg in r {
            sink_clone.fetch_add(msg.val, Ordering::Release); // Process Event.val
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

pub fn std_mpsc(should_print: bool) {
    let buf_size = 32_768;
    let producer_msg_no = 10_000_000;
    let (s, r): (SyncSender<Event>, Receiver<Event>) = sync_channel(buf_size);
    let s2 = s.clone();

    let start_time = Instant::now();
    // Producer 1
    let t1 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            let event = Event { val: 1 };
            s.send(event).unwrap();
        }
    });

    // Producer 2
    let t2 = thread::spawn(move || {
        for _ in 0..producer_msg_no {
            let event = Event { val: 1 };
            s2.send(event).unwrap();
        }
    });

    let sink = Arc::new(AtomicI32::new(0));
    let sink_clone = Arc::clone(&sink);
    // Consumer
    let c1: JoinHandle<()> = thread::spawn(move || {
        for msg in r {
            sink_clone.fetch_add(msg.val, Ordering::Release);
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

