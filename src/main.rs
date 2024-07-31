use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Ctrl-C to exit.\n\r",
           clear::All,
           cursor::Goto(1, 1))
        .unwrap();
    stdout.flush().unwrap();

    for k in stdin().keys() {
        match k.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('\n') => print!("\n\r"),
            Key::Char(c) => print!("{}", c),
            _ => print!("Unknown input")
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", clear::All).unwrap();
}
