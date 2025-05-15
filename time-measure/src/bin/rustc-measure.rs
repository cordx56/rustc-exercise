use std::collections::HashMap;
use std::env;
use std::fs::canonicalize;
use std::process::{Command, Stdio};
use time_measure::*;

fn main() {
    let output = Command::new("cargo")
        .args(["build"])
        .env("RUSTC", canonicalize(env::args().nth(1).unwrap()).unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();

    let mut typeck = HashMap::new();
    let mut borrowck = HashMap::new();
    for line in output.lines() {
        if let Ok(measure) = serde_json::from_str::<TimeMeasure>(line) {
            match measure {
                TimeMeasure::TypeCheck { krate, time } => {
                    *typeck.entry(krate).or_insert(0) += time.parse::<u128>().unwrap();
                }
                TimeMeasure::BorrowCheck { krate, time } => {
                    *borrowck.entry(krate).or_insert(0) += time.parse::<u128>().unwrap();
                }
                TimeMeasure::Whole { krate, time } => {
                    println!(
                        "crate {krate} => whole {} ms, typeck: {} ms, borrowck: {} ms",
                        time.parse::<u128>().unwrap() / 1_000_000,
                        typeck.get(&krate).unwrap_or(&0) / 1_000_000,
                        borrowck.get(&krate).unwrap_or(&0) / 1_000_000,
                    );
                }
            }
        }
    }
}
