//terminal io operations

use std::io::{stdin, stdout, Read, Write,Result};
use std::env;

///gets a line of input from stdin
pub fn get_input() -> Result<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

pub fn clear_screen() {
    //debug
    println!("\n-------------------------------------------------------------------------------------------\n");

    match env::consts::OS {
        "linux" => std::process::Command::new("clear"),
        "macos" => std::process::Command::new("clear"),
        "windows" => std::process::Command::new("cls"),
        _ => return,
    };
}

pub fn press_to_continue() {
    let mut stdout = stdout();
    let mut buffer = String::new();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut buffer).unwrap();
}