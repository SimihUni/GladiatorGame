use crate::gladiator_struct::Gladiator;
use crate::battle::battle_struct::Battle;
use crate::battle::battle_choose_move::battle_choose_move;

///main function for logic of a battle
pub fn battle_loop(battle_order: &mut u16, player: &mut Gladiator) {
    //idea: could add intro depending on what battle it is.

    //here should be a switch with enemies depending what battle order is, or randomized enemies made to match or challenge player
    //temporary enemy for testing
    let enemy = Gladiator::new("Grog".to_string(),4,3,4);

    //creating battle structure that controls the data that is relevent to the current battle.
    let mut battle_info = Battle::new(&player,&enemy);

    //idea: add turn limit, to balance infinite move uses --> Done
    let turn_limit: u32 = *battle_order as u32 + 5;
    
    //loop
    loop {
        //idea: add items, which means adding a currency, and menu for choosing items and so on
        //idea: add move uses to balance strong moves --> Obsolete because turn limit
        //idea: add battles against two enemies (not likely to happen)

        //screen with battle stats
        println!("{}",battle_info);
        println!("{} turns remaining.",turn_limit-battle_info.get_turn_count());

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
                    //idea: if battle is successful give player tp ( and money if shop is implemented)
                    println!("Player won.");
                    //adding 2 TP to player
                    player.set_tp(player.get_tp() + 2);
                    *battle_order += 1;
                    break
                }
                else {
                    println!("Enemy won.");
                    //lower difficulty
                    if *battle_order > 1 {
                        *battle_order -= 1;
                    }
                    break
                }
            },
            None => {
                if turn_limit == battle_info.get_turn_count() {
                    println!("Draw. Both gladiators survived the battle.");
                    //adding 1 TP to player
                    player.set_tp(player.get_tp() + 1);
                    *battle_order += 1;
                    break
                }
                else {
                    continue;
                }
            },
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