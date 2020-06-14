use std::collections::HashMap;

use crate::game_state::GameState;
use crate::parse_file::load_file;
use crate::parse_input::{get_input, query, sanitize};
use crate::print_debug;
use crate::write_out::{type_text, Color};

/// StoryBlocks are atomic chunks of interactive narrative.
/// 
/// They have a name, a list of text that will be presented to the player,
/// a list of options that will be presented to the player, and a series of effects that will be applied to the GameState (flag or counter alterations).
/// 
/// Text and options may be filtered based on conditionals that check flags and counters in the current GameState.
/// However, such conditions must pass at the **start** of the block. e.g. you cannot set a flag and get the result you just set in the same block.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StoryBlock {
    /// The name of the storyblock, may be referenced as the "result" of options.
    pub name: String,
    /// The text that will be typed out line by line, your story!
    pub text: Vec<String>,
    /// The options available to choose from by the player.
    pub options: Vec<Choice>,
    /// The flags that will be applied to our GameState by this block.
    pub flags: HashMap<String, bool>,
    /// The counters that will be applied to our GameState by this block.
    pub counters: HashMap<String, i32>,
}

/// A choice has some text that the player will see, a list of words to match input against, and a result.
/// 
/// The result can be the name of a story block in the same file, or the filename of a story file.
/// If pointing to a new story file, the game will start at the first block in that file.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Choice {
    /// The string that will be typed out and presented to the player for this option.
    pub text: String,
    /// If the user types a substring of this string, the option will be selected.
    pub typed: String,
    /// Corresponds to the name of a story block or story file
    pub result: String,
}

impl Choice {
    // Print out the text of a Choice with a number before it to produce an ordered list.
    fn present(&self, num: i32, _game: &mut GameState) {
        let numbered_option: &str = &format!("{}) {}", num, &self.text)[..];
        type_text(numbered_option, Color::White, true);
    }

    // Searches the text, "typed" string, and number corresponding with an option for the given input string
    //
    // This determines if the player was selecting that option.
    fn match_option(&self, input: &str, num: i32) -> bool {
        sanitize(self.text.clone()) == *input
            || self.result == *input
            || num.to_string() == *input
            || self.typed.contains(input)
            || self.typed.starts_with('@') && query(&(self.typed[..]), input)
    }
}

impl StoryBlock {
    /// Returns an empty story block with the given name.
    /// 
    /// ```
    /// # use std::collections::HashMap;
    /// # use intfic::story_block::StoryBlock;
    /// let mut block: StoryBlock = StoryBlock::new(String::from("Test GameState"));
    /// 
    /// assert_eq!(
    ///     block, 
    ///     StoryBlock {
    ///         name: String::from("Test GameState"),
    ///         text: Vec::new(),
    ///         options: Vec::new(),
    ///         flags: HashMap::new(),
    ///         counters: HashMap::new(),
    ///     }
    /// );
    /// ```
    pub fn new(name_in: String) -> StoryBlock {
        StoryBlock {
            name: name_in,
            text: Vec::new(),
            options: Vec::new(),
            flags: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    // Plays out the contents and effects of a block, then presents the options to the player.
    //
    // Also updates the GameState progress with this block's name.
    fn read(&self, game: &mut GameState, blocks: &[StoryBlock]) {
        game.progress.1 = self.name.clone();
        self.read_text(game);
        self.apply_effects(game);
        self.present_options(game, blocks);
    }

    // Reads the text of this block line by line.
    fn read_text(&self, game: &GameState) {
        for line in self.text.iter() {
            read_line(line, game);
        }

        println!();
    }

    // Applies any flags or counters associated with this block to the GameState flag and counter environments.
    fn apply_effects(&self, game: &mut GameState) {
        for (k, v) in self.flags.iter() {
            game.set_flag(k, *v);
        }

        for (k, v) in self.counters.iter() {
            game.update_counter(k, *v);
        }
    }

    // Presents a filtered, ordered list of options for the player to choose from, and facitates the player making a choice
    //
    // Filters out any options that have conditions which are not satisfied in our GameState.
    // Once the options are presented, waits for user input and then checks that input against the given options.
    // If any match, start that option's "result" block or file. If non match, ask for another input.
    fn present_options(&self, game: &mut GameState, blocks: &[StoryBlock]) {
        let options: &Vec<Choice> = &filter_options(&self.options, game);
        let num_options = options.len();

        if num_options == 0 {
            return;
        } else if num_options == 1 {
            play_next(&options[0].result, game, blocks);
            return;
        }

        let mut num: i32 = 1;
        for choice in options {
            choice.present(num, game);
            num += 1;
        }
        println!();

        let mut valid_choice: bool = false;
        while !valid_choice {
            if let Some(input) = get_input(game) {
                if input.is_empty() {
                    continue;
                }

                let mut num = 1;
                for choice in options {
                    if choice.match_option(&input, num) {
                        play_next(&choice.result, game, blocks);
                        valid_choice = true;
                        break;
                    }
                    num += 1;
                }

                if !valid_choice {
                    println!("I didn't understand that.");
                }
            } else {
                break;
            }
        }
    }
}

// Given a string with a proper integer comparason conditional, parse and return the result of that conditional.
fn check_counter(cond: &str, game: &GameState) -> bool {
    let mut cond_split = cond.split(' ');
    let count_name: &str = cond_split.nth(1).unwrap();
    let count_amount = game.get_counter(count_name);

    match cond_split.next().unwrap() {
        "<" => count_amount < cond_split.next().unwrap().parse::<i32>().unwrap(),
        "<=" => count_amount <= cond_split.next().unwrap().parse::<i32>().unwrap(),
        "==" => count_amount == cond_split.next().unwrap().parse::<i32>().unwrap(),
        ">=" => count_amount >= cond_split.next().unwrap().parse::<i32>().unwrap(),
        ">" => count_amount > cond_split.next().unwrap().parse::<i32>().unwrap(),
        _ => false,
    }
}

// Print out a line according to conditionals or colors prefixing it.
//
// Checks if a line has a conditional, and on displays the "Then" portion of the line if it passes.
// Lines of text may also have an optional "Else" portion if using a conditional.
// If a line apsses, ti is then checked for color indicators, and is sent to write_out with the appropriate Color enum.
fn read_line(line: &str, game: &GameState) {
    if line.starts_with("?-") {
        let mut cond_split = line.split(" => ");

        if game.get_flag(&(cond_split.next().unwrap())[3..]) {
            read_line(&String::from(cond_split.next().unwrap()), game);
        } else if let Some(else_line) = cond_split.nth(1) {
            read_line(&String::from(else_line), game);
        }
    } else if line.starts_with("#-") {
        let mut line_split = line.split(" => ");

        if check_counter(line_split.next().unwrap(), game) {
            read_line(&String::from(line_split.next().unwrap()), game);
        } else if let Some(else_line) = line_split.nth(1) {
            read_line(&String::from(else_line), game);
        }
    } else if line.starts_with("-y ") {
        type_text(&line[3..], Color::Yellow, false);
    } else if line.starts_with("-b ") {
        type_text(&line[3..], Color::Blue, false);
    } else if line.starts_with("-g ") {
        type_text(&line[3..], Color::Green, false);
    } else if line.starts_with("-r ") {
        type_text(&line[3..], Color::Red, false);
    } else if line.starts_with("  ") {
        println!();
        type_text(&line, Color::Cyan, false);
    } else {
        type_text(&line, Color::White, false);
    }
}

// Filter a list of options to only include those who either have no condition or have a condition that returns true in our GameState
fn filter_options(options: &[Choice], game: &GameState) -> Vec<Choice> {
    let mut filtered: Vec<Choice> = Vec::new();

    for choice in options.iter() {
        if choice.text.starts_with("?-") {
            let mut cond_split = choice.text.split(" => ");

            if game.get_flag(&(cond_split.next().unwrap())[3..]) {
                filtered.push(Choice {
                    text: String::from(cond_split.next().unwrap()),
                    typed: choice.typed.clone(),
                    result: choice.result.clone(),
                })
            }
        } else if choice.text.starts_with("#-") {
            let mut choice_split = choice.text.split(" => ");

            if check_counter(choice_split.next().unwrap(), game) {
                filtered.push(Choice {
                    text: String::from(choice_split.next().unwrap()),
                    typed: choice.typed.clone(),
                    result: choice.result.clone(),
                })
            }
        } else {
            filtered.push(choice.clone());
        }
    }

    filtered
}

/// Starts reading the first block in the given Vec\<StoryBlock>.
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
pub fn start_blocks(blocks: &[StoryBlock], game: &mut GameState) {
    blocks[0].read(game, blocks);
}

/// Starts reading the block with the given name in the given Vec\<StoryBlock>.
/// 
/// If no block matching the name is found, Print a debug message stating so.
/// 
/// ```no_run
/// # use intfic::game_state::GameState;
/// # use intfic::parse_file::load_file;
/// # use intfic::story_block::start_block;
/// let mut game: GameState = GameState::new("Test GameState");
/// 
/// if let Some(loaded_blocks) = load_file("example_1.txt", &mut game) {
///     start_block(String::from("flag_example"), &loaded_blocks, &mut game);
/// }
/// ```
pub fn start_block(name: String, blocks: &[StoryBlock], game: &mut GameState) {
    if let Some(block) = find_block(&name[..], blocks) {
        block.read(game, blocks);
    } else {
        print_debug(format!("No block found with the name {}", name));
    }
}

// Searches the given list fo blocks for one that matches the given name, returning Some(StoryBlock) if successful.
fn find_block<'a>(name: &str, blocks: &'a [StoryBlock]) -> Option<&'a StoryBlock> {
    for block in blocks {
        if block.name == *name {
            return Some(block);
        }
    }
    None
}

// Plays the next StoryBlock or Story file based on the given name.
fn play_next(name: &str, game: &mut GameState, blocks: &[StoryBlock]) {
    if name.ends_with(".txt") {
        if let Some(next_blocks) = load_file(name, game) {
            start_blocks(&next_blocks, game);
        }
    } else if let Some(next_block) = find_block(name, blocks) {
        game.set_flag("saved", false);
        next_block.read(game, blocks);
    } else {
        print_debug(format!("Can't find StoryBlock: {}", name));
    }
}
