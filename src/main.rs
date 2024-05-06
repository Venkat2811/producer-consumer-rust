use clap::{Arg, Command};
use producer_consumer_rust::{std_spsc, std_mpsc, crossbeam_spsc, crossbeam_mpsc, disruptor_spsc, disruptor_mpsc};

fn main() {
    let matches = Command::new("Producer Consumer Demo")
        .version("0.1.0")
        .about("Demonstrates different concurrency libs")
        .arg(Arg::new("lib")
            .short('l')
            .long("lib")
            .value_parser(["crossbeam", "disruptor", "std"])
            .default_value("crossbeam")
            .help("Choose the concurrency lib to run"))
        .get_matches();

    let lib = matches.get_one::<String>("lib").map(String::as_str).unwrap_or("crossbeam");

    match lib {
        "crossbeam" => {
            println!("Running crossbeam demo:");
            crossbeam_spsc(true);
            crossbeam_mpsc(true);
        },
        "disruptor" => {
            println!("Running disruptor demo:");
            disruptor_spsc(true);
            disruptor_mpsc(true);
        },
        "std" => {
            println!("Running std demo:");
            std_spsc(true);
            std_mpsc(true);
        },
        _ => {
            println!("Unknown model, running default crossbeam model:");
            crossbeam_spsc(true);
            crossbeam_mpsc(true);
        }
    }
}
