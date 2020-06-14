use colored::*;
use rand::prelude::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::{FASTMODE, LINETIME, TYPETIME};

/// Represents the available text colors we can output.
#[derive(Debug)]
pub enum Color {
    /// ![Blue](https://via.placeholder.com/16/0000ff/000000?text=+)
    Blue,
    /// ![Cyan](https://via.placeholder.com/16/00ffff/000000?text=+)
    Cyan,
    /// ![Green](https://via.placeholder.com/16/008000/000000?text=+)
    Green,
    /// ![Purple](https://via.placeholder.com/16/800080/000000?text=+)
    Purple,
    /// ![Red](https://via.placeholder.com/16/ff0000/000000?text=+)
    Red,
    /// <span style="border: 1px solid black;">![White](https://via.placeholder.com/16/ffffff/000000?text=+)</span>
    White,
    /// ![Yellow](https://via.placeholder.com/16/ffff00/000000?text=+)
    Yellow,
}

/// Prints out a line one character at a time with a specified color
/// 
/// The last parameter determines how long to sleep after the line is finished typing.
/// One use of this is to make options type out faster so the player doesnt have to wait as long to choose one.
/// 
/// If the line contains one or more quotes (some text within two quotation marks), the color will only be used inside those quotes.
/// This allows the author to associate certain colors with specific characters talking.
/// 
/// ```no_run
/// # use intfic::write_out::{type_text, Color};
/// type_text(
///     "\"You better show up on time tomorrow!\" He shouted as I left. \"No more excuses!\"",
///     Color::Blue,
///     false
/// );
/// ```
/// Output:
/// <pre class="example-wrap"
/// <span style="color:blue;">"You better show up on time tomorrow!" </span>
/// <span style="color:black;">He shouted as I left. </span>
/// <span style="color:blue;">"No more excuses!"</span>
/// </pre>
pub fn type_text(line: &str, color: Color, fast: bool) {
    if contains_quote(line) {
        type_quote(line, color, fast);
    } else {
        type_normal(line, color, fast);
    }
}

// Types out a given line all in the same given color.
//
// After each character, wait a slightly random amount of time based on TYPETIME.
// After each line, wait a shorter or longer amount of time based on the given "fast" parameter and LINETIME.
fn type_normal(line: &str, color: Color, fast: bool) {
    if line.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();
    for c in line.chars() {
        write_char(c, &color);
        naptime(TYPETIME.mul_f64(rng.gen::<f64>() + 0.25));
    }

    naptime(if fast { LINETIME / 2 } else { LINETIME });
    println!();
}

// Types out a given line, only using the given color to accentuate quotes in the line.
//
// After each character, wait a slightly random amount of time based on TYPETIME.
// After each line, wait a shorter or longer amount of time based on the given "fast" parameter and LINETIME.
fn type_quote(line: &str, color: Color, fast: bool) {
    if line.is_empty() {
        return;
    }

    let mut in_quote: bool = false;

    let mut rng = rand::thread_rng();
    for c in line.chars() {
        if c == '"' {
            write_char(c, &color);
            in_quote = !in_quote;
        } else if in_quote {
            write_char(c, &color);
        } else {
            write_char(c, &Color::White);
        }

        naptime(TYPETIME.mul_f64(rng.gen::<f64>() + 0.25));
    }

    naptime(if fast { LINETIME / 2 } else { LINETIME });
    println!();
}

// Returns true if the line contains two quotation marks.
fn contains_quote(line: &str) -> bool {
    let mut seen_quote: bool = false;

    for c in line.chars() {
        if c == '"' && seen_quote {
            return true;
        } else {
            seen_quote = true;
        }
    }

    false
}

// Type a single character with the given color, then flush stdout to display it.
fn write_char(c: char, color: &Color) {
    let mut s = String::default();
    s.push(c);

    match color {
        Color::White  => print!("{}", s),
        Color::Blue   => print!("{}", s.blue()),
        Color::Cyan   => print!("{}", s.cyan()),
        Color::Green  => print!("{}", s.green()),
        Color::Purple => print!("{}", s.purple()),
        Color::Red    => print!("{}", s.red()),
        Color::Yellow => print!("{}", s.yellow()),
    }

    io::stdout().flush().unwrap();
}

// Wait the given duration, unless FASTMODE is enabled.
fn naptime(time: Duration) {
    if !FASTMODE {
        thread::sleep(time);
    }
}
