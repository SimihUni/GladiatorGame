use std::fmt;
use crate::gladiator_struct::{self, Gladiator};

#[derive(PartialEq,Clone,Copy)]
pub enum Attack {
    Stab,
    Dodge,
    Tackle,
    Smash,
    Stun,
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Attack::Stab => write!(f, "Stab"),
           Attack::Dodge => write!(f, "Dodge"),
           Attack::Tackle => write!(f, "Tackle"),
           Attack::Smash => write!(f, "Smash"),
           Attack::Stun => write!(f, "Stun"),
       }
    }
}

impl Attack {
    pub fn calculate_damage(&self,player: &Gladiator, enemy: &Gladiator, is_hit:&mut bool) -> u32 {
        let damage = match *self {
            Attack::Stab => (player.get_str() as f32 * 1.5).floor() as u32,
            Attack::Dodge => 0,
            Attack::Tackle => (player.get_str() * 2) as u32,
            Attack::Smash => (player.get_str() * 3) as u32,
            Attack::Stun => 5,
        };
        if self.check_if_hit(player,enemy){
            *is_hit = true;
            return damage;
        }
        else {
            *is_hit = false;
            //display fail to hit message TODO
            return 0;
        }
    }

    pub fn check_if_hit(&self,player: &Gladiator,enemy: &Gladiator) -> bool {
        //add random element here
        true
    }
}