fn main() {
    println!("Hello, world!");

    let mut hello = String::from("hello there");
    borrow(&hello);

    change(&mut hello);
    println!("{hello}");
}

fn borrow(s: &String) {
    println!("{s}");
}

fn change(s: &mut String) {
    s.push_str(" form space");
}
