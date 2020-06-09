use std::collections::HashMap;

use crate::game_state::*;
use crate::parse_file::*;
use crate::story_block::*;

#[test]
fn test_gamestate() {
    let mut test_state: GameState = GameState::new("Test_Game");

    assert_eq!(test_state.progress, 
        Progress {
            story: String::default(),
            block: String:: default(),
        });

    assert_eq!(test_state.get_flag(String::from("not_set")), false);
    test_state.set_flag(String::from("test"), true);
    assert_eq!(test_state.get_flag(String::from("test")), true);
    test_state.set_flag(String::from("test"), false);
    assert_eq!(test_state.get_flag(String::from("test")), false);

    assert_eq!(test_state.get_counter(String::from("not_set")), 0);
    assert_eq!(test_state.get_counter(String::from("score")), 0);
    test_state.add_score(50);
    assert_eq!(test_state.get_counter(String::from("score")), 50);
}

#[test]
fn test_storyblocks() {
    let mut test_state: GameState = GameState::new("Test_Story");

    if let Some(test_blocks) = load_file("test.txt", &mut test_state) {
        assert_eq!(test_blocks[1].name, "test_1");

        assert_eq!(test_blocks[1].text, vec!(
            String::from(""),
            String::from("You picked test 1!"),
            String::from("?- impossible_condition => this should never be seen"),
            String::from("?- test_condition => this should always be seen"),
            String::from("")
        ));

        assert_eq!(test_blocks[1].options, vec!(
            Choice {
                text: String::default(),
                typed: String::default(),
                result: String::from("test_5"),
            }
        ));

        let mut test_flags: HashMap<String, bool> = HashMap::new();
        test_flags.insert(String::from("test_condition"), false);
        assert_eq!(test_blocks[1].flags, test_flags);

        let test_counters: HashMap<String, i32> = HashMap::new();
        assert_eq!(test_blocks[1].counters, test_counters);
    } else {
        panic!("Couldn't load test.txt into StoryBlocks");
    }
}
