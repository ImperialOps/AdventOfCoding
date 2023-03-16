use search::Search;

pub mod search;

fn main() {
    let mut search = Search::new(String::from("test/"), true);

    println!("search directory: {}", search.directory());

    {
        let files = search.files_mut();
        files.push(String::from("test/me"));
    }

    search.files_mut().push(String::from("test/to"));

    println!("search: {:?}", search);
}


