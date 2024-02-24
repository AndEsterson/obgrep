use colored::Colorize;
use grep::{
    regex::RegexMatcher,
    searcher::{sinks::UTF8, BinaryDetection, SearcherBuilder},
};
use std::error::Error;
use std::io;
use std::process::Command;
use urlencoding::encode;
use walkdir::{DirEntry, WalkDir};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut matches: Vec<(String, u64, String)> = vec![];
    let matcher = RegexMatcher::new_line_matcher(&config.query)?;
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(true)
        .build();
    for entry in WalkDir::new(&config.file_path)
        .into_iter()
        .filter_entry(is_not_dotfile)
        .filter_map(|v| v.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let result = searcher.search_path(
            &matcher,
            entry.path(),
            UTF8(|lnum, line| {
                matches.push((
                    entry.path().to_string_lossy().into_owned(),
                    lnum,
                    line.to_string(),
                ));
                Ok(true)
            }),
        );
        if let Err(err) = result {
            eprintln!("{}: {}", entry.path().display(), err);
        }
    }
    matches.sort_by_key(|(s, _, _)| s.clone());
    let mut matched_files: Vec<&String> = vec![];
    let mut count: u8 = 0;
    for matched in &matches {
        let filename = &matched.0;
        println!(
            "{}){}: {}",
            count.to_string().yellow(),
            matched.0.purple(),
            matched.2.trim()
        );
        if !matched_files.contains(&filename) {
            matched_files.push(filename);
            count += 1;
        }
    }
    if !matches.is_empty() {
        let _response = user_response(matched_files);
    }
    Ok(())
}

fn is_not_dotfile(file: &DirEntry) -> bool {
    file.file_name()
        .to_str()
        .map(|s| file.depth() == 0 || !s.starts_with('.'))
        .unwrap_or(false)
}

fn user_response(matched_files: Vec<&String>) -> Result<(), Box<dyn Error>> {
    loop {
        let mut requested_num = String::new();
        io::stdin()
            .read_line(&mut requested_num)
            .expect("failed to read line");
        let requested_num: usize = match requested_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        if matched_files.len() < requested_num || requested_num == 0 {
            println!("Invalid number");
            continue;
        }
        let _status = Command::new("open")
            .arg("-a")
            .arg("obsidian")
            .arg(format!(
                "obsidian://open?path={}",
                encode(matched_files[requested_num])
            ))
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

        let file_path = args[1].clone();
        let query = args[2].clone();

        Ok(Config { query, file_path })
    }
}
