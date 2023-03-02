fn main() {
    println!("Hello, world!");

    print_something();
    print_this(23);

    let five = five();
    println!("The value of five is: {five}");

    let mut out_counter = 0;
    'outer: loop {
        let mut inner_counter = 0;
        out_counter += 1;

        loop {
            inner_counter += 1;
            println!("inner counter: {inner_counter}");
            println!("outer counter: {out_counter}");

            if out_counter == 10 {
                break 'outer;
            }

            if inner_counter == 2 {
                break;
            }
        }
    }

    let mut while_counter = 0;
    while while_counter < 10 {
        while_counter += 1;
        println!("while counter: {while_counter}");
    }
}

fn print_something() {
    println!("Hello Something");
}

fn print_this(s: i32) {
    println!("The value is {s}");
}

fn five() -> i8 {
    5
}
