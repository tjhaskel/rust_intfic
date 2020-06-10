use intfic::game_state::GameState;

fn main() {
    let mut game = GameState::new("Interactive Fiction Title");

    game.set_progress("example_1.txt", "start");
    game.start();
}
