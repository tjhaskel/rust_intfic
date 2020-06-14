use intfic::game_state::GameState;

fn main() {
    // Create an empty gamestate with the title "Interactive Fiction Title"
    let mut game = GameState::new("Interactive Fiction Title");

    // Set the game progress to indicate the story file and block we want to start with, then starting the game.
    game.set_progress("example_1.txt", "start");
    game.start();

    // Print out the GameState when the game is over. This may not run if the player exits early!
    game.print_debug();
}
