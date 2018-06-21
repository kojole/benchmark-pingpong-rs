extern crate ppbench;

use std::env;
use std::process::exit;
use std::time::Instant;

use ppbench::*;

const USAGE: &'static str = "
Benchmark of pingpong channel messaging.

Usage:
  ppbench [option] <kind> <count>
  ppbench (-h | --help)

Arguments:
  kind   Kinds of pingpong:
           Ping        One-way messaging (request).
           PingPong    Two-way messaging (request and response).
           CBPing      crossbeam-channel version of Ping.
           CBPingPong  crossbeam-channel version of PingPong.
  count  # of pingpong.

Options:
  -h --help      Show this message.
";

fn parse_args() -> Result<(Kind, usize), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("{}", USAGE);
        exit(0);
    }

    if args.len() < 3 {
        println!("{}", USAGE);
        exit(1);
    }

    let kind = Kind::try_from(&args[1])
        .map_err(|err| format!("Invalid argument <kind>: {}: {}", err, args[1]))?;

    let count = args[2]
        .parse::<usize>()
        .map_err(|err| format!("Invalid argument <count>: {}: {}", err, args[2]))?;

    Ok((kind, count))
}

fn main() {
    let (kind, count) = parse_args().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    let start = Instant::now();
    match kind {
        Kind::Ping => ping(count),
        Kind::PingPong => pingpong(count),
        Kind::CbPing => cb_ping(count),
        Kind::CbPingPong => cb_pingpong(count),
    };
    let elapsed = start.elapsed();

    println!(
        "{:.6}",
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9
    );
}
