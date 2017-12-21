use std::env;
use std::process;

mod prob_1_a;

fn main() {
    match env::args().nth(1) {
        Some(prob) => {
            match prob.as_ref() {
                "1A" => prob_1_a::main(),
                _ => {
                    eprintln!("Solution not implemented yet.");
                    process::exit(1);
                }
            }
        },
        None => {
            eprintln!("Please provide the problem number as first argument.");
            process::exit(1);
        }
    }
}