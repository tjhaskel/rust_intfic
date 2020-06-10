//! game_state contains the struct representation of the GameState and all methods to update, save, load, and read from that state.

use dirs::data_local_dir;
use ron::de::from_reader;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{create_dir_all, remove_file, File};
use std::io::prelude::*;
use std::process;

use crate::parse_file::load_file;
use crate::story_block::start_block;
use crate::write_out::{type_text, Color};
use crate::DEBUG;

/// GameState holds information about the name of the game, story progress, boolean flags, and integer counters
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct GameState {
    pub name: String,
    pub progress: (String, String),
    pub flags: HashMap<String, bool>,
    pub counters: HashMap<String, i32>,
}

impl GameState {
    pub fn new(name_in: &str) -> GameState {
        let mut counters_init = HashMap::new();
        counters_init.insert(String::from("score"), 0);

        GameState {
            name: String::from(name_in),
            progress: (String::default(), String::default()),
            flags: HashMap::new(),
            counters: counters_init,
        }
    }

    pub fn print_debug(&self) {
        if DEBUG {
            println!("\nGame State:\n{}", self);
        }
    }

    pub fn set_flag(&mut self, name: String, val: bool) {
        self.flags.insert(name, val);
    }

    pub fn update_counter(&mut self, name: String, val: i32) {
        let new_val: i32 = self.counters[&name] + val;
        self.counters.insert(name, new_val);
    }

    pub fn add_score(&mut self, n: i32) {
        self.update_counter(String::from("score"), n);
    }

    pub fn get_flag(&self, name: String) -> bool {
        if let Some(val) = self.flags.get(&name) {
            *val
        } else {
            false
        }
    }

    pub fn get_counter(&self, name: String) -> i32 {
        if let Some(val) = self.counters.get(&name) {
            *val
        } else {
            0
        }
    }

    pub fn check_game_over(&self) {
        if self.get_flag(String::from("game_over")) {
            self.quit();
        }
    }

    pub fn set_progress(&mut self, story: &str, block: &str) {
        self.progress.0 = String::from(story);
        self.progress.1 = String::from(block);
    }

    pub fn save(&mut self) {
        let save_string =
            to_string_pretty(self, PrettyConfig::new()).expect("Serialization failed");

        if let Some(local_data_dir) = data_local_dir() {
            let save_dir = local_data_dir.join("rust_intfic\\");
            create_dir_all(&save_dir).expect("Couldn't create save directory");

            let save_path = save_dir.join(format!("{}.ron", self.name));
            let display = save_path.display();

            if save_path.exists() {
                remove_file(&save_path).expect("Couldn't remove old save file");
            }

            let mut save_file = match File::create(&save_path) {
                Err(e) => panic!("couldn't create {}: {}", display, e),
                Ok(file) => file,
            };

            match save_file.write_all(save_string.as_bytes()) {
                Err(e) => panic!("couldn't write to {}: {}", display, e),
                Ok(_) => {
                    type_text("Game Saved!", Color::White, false);
                    self.set_flag(String::from("saved"), true);
                }
            }
        } else {
            type_text("Error accessing local appdata", Color::Red, false);
        }
    }

    pub fn load(&mut self) {
        if let Some(local_data_dir) = data_local_dir() {
            let save_dir = local_data_dir.join(format!("rust_intfic\\{}.ron", self.name));
            let display = save_dir.display();

            if save_dir.exists() {
                let save_file = match File::open(&save_dir) {
                    Err(e) => panic!("couldn't load from {}: {}", display, e),
                    Ok(file) => file,
                };

                match from_reader(save_file) {
                    Ok(new_state) => {
                        *self = new_state;
                        type_text("Game Loaded!", Color::White, false);
                    }
                    Err(e) => panic!("Couldn't deserialize gamestate from {}: {}", display, e),
                };
            } else {
                type_text("No save data found", Color::Red, false);
            }
        } else {
            type_text("Error accessing local appdata", Color::Red, false);
        }
    }

    pub fn start(&mut self) {
        if let Some(loaded_blocks) = load_file(&(self.progress.0.clone()[..]), self) {
            start_block(self.progress.1.clone(), &loaded_blocks, self);
        } else {
            panic!("Couldn't start story: {}", self.progress.0.clone());
        }
    }

    pub fn quit(&self) {
        type_text("See you next time!", Color::White, false);
        self.print_debug();
        process::exit(0);
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new("Default")
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  Name: {}\n  Progress: [Story: {}, Block: {}]\n  Flags: {:?}\n  Counters: {:?}\n",
            self.name, self.progress.0, self.progress.1, self.flags, self.counters,
        )
    }
}
