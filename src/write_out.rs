use colored::*;
use rand::prelude::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::{FASTMODE, LINETIME, TYPETIME};

#[derive(Debug)]
pub enum Color {
    Blue,
    Cyan,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

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

pub fn type_text(line: &str, color: Color, fast: bool) {
    if contains_quote(line) {
        type_quote(line, color, fast);
    } else {
        type_normal(line, color, fast);
    }
}

fn type_normal(line: &str, color: Color, fast: bool) {
    if line.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();
    for c in line.chars() {
        write_char(c, &color);
        io::stdout().flush().unwrap();
        naptime(TYPETIME.mul_f64(rng.gen::<f64>() + 0.25));
    }

    naptime(if fast { LINETIME / 2 } else { LINETIME });
    println!();
}

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

        io::stdout().flush().unwrap();
        naptime(TYPETIME.mul_f64(rng.gen::<f64>() + 0.25));
    }

    naptime(if fast { LINETIME / 2 } else { LINETIME });
    println!();
}

fn write_char(c: char, color: &Color) {
    let mut s = String::default();
    s.push(c);

    match color {
        Color::Blue => {
            print!("{}", s.blue());
        }
        Color::Cyan => {
            print!("{}", s.cyan());
        }
        Color::Green => {
            print!("{}", s.green());
        }
        Color::Purple => {
            print!("{}", s.purple());
        }
        Color::Red => {
            print!("{}", s.red());
        }
        Color::Yellow => {
            print!("{}", s.yellow());
        }
        _ => {
            print!("{}", s);
        }
    }
}

fn naptime(time: Duration) {
    if !FASTMODE {
        thread::sleep(time);
    }
}
