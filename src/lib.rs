use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub  query: String,
    pub file_path: String,
    pub ignore_case: bool, 
}
impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, &'static str>{   
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a file_path string"),
        };

        
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path,
            ignore_case, 
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>  {
 
    let contents = fs::read_to_string(config.file_path)?; 
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
    println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
