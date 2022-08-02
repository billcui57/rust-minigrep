use std::error::Error;
use std::fs;

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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        let does_contain = match ignore_case {
            true => line.to_lowercase().contains(&query.to_lowercase()),
            false => line.contains(query),
        };
        if does_contain {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String], ignore_case: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}
