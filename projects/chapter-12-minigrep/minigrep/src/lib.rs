use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        if args.len() == 4 {
           case_sensitive = parse_case_sensitivity_arg(args[3].clone());
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
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
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn parse_case_sensitivity_arg(arg: String) -> bool {
    let normalized_arg = arg.to_lowercase();
    if normalized_arg == "insensitive" {
        return false;
    } else {
        return true;
    }
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
Duct tape.";

       assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}

    #[test]
    fn case_sensitivity_arg_sensitive() {
        let case_sensitivity_arg = String::from("sensitive");
        assert_eq!(true, parse_case_sensitivity_arg(case_sensitivity_arg));
    }

    #[test]
    fn case_sensitivity_arg_insensitive() {
        let case_sensitivity_arg = String::from("insensitive");
        assert_eq!(false, parse_case_sensitivity_arg(case_sensitivity_arg));
    }

    #[test]
    fn case_sensitivity_arg_invalid() {
        let case_sensitivity_arg = String::from("invalid_arg");
        assert_eq!(true, parse_case_sensitivity_arg(case_sensitivity_arg));
    }

