#[macro_use]
extern crate serde_derive;
extern crate structopt;
extern crate reqwest;
extern crate dirs;
extern crate serde_yaml;
extern crate url;
extern crate failure;

mod config;

use failure::Error;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    match config::ZDrinksArgs::from_args() {
        init @ config::ZDrinksArgs::Init { .. }                       => config::store_config(init),
        config::ZDrinksArgs::Play { join_game: Some(game_id) } => {
            println!("Play Command. Joining Game {}", game_id);
            Ok(())
        },
        config::ZDrinksArgs::Play { join_game: None }          => {
            println!("Play Command. Starting New Game");
            Ok(())
        }
    }
}