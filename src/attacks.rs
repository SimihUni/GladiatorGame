use std::fmt;


#[derive(PartialEq)]
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