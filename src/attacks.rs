use std::fmt;
use crate::gladiator_struct::Gladiator;
extern crate rand;
use rand::Rng;

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
    pub fn calculate_damage(&self,dmg_dealer: &Gladiator, dmg_receiver: &Gladiator, is_hit:&mut bool) -> u32 {
        let damage = match *self {
            Attack::Stab => (dmg_dealer.get_str() as f32 * 1.5).floor() as u32,
            Attack::Dodge => 0,
            Attack::Tackle => (dmg_dealer.get_str() * 2) as u32,
            Attack::Smash => (dmg_dealer.get_str() * 3) as u32,
            Attack::Stun => 5,
        };
        if self.check_if_hit(dmg_dealer,dmg_receiver){
            *is_hit = true;
            return damage;
        }
        else {
            *is_hit = false;
            //display fail to hit message TODO
            return 0;
        }
    }

    pub fn check_if_hit(&self,dmg_dealer: &Gladiator,dmg_receiver: &Gladiator) -> bool {
        let chance = match *self {
            Attack::Stab => 70,
            Attack::Dodge => {
                if dmg_dealer.get_spd() > dmg_receiver.get_spd() {
                    60
                }
                else {
                    10
                }
            },
            Attack::Tackle => 60,
            Attack::Smash => 50,
            Attack::Stun => {
                if dmg_dealer.get_str() > dmg_receiver.get_str() {
                    60
                }
                else {
                    10
                }
            },
        }; // get percentage for hit

        let random_number = rand::thread_rng().gen_range(1..101);

        //for debug
        //println!("random number: {}\nchance: {}",random_number,chance);

        random_number < chance 
    }
}