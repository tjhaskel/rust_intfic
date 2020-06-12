#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! intfic is a framework that allows you to write a branching story with minimal code.
//! 
//! It uses story files with a custom markup syntax that allows for the following:
//! 
//! * Write text or specific quotes of text with different colors
//! * Display options that trigger different Story Blocks or Story Files
//! * Set flags or add to counters in the GameState
//! * Check flags or counters in the GameState and conditionally display text or options
//! 
//! Additionally, I've included some basic functions for asking yes-no questions and traveling in the cardinal directions, should you prefer to take a more "text adventure" approach with code.
//! 
//! ## Getting Started
//! 
//! 1. Run the example with "cargo run"
//! 2. Examine the example story files to get familiar with the story markup syntax
//! 3. Write you own story, and update main.rs to start it!
//! 
//! ## License
//! 
//! This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/tjhaskel/rust_intfic/blob/master/LICENSE.md) file for details 

use std::time;

/// Stores, saves, and loads an environment that can be changed and referenced by your story.
pub mod game_state;

/// Parses story files and constructs a list of StoryBlock's.
pub mod parse_file;

/// Sanitizes and parses input, checking for system keywords.
pub mod parse_input;

/// Represents an atomic chunk of story with text, effects, and options.
pub mod story_block;

/// Writes text with a typewriter effect and a variety of possible colors.
pub mod write_out;

#[cfg(test)]
mod tests;

/// If enabled, various debug info will be printed during gameplay
pub const DEBUG: bool = true;

/// If enabled, the typewriter effect of write_out is made instantaneous.
pub const FASTMODE: bool = true;

/// The base amount of time before the next line of a block is started, if FASTMODE is disabled.
pub const LINETIME: time::Duration = time::Duration::from_millis(1200);

/// The base amount of time before the next charater of a line is typed, if FASTMODE is disabled.
pub const TYPETIME: time::Duration = time::Duration::from_millis(24);

/// Prints a string if DEBUG is enabled.
pub fn print_debug(to_print: String) {
    if DEBUG {
        println!("{}", to_print);
    }
}
