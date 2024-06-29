use CLI_app::*;
use std::env;
use std::process;

fn main() {
    
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem with parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    } 
}

