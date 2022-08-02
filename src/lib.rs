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
    contents
        .lines()
        .filter(|line| match ignore_case {
            true => line.to_lowercase().contains(&query.to_lowercase()),
            false => line.contains(query),
        })
        .collect()
}

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(
        mut args: impl Iterator<Item = String>,
        ignore_case: bool,
    ) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}
