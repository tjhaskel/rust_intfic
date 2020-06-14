//! # Story File Markup Specification.
//! Story files house a collection of Story Blocks that make up the narrative of your game.
//! You can link together multiple story files to write and organize your story in a cohesive manner.
//! 
//! Here is an example of a simple story block:
//! ```text
//! :- start
//! 
//! It's nearly pitch black out tonight. There's a bit of light from the city up north, but no stars are peeking out through the clouds.
//! You wonder if he'll come after you; if you're going to be in more trouble for storming out like that.
//! What if it made him more angry? The thought makes you walk a bit faster towards the intersection ahead.
//! As you turn you see the flash of a car's headlights from the direction you came.
//! 
//!   What do you do?
//! *- Keep walking -> walk -> walk_car
//! *- Hide from the car -> hide -> hide_car
//! *- Run from the car -> run -> run_car
//! ```
//! Story Blocks have three main parts:
//! ```text
//! TITLE
//! 
//! TEXT & EFFECTS
//! 
//! QUESTION & OPTIONS
//! ```
//! # TITLE
//! A title indicates the start of a new StoryBlock. It always starts with the characters ":- ", followed by the name of the new block.
//! ```text
//! :- walk_car
//! ```
//! # TEXT & EFFECTS
//! The middle section of a Story Block contains the text the player will see, and any effects that will be applied to the GameState.
//! ```
//! He abruptly yanks the power cord out of the computer and power strip, it shuts off with a sharp buzz.
//! =- computer_access = false
//! -b "You aren't supposed to do that!" You protest. "It can permanently damage the machine!"
//! Your younger brother and sister, having heard the commotion, appear at the doorway between the computer room and kitchen.
//! -g "Dad, can we still use the computer?" Your brother asks, innocently.
//! -y "Yes that's fine, just ask me for the cord when you need it, and make sure to give it back to me after"
//! They seem satisfied and grin at him before heading back to the tv. You feel a pang of embarrassment.
//! +- embarrasment + 1
//! ```
//! 

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use text_io::read;

use crate::game_state::GameState;
use crate::story_block::{Choice, StoryBlock};

/// Takes the name of a story file and parses it, returning Some(Vec\<StoryBlock>) if successful
/// 
/// ```no_run
/// # use intfic::game_state::GameState;
/// # use intfic::parse_file::load_file;
/// # use intfic::story_block::start_blocks;
/// let mut game: GameState = GameState::new("Test GameState");
/// 
/// if let Some(loaded_blocks) = load_file("example_1.txt", &mut game) {
///     start_blocks(&loaded_blocks, &mut game);
/// }
/// ```
pub fn load_file(filename: &str, game: &mut GameState) -> Option<Vec<StoryBlock>> {
    if let Ok(lines) = get_file(filename) {
        game.progress.0 = String::from(filename);

        let mut blocks: Vec<StoryBlock> = Vec::new();
        let mut current_block: StoryBlock = StoryBlock::default();
        let mut seen_block = false;

        for line in lines {
            if let Ok(text) = line {
                parse_line(text, &mut blocks, &mut current_block, &mut seen_block)
            }
        }

        blocks.push(current_block);
        Some(blocks)
    } else {
        println!("Error getting file: {}", filename);
        None
    }
}

// Gathers the text content of a file and saves it as a list of lines if successful.
//
// Story files should be placed in /resources to be found by this function.
fn get_file(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let resources: &Path = Path::new("resources");
    let file = File::open(resources.join(filename))?;
    Ok(io::BufReader::new(file).lines())
}

// Parses each line of the story file and constructs blocks that can be stored in out Vec<StoryBlock>
//
// Full Story File markup specification can be found above.
fn parse_line(
    text: String,
    blocks: &mut Vec<StoryBlock>,
    current_block: &mut StoryBlock,
    seen_block: &mut bool,
) {
    if text.starts_with(":-") {
        if *seen_block {
            blocks.push((*current_block).clone());
        } else {
            *seen_block = true;
        }

        *current_block = StoryBlock::new(read!(":- {}\n", text.bytes()));
    } else if text.starts_with("*-") {
        let mut choice_split: Vec<&str> = text.split(" -> ").collect();
        let new_choice = Choice {
            text: read!("*- {}\n", choice_split[0].bytes()),
            typed: String::from(choice_split[1]),
            result: String::from(choice_split[2]),
        };

        current_block.options.push(new_choice);
    } else if text.starts_with("->") {
        let new_choice = Choice {
            text: String::default(),
            typed: String::default(),
            result: read!("-> {}\n", text.bytes()),
        };

        current_block.options.push(new_choice);
    } else if text.starts_with("=-") {
        let mut var_split: Vec<&str> = text.split(" = ").collect();
        let var_name: String = read!("=- {}\n", var_split[0].bytes());
        let var_value: bool = (var_split[1]).parse().unwrap();

        current_block.flags.insert(var_name, var_value);
    } else if text.starts_with("+-") {
        let mut var_split: Vec<&str> = text.split(" + ").collect();
        let var_name: String = read!("+- {}\n", var_split[0].bytes());
        let var_value: i32 = (var_split[1]).parse().unwrap();

        current_block.counters.insert(var_name, var_value);
    } else {
        current_block.text.push(text);
    }
}
