use crate::gladiator_struct::Gladiator;
use crate::battle::battle_struct::Battle;
use crate::battle::battle_choose_move::battle_choose_move;

///main function for logic of a battle
pub fn battle_loop(battle_order: &u16, player: Gladiator) {
    //idea: could add intro depending on what battle it is.

    //here should be a switch with enemies depending what battle order is, or randomized enemies made to match or challenge player
    //temporary enemy for testing
    let enemy = Gladiator::new("Grog".to_string(),4,3,4);

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
        if battle_info.is_player_turn() {
            //potencially add choice of attack or item here
            //choose an attack and deal damage
            battle_info.deal_damage_to_enemy(battle_choose_move(player.get_move_list()));
        }
        else {
            //enemy's turn
            //deal damage
            //choose enemy's attack
            //todo!();
        }

        //check battle state
        match battle_info.check_if_anyone_is_dead() {
            Some(b) => {
                if b {
                    println!("Player won.");
                    break
                }
                else {
                    println!("Enemy won.");
                }
            },
            None => continue,
        };

        //update turn variables
        battle_info.increment_turns();
        //check if a gladiator is stunned and if there is a need to change whose turn it is
        if (battle_info.is_player_turn() && battle_info.is_enemy_stunned()) || (!battle_info.is_player_turn() && !battle_info.is_enemy_stunned()) {
            //display who got stunned
            battle_info.unstun_gladiators();
            continue;
        }

        battle_info.flip_turn();
        //repeat
    }
}