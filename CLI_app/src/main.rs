use std::env;
use std::error::Error;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem with parsing arguments: {err}");
        process::exit(1);
    });
    
    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>>  {
 
    let contents = fs::read_to_string(config.file_path)?; 
    
    println!("{contents}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str>{   
        if args.len() < 3 {
            return Err("There need to be 2 arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}
