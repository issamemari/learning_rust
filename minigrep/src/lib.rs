use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct InputArgs {
    pub query: String,
    pub file_path: String,
}

impl InputArgs {
    pub fn new(args: &[String]) -> Result<InputArgs, String> {
        let len = args.len();
        if len != 3 {
            return Err(format!("expected 2 arguments, got {}", len - 1));
        }

        Ok(InputArgs {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(args: InputArgs) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    for line in search(&args.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
