use std::{
    env::{self, current_dir},
    fs::{File, OpenOptions},
    io::Write,
};

mod operations;
use operations::*;

use dirs::home_dir;

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
                    OperationsController::get_one(&path, num);
                }
                Err(_) => {
                    //match operations
                    if flag == "l" || flag == "list" {
                        OperationsController::list_paths(&path);
                    } else if flag == "c" || flag == "clear" {
                        OperationsController::clear_all(&path);
                    } else if flag == "s" || flag == "save" {
                        OperationsController::save_path(&path);
                    } else if flag == "change" {
                        OperationsController::change_to(&path);
                    } else if flag == "r" || flag == "remove" {
                        OperationsController::remove_one(&path);
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
    //verify shapeshifter.conf existence

    //create file if doesnt exist

    //verify flags if succeded

    Ok(())
}
