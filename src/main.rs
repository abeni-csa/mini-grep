use std::error::Error;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    println!("searcging for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = run(config) {
        println!("Application ERROR: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("\nno enough argumements passed try minigrep string path ");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contests = fs::read_to_string(config.file_path)?;
    println!("with text:\n {contests}");
    Ok(())
}
