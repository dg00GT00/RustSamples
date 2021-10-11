use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let filename = args[2].clone();
        Self { query, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}\n", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
