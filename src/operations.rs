use std::{
    env::{current_dir, set_current_dir},
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
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
                }
            }
            Err(e) => {
                println!("could not save path, error: {}", e);
            }
        }
    }

    pub fn list_paths(path: &PathBuf) -> Result<(), std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut count = 1;
        let mut paths = Vec::new();

        for line in reader.lines() {
            let str = line.unwrap();
            if str != "[paths]" && str != "" {
                println!("{} - {}", count, str);
                count += 1;
                paths.push(str);
            }
        }

        println!("\nShift to: \n");
        let mut idx: usize = read!();
        // println!("{}", paths[idx - 1]);

        while idx >= count {
            println!("invalid number, try again: \n");
            idx = read!();
        }

        println!("idx: {}", idx);
        println!("count: {}", count);
        println!("{}", paths[idx - 1]);

        let splited_path = paths[idx - 1].split("=").last().unwrap().trim();
        let cleaned_path = splited_path.trim_start_matches('"').trim_end_matches('"');
        Self::change_dir(PathBuf::from(cleaned_path));

        Ok(())
    }

    pub fn change_dir(path: PathBuf) -> std::io::Result<()> {
        // set_current_dir(path)?;
        let c = format!("cd {}", path.to_string_lossy());


        Command::new("sh")
            .arg("-c")
            .arg(format!(". shapeshifter.sh {}", path.to_string_lossy()))
            .status()
            .expect("cd command failed to start");

        // println!("Current directory changed to: {:?}", current_dir()?);
        Ok(())
    }
}
