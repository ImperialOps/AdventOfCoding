pub trait Health {
    fn healthy(&self) -> bool;
}

pub struct Human {
    pub name: String,
    pub height: f32,
    pub weight: u16,
}

impl Human {
    pub fn new(name: String, height: f32, weight: u16) -> Self {
        Self {
            name,
            height,
            weight,
        }
    }
}

impl Health for Human {
    fn healthy(&self) -> bool {
        self.weight < 100
    }
}

fn main() {
    let josh = Human::new(String::from("josh"), 5.9, 80);
    println!("josh is healthy: {}", josh.healthy());

    let number_list = vec![34, 50, 25, 100, 65];

    let small = smallest(&number_list).expect("vector must contain values");
    let large = largest(&number_list).unwrap();

    println!("smallest: {small}");
    println!("largest: {large}");
}

fn smallest(list: &Vec<i32>) -> Option<&i32> {
    if list.len() == 0 {
        None
    } else {
        let mut smallest = &list[0];

        for number in list {
            if number < smallest {
                smallest = number;
            }
        }
        Some(smallest)
    }
}

fn largest(list: &Vec<i32>) -> Option<&i32> {
    if list.len() == 0 {
        None
    } else {
        let mut largest = &list[0];

        for number in list {
            if number > largest {
                largest = number;
            }
        }
        Some(largest)
    }
}
