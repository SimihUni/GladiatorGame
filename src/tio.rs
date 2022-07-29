//terminal io operations

use std::io;

///gets a line of input from stdin
pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}