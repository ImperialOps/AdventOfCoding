use std::env;
use std::process;

use rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = rust::run(config) {
        println!("application error: {e}");
        process::exit(1);
    }
}
