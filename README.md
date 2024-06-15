# Shapeshifter

## Description
Shapeshifter is a command-line tool written in Rust that allows you to quickly switch between saved directory paths, making navigation in the terminal more efficient.

## Demo


https://github.com/Marlon-Sbardelatti/shapeshifter/assets/117592329/eced1fe6-47f5-4985-a8c7-eada29cd32f4



## Features
- Quick Navigation: Save directory paths as markers and switch between them with a single command.
- Simple CLI Interface: Easy-to-use commands for managing and navigating between saved paths.

## Table of Contents
- [Prerequesites](#prerequesites)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Prerequesites
### Before installing Shapeshifter, ensure you have the following dependencies installed on your system:

* Rust
* fzf
* find command (typically available on Unix-like systems)

### Installing rust
- install Rust from rustup.rs

### Installing fzf and find

#### On Linux:
#### Debian based distros: 
```bash
sudo apt-get install fzf findutils
```
#### Arch: 
```bash
yay -S fzf findutils
```
#### On macOS (using Homebrew):
```bash
brew install fzf findutils
```



## Installation
To use Shapeshifter, you'll need Rust installed on your system. If you haven't already, you can install Rust from rustup.rs.

1. Clone the repository:
```bash
git clone git@github.com:Marlon-Sbardelatti/shapeshifter.git
cd shapeshifter
```
2. Build the project using Cargo (Rust's package manager):
```bash
cargo build --release
```
3. Copy the Shapeshifter executable to /usr/local/bin:
```bash
sudo cp target/release/jargo /usr/local/bin
```
4. Copy the funcition to your .bashrc:
- OBS: We won't directly use the shapeshifter command; instead, we'll create a bash function that acts as an alias, and this function will call our Rust binary. That's why we need this function
```bash
shs(){
    if [[ $1 =~ ^[0-9]+$ ]]; then
        output=$(shapeshifter $1)

    read -ra words <<< "$output"
    last_word="${words[-1]}"

    if [[ $last_word != "99" ]]; then
        cd "$output"
    else 
        echo "$output"
    fi


else
    if [ "$1" = "l" ]; then
        shapeshifter l
    elif [[ "$1" = "r" ]]; then
        shapeshifter r 
    elif [[ "$1" = "c" ]]; then
        shapeshifter c 
    elif [[ "$1" = "s" ]]; then
        shapeshifter s 
    elif [[ "$1" = "h" ]]; then
        shapeshifter h 
    else
        cd $(shapeshifter change)
    fi
    fi

}

```
5. Source .bashrc file:
```bash
source ~/.bashrc
```


## Usage
To use Shapeshifter, you can use the following commands:

## Commands

| Command             | Description                                                                      |
|---------------------|----------------------------------------------------------------------------------|
| `shs`               | Display all saved paths using `fzf` for selection. Change to selected directory. |
| `shs "number"`      | Change to the directory corresponding to the number provided.                     |
| `shs list` or `shs l` | List all saved paths.                                                           |
| `shs save` or `shs s` | Save the current directory path.                                                |
| `shs remove` or `shs r` | Remove a saved path by selecting its number.                                    |
| `shs clear` or `shs c` | Delete all saved paths.                                                         |
| `shs help` or `shs h`  | Display help for all commands.                                                  |



## Contributing
We welcome contributions! Follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (git checkout -b feature/awesome-feature).
3. Commit your changes (git commit -m 'Add some awesome feature').
4. Push to the branch (git push origin feature/awesome-feature).
5. Open a pull request.

## License
This project is licensed under the MIT License.

## Contact
Feel free to reach out with any questions or feedback.

* Email: hetzwga@gmail.com
* GitHub: Marlon-Sbardelatti
