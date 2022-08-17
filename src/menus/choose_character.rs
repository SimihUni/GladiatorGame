
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
        tio::clear_screen();
        print!("\n\t   SPD|STM|STR");
        print!("\n\t1: 10 |5  |5\n\t2: 5  |10 |5\n\t3: 5  |5  |10\n    Input: \n");
        //input segment
        let input = match tio::get_input() {
            Ok(i) => i,
            Err(e) => panic!("Error in io. {}",e),
        };
        let input = input.as_str().trim();
        //debug
        //println!("{:?}",&input);
        let choice = u8::from_str(input).unwrap_or_default();
        //end of input segment
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
    //input segment
    let input = match tio::get_input() {
        Ok(i) => i,
        Err(e) => panic!("Error in io. {}",e),
    };
    let input = String::from(input.trim());
    //end of input segment

    //debug
    println!("\nName of gladiator : {}",&input);

    match input.as_str() {
        "Max0r" => gladiator_struct::Gladiator::new(input,255,255,255),
        _ => gladiator_struct::Gladiator::new(input,stats.0,stats.1,stats.2)
    }
}