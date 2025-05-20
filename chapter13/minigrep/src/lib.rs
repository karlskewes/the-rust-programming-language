use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // `new()` returning an error is apparently unidiomatic:
    // "change the function name from new to build because many programmers expect new functions to never fail"
    // `try_new()` is common in the standard library.
    // Also could return Result<Config, &'static str> with Err("some error") rather
    // than a typed error.
    // TBD: error wrapping for different consumers, operators, developers, users...
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, io::Error> {
        args.next(); // skip name of program.

        let query: String = match args.next() {
            Some(arg) => arg,
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "query must be provided",
                ));
            }
        };
        let file_path: String = match args.next() {
            Some(arg) => arg,
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "file path must be provided",
                ));
            }
        };
        // not technically correct:
        //   - IGNORE_CASE=false cargo run ... case insensitive.
        //   - let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        // correct implementation below, supports 4th arg and env var.
        //   - cargo run <word> <file> true ... case insensitive.
        let val: String = args.next().map_or_else(
            || {
                env::var("IGNORE_CASE").map_or_else(
                    |_| "false".to_string(), // fallback
                    |v| v.to_lowercase(),
                )
            },
            |arg| arg.to_lowercase(),
        );

        let ignore_case: bool = val == "true" || val == "1";

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.file_path);

    let contents = fs::read_to_string(cfg.file_path)?;

    let results = if cfg.ignore_case {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // println!("With text:\n{contents}");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_result() {
        let results: Vec<&str> = Vec::new();
        let query = "golang";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(results, search(query, contents));
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "body";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
        ";
        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!",
            ],
            search(query, contents)
        );
    }

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
    fn case_insenstive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
