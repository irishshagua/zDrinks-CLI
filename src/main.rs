extern crate structopt;
extern crate reqwest;

use structopt::StructOpt;
use reqwest::Url;

#[derive(StructOpt)]
#[structopt(name = "zDrinks", about = "The latest drinking game")]
enum ZDrinksArgs {
    #[structopt(name = "init", help = "Store the zDrinks configuration options in a DOT file for quicker start-up")]
    Init {
        #[structopt(short = "n", long = "user_name", help = "The unique id that you will use to join games")]
        name: String,
        #[structopt(short = "t", long = "target_game_server_url", help = "The address of the service API for zDrinks games")]
        target: Url,
    },
    #[structopt(name = "play")]
    Play {
        #[structopt(long = "g", long = "join_game", help = "Optional ID of a game that you want to join")]
        join_game: Option<String>,
    }
}

fn main() {
    let opt = ZDrinksArgs::from_args();
    match opt {
        ZDrinksArgs::Init { name, target }             => println!("Init Command. Name {} Target {}", name, target),
        ZDrinksArgs::Play { join_game: Some(game_id) } => println!("Play Command. Joining Game {}", game_id),
        ZDrinksArgs::Play { join_game: None }          => println!("Play Command. Starting New Game")
    }
}