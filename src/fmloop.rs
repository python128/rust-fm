use crate::state::FmState;

use std::io::{stdin, Write};

use termion::input::TermRead;

/// Starts the program loop
pub fn start_loop<W: Write>(state: &mut FmState<W>) {
    state.reload();

    loop {
        state.show();
	    let key = stdin().keys().next().unwrap().unwrap();
        match key {
            termion::event::Key::Ctrl('c') | termion::event::Key::Ctrl('q') => break,
            _ => state.handle_key(key),
        }
    }
}
