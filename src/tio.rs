//terminal io operations

use std::io;
use std::env;

///gets a line of input from stdin
pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
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