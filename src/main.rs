use minigrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("[!] Problem parsing the arguments: {err}");
        process::exit(1);
    });
    println!("[+] searcging for {}", config.query);
    println!("[+] In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("[!] Application ERROR: {e}");
        process::exit(1);
    }
}
