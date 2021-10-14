use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let filename: String = args[2].clone();

    Config { query, filename }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searchinng for {}", config.query);
    println!("In file {}", config.filename);

    let content: String = fs::read_to_string(config.filename).expect("File not found");
    println!("{}", content);
}
