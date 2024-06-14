use colored::*;
use std::fs;
use std::{
    env::current_dir,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    process::Command,
};
use text_io::read;

pub struct OperationsController;

impl OperationsController {
    pub fn save_path(path: &PathBuf) {
        let path_to_save = current_dir().unwrap();
        match OpenOptions::new().append(true).open(path) {
            Ok(mut file) => {
                if let Some(last) = path_to_save.file_name().unwrap().to_str() {
                    let data = format!("\n{} = {:?}", last, &path_to_save);
                    file.write(data.as_bytes()).expect("failed to save path");
                    println!("{}", "Path saved!".green());
                }
            }
            Err(e) => {
                println!("could not save path, error: {}", e);
            }
        }
    }

    pub fn change_to(path: &PathBuf) -> Result<(), std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut paths = Vec::new();

        for line in reader.lines() {
            let str = line.unwrap();
            if str != "[paths]" && str != "" {
                paths.push(str);
            }
        }

        let mut query = String::new();

        for path in &paths {
            let split = path.split("=").last().unwrap().trim();
            let cleaned_path = split.trim_start_matches('"').trim_end_matches('"');
            query.push_str(cleaned_path);
            query.push_str(" ");
        }

        let c = format!("find {} -maxdepth 0 -type d | fzf", query);
        Command::new("sh")
            .arg("-c")
            .arg(c)
            .status()
            .expect("cd command failed to start");

        Ok(())
    }

    pub fn list_paths(path: &PathBuf) -> Result<(), std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut count = 1;
        let mut paths = Vec::new();
        println!("");

        for line in reader.lines() {
            let str = line.unwrap();
            if str != "[paths]" && str != "" {
                println!("({})  {}", count, str);
                count += 1;
                paths.push(str);
            }
        }

        Ok(())
    }

    pub fn clear_all(path: &PathBuf) -> std::io::Result<()> {
        OpenOptions::new().write(true).truncate(true).open(path)?;
        fs::write(path, b"[paths]")?;
        Ok(())
    }

    pub fn remove_one(path: &PathBuf) -> Result<(), io::Error> {
        let _ = Self::list_paths(path);
        println!("");
        println!("Remove:");
        let input: i32 = read!();

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut count = 1;
        let mut paths = Vec::new();
        let mut found = false;

        for line in reader.lines() {
            let str = line.unwrap();
            if str != "[paths]" && str != "" {
                if count == input {
                    found = true;
                }
                if count != input {
                    // println!("({}) - {}", count, str);
                    paths.push(str);
                }
                count += 1;
            }
        }

        if found {
            let _ = Self::clear_all(path);
            match OpenOptions::new().append(true).open(path) {
                Ok(mut file) => {
                    for path in &paths {
                        let data = format!("\n{}", path);
                        file.write(data.as_bytes()).expect("failed to save path");
                    }
                    return Ok(());
                }
                Err(e) => {
                    println!(
                        "could not remove path from shapeshifter.toml file, error: {}",
                        e
                    );
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Path number not found. Please select a valid path number from the list.",
        ))
    }

    pub fn get_one(path: &PathBuf, num: usize) -> Result<(), io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut count = 1;

        for line in reader.lines() {
            let str = line.unwrap();
            if str != "[paths]" && str != "" {
                if count == num {
                    let split = str.split("=").last().unwrap().trim();
                    let cleaned_path = split.trim_start_matches('"').trim_end_matches('"');
                    println!("{}", cleaned_path);
                    return Ok(());
                }
                count += 1;
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Path number not found. Please select a valid path number from the list.",
        ))
    }

    pub fn help() {
        println!(
            "{}         {}",
            "Shapeshifter".green(),
            "Unleash the Power of Quick Shifts!\n"
        );
        println!("{} {}", "Usage:".green().bold(), "shs".cyan());
        println!(
            "   {}               {}",
            "shs".cyan(),
            "Retrieve all saved paths, pipe them to fzf, and change directory after selection"
        );
        println!(
            "   {}      {}\n",
            "shs + number".cyan(),
            "Navigate to the path saved at the specified number"
        );

        println!("\n{} {}\n", "Usage:".green().bold(), "shs [command]".cyan());
        println!("{}", "Commands:".green().bold());

        println!(
            "   {}           {}",
            "help, h".cyan(),
            "List all commands available"
        );
        println!(
            "   {}           {}",
            "list, l".cyan(),
            "List all saved paths"
        );
        println!(
            "   {}           {}",
            "save, s".cyan(),
            "Save the current path"
        );
        println!(
            "   {}          {}",
            "clear, c".cyan(),
            "Clear all saved paths"
        );
        println!(
            "   {}         {}",
            "remove, r".cyan(),
            "Remove a specific path"
        );
    }
}
