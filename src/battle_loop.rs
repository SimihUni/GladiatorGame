use crate::gladiator_struct::Gladiator;
use crate::battle_struct::Battle;

pub fn battle_loop(battle_order: &u16, player: Gladiator){
    //idea: could add intro depending on what battle it is.

    //here should be a switch with enemies depending what battle order is, or randomized enemies made to match or challenge player
    let mut enemy = Gladiator::new("Grog".to_string(),4,3,4);

    //creating battle structure that controls the data that is relevent to the current battle.
    let mut battle_info = Battle::new(&player,&enemy);
    
}