extern crate gladiator;
use gladiator::menus;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
author="\nAuthor:Simeon Mihaylov\n\n", 
version,
about="A small gladiator turn-based rpg game for the terminal.",
long_about = "A small gladiator turn-based rpg game for the terminal. The goal of the game is to defeat the last boss named \"Boss Man\". The game uses numbered menus. The player can train up to five available moves, and train their 3 basic stats. The battle oponents are chosen randomly from a list, based on the current difficulty level, that changes through battle results. Every battle length as in turn is determined by the difficulty level. If the battle difficulty is high enough the last boss will be available to fight. Good Luck"
)]
struct Args {}

fn main() {
    let _args = Args::parse();
    menus::start::start_menu();
}