use super::board;

use std::io;
use std::io::Write;
use std::thread;
use std::time;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn run_loop(board: &mut board::Board) {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    loop {
        // Read input (if any)
        let input = stdin.next();
        let mut input_command: Option<String> = None;

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('q') => break,
                termion::event::Key::Left => {
                    input_command = Some("left".to_string());
                },
                termion::event::Key::Right => {
                    input_command = Some("right".to_string());
                },
                _ => {
                }
            }
        }

        let is_game_over = board.update(input_command);
        if is_game_over {
            println!("Game Over!\r");
            break
        }
        board.draw();
        stdout.lock().flush().unwrap();

        thread::sleep(time::Duration::from_millis(100));
    }
}
