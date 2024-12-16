use ::std::fs;
use std::error::Error;
use std::env;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    let search_results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };
    for result in search_results {
        println!("{result}");
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // first arg is name of the program
        args.next();
        
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("didn't get a query string"),
        };
        let file_path = match args.next(){
            Some(arg)=> arg,
            None => return Err("didn't get file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|l| l.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    file_contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
