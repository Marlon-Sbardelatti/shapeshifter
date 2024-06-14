use colored::*;
use std::{
    env::{self},
    fs::File,
    io::Write,
};
mod operations;
use dirs::home_dir;
use operations::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let flag = &args[1];

    let home = home_dir().unwrap();
    let path = home.join(".config/shapeshifter.toml");

    match File::open(&path) {
        Ok(_) => {
            match flag.parse::<usize>() {
                Ok(num) => {
                    //this means that input num == path idx in .toml file
                    //get_path_by_num(flag)
                    match OperationsController::get_one(&path, num) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error: {} Error number {}", e, 99);
                        }
                    }
                }
                Err(_) => {
                    //match operations
                    if flag == "l" || flag == "list" {
                        let _ = OperationsController::list_paths(&path);
                    } else if flag == "c" || flag == "clear" {
                        let _ = OperationsController::clear_all(&path);
                    } else if flag == "s" || flag == "save" {
                        OperationsController::save_path(&path);
                    } else if flag == "change" {
                        let _ = OperationsController::change_to(&path);
                    } else if flag == "r" || flag == "remove" {
                        match OperationsController::remove_one(&path) {
                            Ok(_) => {
                                println!("{}", "Path removed successfully!".green());
                            }
                            Err(e) => println!("{} {}", "Error:".red(), e),
                        }
                    } else if flag == "h" || flag == "help" {
                        OperationsController::help();
                    } else {
                        println!("Unknown command");
                    }
                }
            }
        }
        Err(_) => {
            //.TOML file created
            let mut file = File::create_new(path)?;
            file.write_all(b"[paths]")?;
        }
    }
    Ok(())
}
