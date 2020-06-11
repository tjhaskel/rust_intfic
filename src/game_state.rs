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

/// GameState holds information about the name of the game, story progress, boolean flags, and integer counters.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct GameState {
    /// The name of your game, also used as the name for its save file.
    pub name: String,

    /// A tuple representing the filename of the story file and the name of the story block curently presented.
    pub progress: (String, String),

    /// A HashMap environment of named booleans that can be modified and checked against at runtime.
    pub flags: HashMap<String, bool>,

    /// A HashMap environment of named integers that can be modified and checked against at runtime.
    pub counters: HashMap<String, i32>,
}

impl GameState {
    /// Creates a new Gamestate with the given name and "score" == 0 in counters.
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

    /// Prints the current state of the game if DEBUG is enabled.
    pub fn print_debug(&self) {
        if DEBUG {
            println!("\nGame State:\n{}", self);
        }
    }

    /// Sets or updates a flag in the GameState flags HashMap.
    pub fn set_flag(&mut self, name: String, val: bool) {
        self.flags.insert(name, val);
    }

    /// Sets or updates a counter in the GameState flags HashMap.
    pub fn update_counter(&mut self, name: String, val: i32) {
        let new_val: i32 = self.counters[&name] + val;
        self.counters.insert(name, new_val);
    }

    /// Helper to add the given i32 to the score counter.
    pub fn add_score(&mut self, n: i32) {
        self.update_counter(String::from("score"), n);
    }

    /// If the given flag is in our GameState flags Hashmap, return it. Otherwise, return false.
    pub fn get_flag(&self, name: String) -> bool {
        if let Some(val) = self.flags.get(&name) {
            *val
        } else {
            false
        }
    }

    /// If the given counter is in our GameState counters Hashmap, return it. Otherwise, return 0.
    pub fn get_counter(&self, name: String) -> i32 {
        if let Some(val) = self.counters.get(&name) {
            *val
        } else {
            0
        }
    }

    /// Helper to check if a "game_over" flag is true in our GameState flags Hashmap, and quit the game if so.
    pub fn check_game_over(&self) {
        if self.get_flag(String::from("game_over")) {
            self.quit();
        }
    }

    /// Helper to set the progress in our GameState to the given strings
    pub fn set_progress(&mut self, story: &str, block: &str) {
        self.progress.0 = String::from(story);
        self.progress.1 = String::from(block);
    }

    /// Saves the game to "\<local data dir>\rust_intfic\\\<game name>.ron".
    /// 
    /// * On Windows, \<local data dir> corresponds to "C:\Users\<username>\AppData\Local".
    /// * On macOS, \<local data dir> corresponds to "/Users/\<username>/Library/Application Support".
    /// * On Linux, \<local data dir> corresponds to "/home/\<username>/.local/share".
    /// 
    /// Saving overwrites any previous save file currently.
    /// 
    /// Saving sets a flag in our GameState to indicate it is safe to quit.
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

    /// Loads the game from "\<local data dir>\rust_intfic\\\<game name>.ron".
    /// 
    /// * On Windows, \<local data dir> corresponds to "C:\Users\<username>\AppData\Local".
    /// * On macOS, \<local data dir> corresponds to "/Users/\<username>/Library/Application Support".
    /// * On Linux, \<local data dir> corresponds to "/home/\<username>/.local/share".
    /// 
    /// If the load is successful, the current GameState will be overwritten with the loaded one.
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

    /// Searhes for the story file and block indicated in "progress", then starts reading the story there if successful.
    pub fn start(&mut self) {
        if let Some(loaded_blocks) = load_file(&(self.progress.0.clone()[..]), self) {
            start_block(self.progress.1.clone(), &loaded_blocks, self);
        } else {
            panic!("Couldn't start story: {}", self.progress.0.clone());
        }
    }

    /// Prints out the current GameState and then stops execution.
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
