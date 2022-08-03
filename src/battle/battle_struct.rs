use crate::gladiator_struct::Gladiator;
use std::fmt;

pub struct Battle {
    players_turn: bool, // is it the players turn
    turn_count: u32,    // what turn is it currently
    player: Gladiator,
    enemy: Gladiator,
}

impl Battle {
    pub fn new(player: &Gladiator, enemy: &Gladiator) -> Battle {
        let  players_turn: bool = player.get_stats().0 >= enemy.get_stats().0;
        Battle {
            players_turn,
            turn_count: 1,
            player: player.clone(),
            enemy: enemy.clone(),
        }
    }

    ///increments turn_count with 1
    pub fn increment_turns(&mut self){
        self.turn_count += 1;
    }

    ///check if it is the players turn
    pub fn is_player_turn(&self) -> bool {
        self.players_turn
    }

    ///set if it is the players turn
    pub fn set_player_turn(&mut self, new_value: bool) {
        self.players_turn = new_value;
    }

    ///gets the name of this turn's gladiator
    fn get_this_turn_name(&self) -> String {
        if self.players_turn {
            return self.player.get_name()
        }
        else {
            return self.enemy.get_name()
        }
    }
}

impl fmt::Display for Battle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("\n\t Battle between {} and {}\n\n\tPlayer HP:{}\n\tEnemy HP:{}\n\n\tIt is {}'s turn.",
         self.player.get_name(), self.enemy.get_name(),self.player.get_hp(), self.enemy.get_hp(),self.get_this_turn_name()))
    }
}