pub mod event;
pub mod std_demo;
pub mod crossbeam_demo;
pub mod disruptor_demo;

pub use crate::std_demo::{std_spsc, std_mpsc};
pub use crate::crossbeam_demo::{crossbeam_spsc, crossbeam_mpsc};
pub use crate::disruptor_demo::{disruptor_spsc, disruptor_mpsc};

// fn main() {
//
// }