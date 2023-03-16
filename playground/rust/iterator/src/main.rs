use std::str;

fn main() {
    let s = String::from("hello there");

    let b = s.as_bytes();

    for (i, &value) in b.iter().enumerate() {
        println!("{i}, {}", str::from_utf8(&[value+1, value]).unwrap());
    }

    for i in 0..b.len() {
        println!("{}", str::from_utf8(&[b[i]]).unwrap());
    }
    println!("{}", str::from_utf8(&b).unwrap());
}
