extern crate structopt;
extern crate reqwest;
extern crate dirs;
extern crate toml;

use structopt::StructOpt;
use reqwest::Url;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "zDrinks", about = "The latest drinking game")]
enum ZDrinksArgs {
    #[derive(Deserialize)]
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
        ZDrinksArgs::Init { .. }                       => write_prefs_to_fs(opt),
        ZDrinksArgs::Play { join_game: Some(game_id) } => println!("Play Command. Joining Game {}", game_id),
        ZDrinksArgs::Play { join_game: None }          => println!("Play Command. Starting New Game")
    }
}

fn write_prefs_to_fs(_prefs: ZDrinksArgs) {
    let maybe_prefs_file: Option<PathBuf> = dirs::home_dir()
        .map(|home_dir| {
            home_dir.join(".zdrinks").join("settings.toml")
        });

    match maybe_prefs_file {
        Some(ref file) if file.exists() => println!("Prefs file already exists: {:?}", file),
        Some(file)                      => println!("No current prefs. Create: {:?}", file),
        None                            => eprintln!("Could not resolve home directory to store preferences")
    }
}