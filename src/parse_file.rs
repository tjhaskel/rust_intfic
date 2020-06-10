use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use text_io::read;

use crate::game_state::GameState;
use crate::story_block::{Choice, StoryBlock};

fn get_file(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let resources: &Path = Path::new("resources");
    let file = File::open(resources.join(filename))?;
    Ok(io::BufReader::new(file).lines())
}

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
