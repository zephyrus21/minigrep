use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else();

    println!("Searchinng for {}", config.query);
    println!("In file {}", config.filename);

    let content: String = fs::read_to_string(config.filename).expect("File not found");
    println!("{}", content);
}
