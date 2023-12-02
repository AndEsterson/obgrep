use std::fs;
use std::io;
use std::error::Error;
use std::collections::HashSet;
use std::process::Command;
use walkdir::WalkDir;


fn find_files(query: &str, path: &String) {
    let mut matched_files = HashSet::new();
    let mut file_num: u8 = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file = entry.path().display();
        let filename = entry.path().to_string_lossy().into_owned();
        match fs::read_to_string(entry.path()) {
            Ok(contents) => {
                for line in search(&query, &contents) {
                    matched_files.insert(filename.clone());
                    println!("({0}) {file}: {line}", file_num.to_string());
                    file_num += 1
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    println!("{} matches", matched_files.len());
    let v: Vec<_> = matched_files.into_iter().collect();
    user_response(v);
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
                println!("Invalid number");
                continue;
            }
        };
        if v.len() <= requested_num {
            println!("Invalid number");
            continue;
        }
        println!("file is {}", v[requested_num]);
        let status = Command::new("open")
            .arg(&v[requested_num])
            .status();
        break Ok(());
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    find_files(&config.query, &config.file_path);
    Ok(())
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

