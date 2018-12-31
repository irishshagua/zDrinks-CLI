use structopt::StructOpt;
use std::str::FromStr;
use serde::Serializer;
use serde::Serialize;
use reqwest::Url;
use failure::Error;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;

// :-/ tom foolery to get serialization working for Url
// The url lib has a dep on <1.0 serde
pub struct Server(Url);
impl Serialize for Server {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.0.as_str())
    }
}
impl FromStr for Server {
    type Err = url::ParseError;

    #[inline]
    fn from_str(input: &str) -> Result<Server, url::ParseError> {
        Url::parse(input).map(|url| Server(url))
    }
}

#[derive(StructOpt, Serialize)]
#[structopt(name = "zDrinks", about = "The latest drinking game")]
pub enum ZDrinksArgs {
    #[structopt(name = "init", help = "Store the zDrinks configuration options in a DOT file for quicker start-up")]
    Init {
        #[structopt(short = "n", long = "user_name", help = "The unique id that you will use to join games")]
        name: String,
        #[structopt(short = "t", long = "target_game_server_url", help = "The address of the service API for zDrinks games")]
        target: Server
    },
    #[structopt(name = "play")]
    Play {
        #[structopt(long = "g", long = "join_game", help = "Optional ID of a game that you want to join")]
        join_game: Option<String>
    }
}

pub fn store_config(prefs: ZDrinksArgs) -> Result<(), Error> {
    let maybe_prefs_file: Option<PathBuf> = dirs::home_dir()
        .map(|home_dir| { home_dir.join(".zdrinks") });

    match maybe_prefs_file {
        Some(zdrinks_dir) => serialize_and_write_to_fs(prefs, zdrinks_dir),
        None              => Err(failure::err_msg("Could not resolve home directory to write config file. Hint: We try to write to ~/.zdrinks/settings.yaml"))
    }
}

fn serialize_and_write_to_fs(prefs: ZDrinksArgs, path: PathBuf) -> Result<(), Error> {
    let conf = serde_yaml::to_string(&prefs)?;
    fs::create_dir_all(&path)?;
    let mut file = File::create(&path.join("settings.yaml"))?;
    file.write_all(conf.as_bytes())?;

    Ok(())
}