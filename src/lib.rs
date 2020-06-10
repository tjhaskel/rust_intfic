use std::time;

pub mod game_state;
pub mod parse_file;
pub mod parse_input;
pub mod story_block;
pub mod write_out;

#[cfg(test)]
mod tests;

pub const DEBUG: bool = true;
pub const FASTMODE: bool = true;
pub const TYPETIME: time::Duration = time::Duration::from_millis(24);
pub const LINETIME: time::Duration = time::Duration::from_millis(1200);

pub fn print_debug(to_print: String) {
    if DEBUG {
        println!("{}", to_print);
    }
}
