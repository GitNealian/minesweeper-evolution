use colored::*;
use crossterm::{cursor, event::{read, Event, KeyCode}, execute, terminal::{self, enable_raw_mode}};
use rand::prelude::*;
use std::io::{stdout,Write };

fn main() {
    let mut board = start_game(10, 5);
    let mut row = 0;
    let mut col = 0;
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    loop {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide,
        ).unwrap();
        print_game(&board, 10);
        let event = read().unwrap();
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Up => {
                    row = if move_to(&mut board, 10, row - 1, col) {
                        row - 1
                    } else {
                        row
                    };
                    print!("up");
                }

                KeyCode::Down => {
                    row = if move_to(&mut board, 10, row + 1, col) {
                        row + 1
                    } else {
                        row
                    }
                }
                KeyCode::Left => {
                    col = if move_to(&mut board, 10, row, col - 1) {
                        col - 1
                    } else {
                        col
                    }
                }
                KeyCode::Right => {
                    col = if move_to(&mut board, 10, row, col+1) {
                        col + 1
                    } else {
                        col
                    }
                }
                KeyCode::Enter => {
                    explore(&mut board, 10, row, col);
                }
                KeyCode::Char(' ') => {
                    mark(&mut board, 10, row, col);
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
}

fn mark(board: &mut Vec<u8>, width: i32, row: i32, col: i32) {

}

fn explore(board: &mut Vec<u8>, width: i32, row: i32, col: i32) {}

fn move_to(board: &mut Vec<u8>, width: i32, row: i32, col: i32) -> bool {
    if 0 > row || row >= width || 0 > col || col >= width {
        return false;
    }
    board[(row * width + col) as usize] &= 0xF0;
    return true;
}

fn print_game(board: &Vec<u8>, width: i32) {
    let mut i: i32 = 0;
    for b in board {
        print!(
            "[{}]",
            match (b & 0xf0) >> 4 {
                0xf => String::from("X").yellow(),
                0xe => String::from("P").green(),
                0xd => String::from("?").yellow(),
                a @ 1..=8 => format!("{}", a).white(),
                _ => String::from(" ").white(),
            }
        );
        i = i + 1;
        if i % width == 0 {
            print!("\n\r");
        }
    }
}

fn start_game(width: i32, mines: i32) -> Vec<u8> {
    assert!(width * width > mines);
    let mines_pos = (1..width * width).choose_multiple(&mut rand::thread_rng(), mines as usize);
    return (0..width * width)
        .map(|x| if mines_pos.contains(&x) { 0xf1 } else { 0xf0 })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_game() {
        assert_eq!(
            start_game(10, 5)
                .into_iter()
                .filter(|&x| x & 0x0f == 1)
                .count(),
            5
        );
    }

    #[test]
    fn test_print_game() {
        print_game(&start_game(10, 5), 10);
    }
}
