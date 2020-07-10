use text_io::read;

use crate::game_state::GameState;
use crate::print_debug;
use crate::write_out::{type_text, Color};

/// Yes, No, or Unsure. Corresponds with a dictionary of responses that indicate one of these three answers.
#[derive(Debug, PartialEq)]
pub enum Answer {
    /// Yes, y, yeah, sure, etc.
    Yes,
    /// No, n, nah, nope, etc.
    No,
    /// Not sure, idk, maybe, etc.
    Unsure,
}

/// Cardinal directions, as well as Up, Down, and Return. Corresponds with a dictionary of responses.
#[derive(Debug, PartialEq)]
pub enum Direction {
    /// North, n, forward, etc.
    North,
    /// East, e, right, etc.
    East,
    /// South, s, backward, etc.
    South,
    /// West, w, left, etc.
    West,
    /// Up, u, ascend, etc.
    Up,
    /// Down, d, descend, etc.
    Down,
    /// Return, r, go back, etc.
    Return,
}

// Dictionary for Answer::Yes
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

// Dictionary for Answer::No
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

// Dictionary for Answer::Unsure
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

// Dictionary for Direction::North
const NORTHS: &[&str] = &[
    "forward",
    "go forward",
    "go north",
    "n",
    "north",
    "northbound",
    "northward",
];

// Dictionary for Direction::East
const EASTS: &[&str] = &[
    "e",
    "east",
    "eastbound",
    "eastward",
    "go east",
    "go right",
    "right",
];

// Dictionary for Direction::South
const SOUTHS: &[&str] = &[
    "backward",
    "go backward",
    "go south",
    "s",
    "south",
    "southbound",
    "southward",
];

// Dictionary for Direction::West
const WESTS: &[&str] = &[
    "go left",
    "go west",
    "left",
    "w",
    "west",
    "westbound",
    "westward",
];

// Dictionary for Direction::Up
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

// Dictionary for Direction::Down
const DOWNS: &[&str] = &[
    "climb down",
    "d",
    "descend",
    "down",
    "fall",
    "glide",
    "go down",
];

// Dictionary for Direction::Return
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

// Dictionary for game.quit()
const EXITS: &[&str] = &[
    "exit",
    "exit game",
    "quit",
    "quit game",
];

// Dictionary for game.save()
const SAVES: &[&str] = &[
    "save",
    "save game",
];

// Dictionary for game.load()
const LOADS: &[&str] = &[
    "load",
    "load game",
];

/// Returns true if the given dictionary contains the given input string.
/// 
/// ```
/// # use intfic::parse_input::query;
/// assert_eq!(query("@AFFIRMATIVES", "sure"), true);
/// ```
/// Dictionaries are specified with an @ sign in front of them, and can be referenced this way in the "typed" field of a Choice.
/// ```text
///   Isn't that neat?
/// *- Yeah I guess. -> @AFFIRMATIVES -> neat
/// *- Not really. -> @NEGATIVES -> not_neat
/// ```
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

/// Returns a lowercase string containing only alphanumeric characters and spaces.
/// 
/// ```
/// # use intfic::parse_input::sanitize;
/// 
/// assert_eq!(sanitize(String::from("OH mOst DefiniTEly!?!")), String::from("oh most definitely"));
/// ```
pub fn sanitize(input: String) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .trim()
        .to_lowercase()
}

/// Gets input from the user and checks if it matches a keyword (returns None) or else returns Some(String).
/// 
/// Keywords are the following:
/// * exit - asks to save if you haven't recently, then quits the game. See [game_state::GameState::quit()](../game_state/struct.GameState.html#method.quit)
/// * save - saves the game. See [game_state::GameState::save()](../game_state/struct.GameState.html#method.save)
/// * load - loads the game. See [game_state::GameState::load()](../game_state/struct.GameState.html#method.load)
/// 
/// ```no_run
/// # use intfic::parse_input::get_input;
/// # use intfic::game_state::GameState;
/// let mut game: GameState = GameState::new("Test GameState");
/// 
/// assert_eq!(get_input(&mut game), Some(String::from("open door"))); // If the user typed "open door"
/// assert_eq!(get_input(&mut game), None); // If the user typed "load" or "save"
/// ```
pub fn get_input(game: &mut GameState) -> Option<String> {
    let input: String = sanitize(read!("{}\n"));

    if EXITS.contains(&&input[..]) {
        if game.get_flag("saved") {
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

/// Asks a given yes-no question and returns Some(Answer) if the user doesn't type a keyword.
/// 
/// If the user types something that does not correspond to any of the Answer dictionaries, 
/// the question will repeat until a proper response or keyword is given.
/// 
/// ```no_run
/// # use intfic::parse_input::{ask_question, Answer};
/// # use intfic::game_state::GameState;
/// let mut game: GameState = GameState::new("Test GameState");
/// 
/// assert_eq!(ask_question("Continue?", &mut game), Some(Answer::Yes)); // If the user typed "y"
/// assert_eq!(ask_question("Continue?", &mut game), None); // If the user typed "load" or "save"
/// ```
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

// Searches Answer dictionaries for given input
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

/// Asks for a direction and returns Some(Answer) if the user doesn't type a keyword.
/// 
/// If the user types something that does not correspond to any of the Direction dictionaries, 
/// the question will repeat until a proper response or keyword is given.
/// 
/// ```no_run
/// # use intfic::parse_input::{ask_direction, Direction};
/// # use intfic::game_state::GameState;
/// let mut game: GameState = GameState::new("Test GameState");
/// 
/// assert_eq!(ask_direction("Which Way?", &mut game), Some(Direction::North)); // If the user typed "Northward!"
/// assert_eq!(ask_direction("Which Way??", &mut game), None); // If the user typed "load" or "save"
/// ```
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

// Searches Direction dictionaries for given input
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

// Helper to print the input and parsed result if DEBUG is enabled
fn print_parse_result(input: &str, parsed: &str) {
    print_debug(format!("Input: {}, Parsed: {}", input, parsed));
}
