use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments{err}");
        process::exit(1);
    });

    println!("searcging for {}", config.query);
    println!("In file {}", config.file_path);
    let contests =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("with text:\n {contests}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("no enough argumements passed \ntry minigrep string path ");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
