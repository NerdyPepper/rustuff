use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::env;

// this is the comment
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(argvec: &[String]) -> Result<Config, &'static str> {
        if argvec.len() < 3 {
            return Err("not enough args");
        }

        let query = argvec[1].clone();
        let filename = argvec[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok( Config {
            query,
            filename,
            case_sensitive,
        } )
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut filecontents = String::new();
    f.read_to_string(&mut filecontents)?;

    let results = if config.case_sensitive {
        search(&config.query, &filecontents)
    } else {
        search_case_insensitive(&config.query, &filecontents)
    };

    for item in results {
        println!("{}", item);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

mod test {
    use super::*;

    #[test]
    fn first_search() {
        let query = "iS";
        let contents = "\
        This is just
        fine";

        assert_eq!(
            vec!["This is just"],
            search_case_insensitive(query, contents)
            );
    }
}
