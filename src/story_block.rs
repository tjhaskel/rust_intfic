use std::collections::HashMap;
use text_io::read;

use crate::game_state::GameState;
use crate::parse_file::load_file;
use crate::parse_input::{get_input, sanitize};
use crate::print_debug;
use crate::write_out::{type_text, Color};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Choice {
    pub text: String,
    pub typed: String,
    pub result: String,
}

impl Choice {
    fn present(&self, num: i32, _game: &mut GameState) {
        let numbered_option: &str = &format!("{}) {}", num, &self.text)[..];
        type_text(numbered_option, Color::White, true);
    }

    fn match_option(&self, input: &str, num: i32) -> bool {
        sanitize(self.text.clone()) == *input
            || self.result == *input
            || num.to_string() == *input
            || self.typed.contains(input)
    }
}

#[derive(Debug, Default, Clone)]
pub struct StoryBlock {
    pub name: String,
    pub text: Vec<String>,
    pub options: Vec<Choice>,
    pub flags: HashMap<String, bool>,
    pub counters: HashMap<String, i32>,
}

impl StoryBlock {
    pub fn new(name_in: String) -> StoryBlock {
        StoryBlock {
            name: name_in,
            text: Vec::new(),
            options: Vec::new(),
            flags: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    fn read(&self, game: &mut GameState, blocks: &[StoryBlock]) {
        game.progress.block = self.name.clone();
        self.read_text(game);
        self.apply_effects(game);
        self.present_options(game, blocks);
    }

    fn read_text(&self, game: &GameState) {
        for line in self.text.iter() {
            read_line(line, game);
        }

        println!();
    }

    fn apply_effects(&self, game: &mut GameState) {
        for (k, v) in self.flags.iter() {
            game.set_flag(String::from(k), *v);
        }

        for (k, v) in self.counters.iter() {
            game.update_counter(String::from(k), *v);
        }
    }

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

fn read_line(line: &str, game: &GameState) {
    if line.starts_with("?-") {
        let mut cond_split = line.split(" => ");

        if game.get_flag(read!("?- {}\n", cond_split.next().unwrap().bytes())) {
            read_line(&String::from(cond_split.next().unwrap()), game);
        } else if let Some(else_line) = cond_split.nth(1) {
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

fn filter_options(options: &[Choice], game: &GameState) -> Vec<Choice> {
    let mut filtered: Vec<Choice> = Vec::new();

    for choice in options.iter() {
        if choice.text.starts_with("?-") {
            let mut cond_split = choice.text.split(" => ");

            if game.get_flag(read!("?- {}\n", cond_split.next().unwrap().bytes())) {
                filtered.push(Choice {
                    text: String::from(cond_split.next().unwrap()),
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

pub fn start_blocks(blocks: &[StoryBlock], game: &mut GameState) {
    blocks[0].read(game, blocks);
}

pub fn start_block(name: String, blocks: &[StoryBlock], game: &mut GameState) {
    if let Some (block) = find_block(&name[..], blocks) {
        block.read(game, blocks);
    }
}

pub fn find_block<'a>(name: &str, blocks: &'a [StoryBlock]) -> Option<&'a StoryBlock> {
    for block in blocks {
        if block.name == *name {
            return Some(block);
        }
    }
    None
}

fn play_next(name: &str, game: &mut GameState, blocks: &[StoryBlock]) {
    if name.ends_with(".txt") {
        if let Some(next_blocks) = load_file(name, game) {
            start_blocks(&next_blocks, game);
        }
    } else if let Some(next_block) = find_block(name, blocks) {
        game.set_flag(String::from("saved"), false);
        next_block.read(game, blocks);
    } else {
        print_debug(format!("Can't find StoryBlock: {}", name));
    }
}
