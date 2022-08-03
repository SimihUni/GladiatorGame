use crate::gladiator_struct::Gladiator;
use crate::battle::battle_struct::Battle;


pub fn battle_loop(battle_order: &u16, player: Gladiator) {
    //idea: could add intro depending on what battle it is.

    //here should be a switch with enemies depending what battle order is, or randomized enemies made to match or challenge player
    let mut enemy = Gladiator::new("Grog".to_string(),4,3,4);

    //creating battle structure that controls the data that is relevent to the current battle.
    let mut battle_info = Battle::new(&player,&enemy);
    
    //loop
    loop {
        //idea: add items, which means adding a currency, and menu for choosing items and so on
        //idea: add move uses to balance strong moves
        //idea: add battles against two enemies (not likely to happen)

        //screen with battle stats
        println!("{}",battle_info);

        //choose attack
        todo!();

        //deal damage
        todo!();

        //check battle state
        todo!();

        //update turn variables
        todo!();

        //repeat
    }
}