use std::{
    env::{self, current_dir},
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // let flag = &args[1];

    let path = current_dir()?;
    let path_str = path.to_str();
    // println!("{:?}", path);
    match File::open("/home/hetzwga/.config/shapeshifter.toml") {
        Ok(_) => println!("success"),
        Err(_) => {
            let mut file = File::create_new("/home/hetzwga/.config/shapeshifter.toml")?;
            // let default_txt = "[paths]".to_string();
            file.write_all(b"[paths]")?;
        }
    }
    //verify shapeshifter.conf existence

    //create file if doesnt exist

    //verify flags if succeded

    Ok(())
}
