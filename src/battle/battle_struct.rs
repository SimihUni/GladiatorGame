use crate::gladiator_struct::Gladiator;
use std::fmt;

pub struct Battle {
    ///is it the players turn
    players_turn: bool,
    ///what turn is it currently
    turn_count: u32,
    player: Gladiator,
    enemy: Gladiator,
    ///is (player,enemy) stunned
    stunned: (bool,bool),
}

impl Battle {
    pub fn new(player: &Gladiator, enemy: &Gladiator) -> Battle {
        let  players_turn: bool = player.get_stats().0 >= enemy.get_stats().0;
        Battle {
            players_turn,
            turn_count: 1,
            player: player.clone(),
            enemy: enemy.clone(),
            stunned: (false,false),
        }
    }

    pub fn unstun_gladiators(&mut self) {
        self.stunned = (false,false)
    }

    pub fn stun_player(&mut self) {
        self.stunned.0 = true
    }

    pub fn is_player_stunned(&self) -> bool {
        self.stunned.0
    }

    pub fn stun_enemy(&mut self) {
        self.stunned.0 = true
    }

    pub fn is_enemy_stunned(&self) -> bool {
        self.stunned.0
    }

    ///invert bool value of who's turn it is
    pub fn flip_turn(&mut self) {
        self.players_turn = !self.players_turn
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

    ///Checks if anyone is dead, 3 possible states: None, Some(true) -> player wins, Some(false) -> enemy wins
    pub fn check_if_anyone_is_dead(&self) -> Option<bool> {
        if self.enemy.get_hp() * self.player.get_hp() == 0 {
            if self.enemy.get_hp() == 0 {
                return Some(true) // player wins
            }
            else {
                return Some(false) // enemy wins
            }
        }
        None
    }
}

impl fmt::Display for Battle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("\n\t Battle between {} and {}\n\n\tPlayer HP:{}\n\tEnemy HP:{}\n\n\tIt is {}'s turn.",
         self.player.get_name(), self.enemy.get_name(),self.player.get_hp(), self.enemy.get_hp(),self.get_this_turn_name()))
    }
}