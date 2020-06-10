use text_io::read;

use crate::game_state::GameState;
use crate::print_debug;
use crate::write_out::{type_text, Color};

#[derive(Debug)]
pub enum Answer {
    Yes,
    No,
    Unsure,
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
    Return,
}

const AFFIRMATIVES: &[&str] = &[
    "10-4",
    "affirmative",
    "alright",
    "aye",
    "fuck yeah",
    "fuck yes",
    "hell yeah",
    "hell yes",
    "ok",
    "okay",
    "please",
    "positive",
    "sure",
    "y",
    "yay",
    "ye",
    "yeah",
    "yeah ok",
    "yeah sure",
    "yep",
    "yes",
    "yes please",
    "yup",
];

const NEGATIVES: &[&str] = &[
    "fuck nah",
    "fuck no",
    "hell nah",
    "hell no",
    "n",
    "nah",
    "nay",
    "negative",
    "never",
    "no",
    "nope",
    "no please",
    "not ok",
    "not okay",
    "no way",
];

const UNSURATIVES: &[&str] = &[
    "dunno",
    "huh",
    "idk",
    "i dont know",
    "i dunno",
    "i guess",
    "maybe",
    "no clue",
    "no idea",
    "not sure",
    "que",
    "shrug",
    "unsure",
    "what",
];

const NORTHS: &[&str] = &[
    "forward",
    "go forward",
    "go north",
    "n",
    "north",
    "northbound",
    "northward",
];

const EASTS: &[&str] = &[
    "e",
    "east",
    "eastbound",
    "eastward",
    "go east",
    "go right",
    "right",
];

const SOUTHS: &[&str] = &[
    "backward",
    "go backward",
    "go south",
    "s",
    "south",
    "southbound",
    "southward",
];

const WESTS: &[&str] = &[
    "go left",
    "go west",
    "left",
    "w",
    "west",
    "westbound",
    "westward",
];

const UPS: &[&str] = &[
    "ascend",
    "climb",
    "climb up",
    "fly",
    "fly up",
    "go up",
    "rise",
    "u",
    "up",
];

const DOWNS: &[&str] = &[
    "climb down",
    "d",
    "descend",
    "down",
    "fall",
    "glide",
    "go down",
];

const RETURNS: &[&str] = &[
    "b",
    "back",
    "fall back",
    "go back",
    "r",
    "retreat",
    "return",
    "run",
    "run away",
];

const EXITS: &[&str] = &[
    "exit",
    "exit game",
    "quit",
    "quit game",
];

const SAVES: &[&str] = &[
    "save",
    "save game",
];

const LOADS: &[&str] = &[
    "load",
    "load game",
];

pub fn query(dict: &str, name: &str) -> bool {
    match dict {
        "@AFFIRMATIVES" => AFFIRMATIVES.contains(&name),
        "@NEGATIVES" => NEGATIVES.contains(&name),
        "@UNSURATIVES" => UNSURATIVES.contains(&name),
        "@NORTHS" => NORTHS.contains(&name),
        "@EASTS" => EASTS.contains(&name),
        "@SOUTHS" => SOUTHS.contains(&name),
        "@WESTS" => WESTS.contains(&name),
        "@UPS" => UPS.contains(&name),
        "@DOWNS" => DOWNS.contains(&name),
        "@RETURNS" => RETURNS.contains(&name),
        "@SAVES" => SAVES.contains(&name),
        "@LOADS" => LOADS.contains(&name),
        "@EXITS" => EXITS.contains(&name),
        _ => false,
    }
}

pub fn sanitize(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .trim()
        .to_lowercase()
}

pub fn get_input(game: &mut GameState) -> Option<String> {
    let input: String = sanitize(read!("{}\n"));

    if EXITS.contains(&&input[..]) {
        if game.get_flag(String::from("saved")) {
            game.quit();
            None
        } else {
            match ask_question("Do you want to save first?", game) {
                Some(Answer::Yes) => {
                    game.save();
                    game.quit();
                    None
                }
                Some(Answer::No) => {
                    game.quit();
                    None
                }
                Some(Answer::Unsure) => {
                    type_text("I'll just save for you...", Color::White, false);
                    game.save();
                    game.quit();
                    None
                }
                _ => None,
            }
        }
    } else if SAVES.contains(&&input[..]) {
        game.save();
        game.start();
        None
    } else if LOADS.contains(&&input[..]) {
        game.load();
        game.start();
        None
    } else {
        Some(input)
    }
}

fn print_parse_result(input: &str, parsed: &str) {
    print_debug(format!("Input: {}, Parsed: {}", input, parsed));
}

pub fn ask_question(question: &str, game: &mut GameState) -> Option<Answer> {
    loop {
        type_text(question, Color::Cyan, true);
        if let Some(input) = get_input(game) {
            if input.is_empty() {
                continue;
            }

            if let Some(answer) = parse_answer(&input[..]) {
                return Some(answer);
            }

            type_text("I didn't understand that.", Color::White, false);
        } else {
            return None;
        }
    }
}

fn parse_answer(input: &str) -> Option<Answer> {
    if AFFIRMATIVES.contains(&input) {
        print_parse_result(input, "Answer->Yes");
        Some(Answer::Yes)
    } else if NEGATIVES.contains(&input) {
        print_parse_result(input, "Answer->No");
        Some(Answer::No)
    } else if UNSURATIVES.contains(&input) {
        print_parse_result(input, "Answer->Unsure");
        Some(Answer::Unsure)
    } else {
        print_parse_result(input, "Answer->None");
        None
    }
}

pub fn ask_direction(question: &str, game: &mut GameState) -> Option<Direction> {
    loop {
        type_text(question, Color::Cyan, true);
        if let Some(input) = get_input(game) {
            if input.is_empty() {
                continue;
            }

            if let Some(direction) = parse_direction(&input[..]) {
                return Some(direction);
            }

            type_text("I didn't understand that.", Color::White, false);
        } else {
            return None;
        }
    }
}

fn parse_direction(input: &str) -> Option<Direction> {
    if NORTHS.contains(&input) {
        print_parse_result(input, "Direction->North");
        Some(Direction::North)
    } else if EASTS.contains(&input) {
        print_parse_result(input, "Direction->East");
        Some(Direction::East)
    } else if SOUTHS.contains(&input) {
        print_parse_result(input, "Direction->South");
        Some(Direction::South)
    } else if WESTS.contains(&input) {
        print_parse_result(input, "Direction->West");
        Some(Direction::West)
    } else if UPS.contains(&input) {
        print_parse_result(input, "Direction->Up");
        Some(Direction::Up)
    } else if DOWNS.contains(&input) {
        print_parse_result(input, "Direction->Down");
        Some(Direction::Down)
    } else if RETURNS.contains(&input) {
        print_parse_result(input, "Direction->Return");
        Some(Direction::Return)
    } else {
        print_parse_result(input, "Direction->None");
        None
    }
}
