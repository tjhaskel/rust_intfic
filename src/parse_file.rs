//! # intfic Story File Markup Specification.
//! Story files house a collection of Story Blocks that make up the narrative of your game.
//! You can link together multiple story files to write and organize your story in a cohesive manner.
//! 
//! Here is an example of a simple story block:
//! <pre>
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
//! </pre>
//! Story Blocks have three main parts:
//! <pre>
//! TITLE
//! 
//! TEXT & EFFECTS
//! 
//! QUESTION & OPTIONS
//! </pre>
//! 
//! ## TITLE
//! A title indicates the start of a new StoryBlock. It always starts with the characters ":- ", followed by the name of the new block.
//! <pre>
//! :- lose_computer
//! </pre>
//! 
//! ## TEXT & EFFECTS
//! The middle section of a Story Block contains the text the player will see, and any effects that will be applied to the GameState.
//! <pre>
//! He abruptly yanks the power cord out of the computer and power strip, it shuts off with a sharp buzz.
//! ?- saved_work => Thank god you had just saved, you can't imagine having lost all that work. => You can't believe what just happened. Why didn't you save? So much work just gone.
//! =- computer_access = false
//! -b "You aren't supposed to do that!" You protest. "It can permanently damage the machine!"
//! #- score >= 50 => -y "I'm sorry son, but I think this will help." He says calmly. => -y "You won't learn any other way!" He yells back.
//! Your younger brother and sister, having heard the commotion, appear at the doorway between the computer room and kitchen.
//! -g "Dad, can we still use the computer?" Your brother asks, innocently.
//! -y "Yes that's fine, just ask me for the cord when you need it, and make sure to give it back to me after"
//! They seem satisfied and grin at him before heading back to the tv. You feel a pang of embarrassment.
//! +- shame + 1
//! </pre>
//! The folowing character combinations, when used at the start of a line or conditional line, have special effects:
//! * `"-b "`: Prints the line in <span style="color:blue; text-shadow: 1px 0.5px #555">blue</span>.
//! * `"-c "`: Prints the line in <span style="color:cyan; text-shadow: 1px 0.5px #555">cyan</span>.
//! * `"-g "`: Prints the line in <span style="color:green; text-shadow: 1px 0.5px #555">green</span>.
//! * `"-p "`: Prints the line in <span style="color:purple; text-shadow: 1px 0.5px #555">purple</span>.
//! * `"-r "`: Prints the line in <span style="color:red; text-shadow: 1px 0.5px #555">red</span>.
//! * `"-y "`: Prints the line in <span style="color:yellow; text-shadow: 1px 0.5px #555">yellow</span>.
//! * `"=- "`: Sets the given flag to the given value in our GameState
//!   > **Example:** `"=- computer_access = false"` sets **computer_access** to **false**.
//! * `"+- "`: Adds the given value to the given counter in our GameState
//!   > **Example:** `"+- shame + 1"` adds **1** to whatever value **shame** has, or sets it to **1** if it is not set.
//! * `"?- "`: Prints a "then" or optional "else" line based on the given flag's value in our GameState.
//!   > **Example:** `"?- saved_work => saved_work then line => saved_work else line"`\
//!   > &nbsp;&nbsp; This will print `"saved_work then line"` if **saved_work** is **true**,\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; and will print `"saved_work else line"` otherwise.\
//!   > &nbsp;&nbsp; The "else" line is optional, if you would rather no line be read should the condition fail.\
//!   > &nbsp;&nbsp; Note that conditional lines are parsed *recursively*, so you may use colors or nested conditionals in them.
//! * `"#- "`: Prints a "then" or optional "else" line based on the given predicate's value in our GameState's counter environment.
//!   > **Example:** `"#- score >= 50 => score check then line => score check else line"`\
//!   > &nbsp;&nbsp; This will print `"score check then line"` if **score >= 50** is **true**,\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; and will print `"score check else line"` otherwise.\
//!   > &nbsp;&nbsp; The "else" line is optional, if you would rather no line be read should the condition fail.\
//!   > &nbsp;&nbsp; Note that conditional lines are parsed *recursively*, so you may use colors or nested conditionals in them.
//! 
//! ## QUESTION & OPTIONS
//! The final section of a StoryBlock is the question and options presented.
//! <pre>
//!   What do you do?
//! *- #- strength >= 25 => Punch your dad. -> punch him, violence -> fight_dad.txt
//! *- Leave the house. -> take walk, run -> wander_neighborhood.txt
//! *- Go to bed. ->  -> sleep
//! *- ?- have_time_machine => Go five minutes in the past to fix this -> go back in time, time travel -> time_fix
//! </pre>
//! The "question" is usually indicated by a line of text just before the options with two extra spaces at its start.
//! These spaces tell the parser to print the question in <span style="color:cyan; text-shadow: 1px 0.5px #555">cyan</span>,
//! but they and the question itself are completely optional, and in fact are just part of the block's text.
//! (Meaning that you could ask different questions based on a conditional query).
//! 
//! Options have the following structure:
//! <pre>
//! *- Text the player will see -> strings to match input -> block or file to read if matched
//! </pre>
//!   > **Example:** `"*- Leave the house. -> take walk, run -> wander_neighborhood.txt"`\
//!   > &nbsp;&nbsp; This option will be presented as `"#) Leave the house."` to the player,\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; Where "#" is the number it is presented as, either **1** or **2** in the above example.\
//!   > &nbsp;&nbsp; The number of an option may be entered by the player to choose that option.\
//!   > &nbsp;&nbsp; When the player types their answer and hits enter, `"take walk, run"` will be searched\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; to see if it contains that input as a substring.\
//!   > &nbsp;&nbsp; Additionally, a match may be made if the input matches the option *text* or *result* string exactly.\
//!   > &nbsp;&nbsp; If a match is found, then the Story File `"wander_neighborhood.txt"` will be loaded\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; and the story will pick up at the first block of that file.
//! 
//!   > **Example:** `"*- Go to bed. ->  -> sleep"`\
//!   > &nbsp;&nbsp; This option will be presented as `"#) Go to bed."` to the player,\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; Where "#" is the number it is presented as, either **2** or **3** in the above example.\
//!   > &nbsp;&nbsp; The number of an option may be entered by the player to choose that option.\
//!   > &nbsp;&nbsp; This example does not use any keywords to match input against. So the player can only choose it\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; by typing the number, "Go to bed", or "sleep" exactly.\
//!   > &nbsp;&nbsp; Though I don't recommend not including extra keywords to match against,\
//!   > &nbsp;&nbsp; &nbsp;&nbsp; You may do so as long as you keep two spaces between the "->" delimiters.\
//!   > &nbsp;&nbsp; If a match is found, then the Story Block `"sleep"` will be played.
//! 
//! The folowing character combinations, when used at the start of an option, have special effects:
//! * `"?- "`: Presents the option only if the given flag's value is true in our GameState.
//!   > **Example:** `"*- ?- have_time_machine => Time Travel -> go back -> time_fix"`\
//!   > &nbsp;&nbsp; This option will only be available to choose from if **have_time_machine** is **true**.\
//!   > &nbsp;&nbsp; Note that their is no "else" option available to show.
//! * `"#- "`: Prints the option only if the given predicate's value is true in our GameState's counter environment.
//!   > **Example:** `"*- #- strength >= 25 => Punch your dad. -> fight -> fight_dad.txt"`\
//!   > &nbsp;&nbsp; This option will only be available to choose from if **strength >= 25** is **true**.\
//!   > &nbsp;&nbsp; Note that their is no "else" option available to show.

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
    // Start of a new block, so the end of the current one!
    if text.starts_with(":-") {
        if *seen_block {
            blocks.push((*current_block).clone());
        } else {
            *seen_block = true;
        }

        *current_block = StoryBlock::new(read!(":- {}\n", text.bytes()));

    // Set a flag in the GameState
    } else if text.starts_with("=-") {
        let mut var_split: Vec<&str> = text.split(" = ").collect();
        let var_name: String = read!("=- {}\n", var_split[0].bytes());
        let var_value: bool = (var_split[1]).parse().unwrap();

        current_block.flags.insert(var_name, var_value);

    // Update a counter in the GameState
    } else if text.starts_with("+-") {
        let mut var_split: Vec<&str> = text.split(" + ").collect();
        let var_name: String = read!("+- {}\n", var_split[0].bytes());
        let var_value: i32 = (var_split[1]).parse().unwrap();

        current_block.counters.insert(var_name, var_value);

    // New choice
    } else if text.starts_with("*-") {
        let mut choice_split: Vec<&str> = text.split(" -> ").collect();
        let new_choice = Choice {
            text: read!("*- {}\n", choice_split[0].bytes()),
            typed: String::from(choice_split[1]),
            result: String::from(choice_split[2]),
        };

        current_block.options.push(new_choice);

    // No choice, just proceed to indicated block/file
    } else if text.starts_with("->") {
        let new_choice = Choice {
            text: String::default(),
            typed: String::default(),
            result: read!("-> {}\n", text.bytes()),
        };

        current_block.options.push(new_choice);

    // Just normal text
    } else {
        current_block.text.push(text);
    }
}
