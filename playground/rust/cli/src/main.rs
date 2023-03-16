use std::env;
use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = env::args().collect();
    let binary: Vec<&str> = args[0].split("/").collect();

    let exe: &str = match binary.len().cmp(&0) {
        Ordering::Greater => binary[binary.len()-1],
        _ => binary[0],
    };

    print!("{exe}")
}
