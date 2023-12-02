use std::fs;
use std::io;
use std::error::Error;
use std::collections::HashSet;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut matched_files = HashSet::new();
    let mut file_num: u8 = 0;
    let walker = WalkDir::new(&config.file_path).into_iter();
    for result in walker.filter_entry(|e| !is_hidden(e)) {
        match result {
            Ok(entry) => {
                let file = entry.path().display();
                let filename = entry.path().to_string_lossy().into_owned();
                match fs::read_to_string(entry.path()) {
                    Ok(contents) => {
                        for line in search(&config.query, &contents) {
                            if !matched_files.contains(&filename) {
                                file_num += 1
                            };
                            matched_files.insert(filename.clone());
                            println!("({0}) {file}: {line}", file_num.to_string());
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    if matched_files.len() > 0 {
        let v: Vec<_> = matched_files.into_iter().collect();
        user_response(v);
    }
    Ok(())
}

fn user_response(v: Vec<String>) -> Result<(), Box<dyn Error>> {
    loop {
        let mut requested_num = String::new();
        io::stdin()
            .read_line(&mut requested_num)
            .expect("failed to read line");
        let requested_num: usize= match requested_num.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Not a number");
                continue;
            }
        };
        if v.len() < requested_num {
            println!("Invalid number");
            continue;
        }
        let status = Command::new("open")
            .arg("-a")
            .arg("obsidian")
            .arg(&v[requested_num - 1])
            .status();
        break Ok(());
    }
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
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

