
use crate::gladiator_struct;
use crate::tio;
use std::str::FromStr;
use std::io;
use std::io::Write;

///menu for choosing starting stats
pub fn choose_character() -> gladiator_struct::Gladiator {
    let stats:(u8,u8,u8);
    //menu for starting stats
    loop {
        print!("\n\t   SPD|STM|STR");
        print!("\n\t1: 10 |5  |5\n\t2: 5  |10 |5\n\t3: 5  |5  |10\n    Input: ");
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap_or_default();
        match choice {
            1 => {stats = (10,5,5); break;},
            2 => {stats = (5,10,5); break;},
            3 => {stats = (5,5,10); break;},
            _ => continue,
        }
    }
    //prompt for character name
    io::stdout().flush().unwrap();
    println!("Choose a name for your gladiator: ");
    let input = match tio::get_input() {
        Ok(i) => i,
        Err(e) => panic!("Error in io. {}",e),
    };
    let input = String::from(input.trim());
    
    //debug
    println!("name of gladiator = {:?}",&input);

    gladiator_struct::Gladiator::new(input,stats.0,stats.1,stats.2)
}