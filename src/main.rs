use colored::*;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{self, enable_raw_mode},
};
use rand::prelude::*;
use std::{
    io::{stdout, Write},
    usize,
};

fn main() {
    let mut board = start_game(10, 5);
    let mut row = 0;
    let mut col = 0;
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let mut end = false;
    loop {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            cursor::Hide,
        )
        .unwrap();
        print_game(&board, 10, row, col);
        if end {
            break;
        }
        let event = read().unwrap();
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Up => {
                    row = if move_to(10, row - 1, col) {
                        row - 1
                    } else {
                        row
                    };
                    print!("up");
                }

                KeyCode::Down => {
                    row = if move_to(10, row + 1, col) {
                        row + 1
                    } else {
                        row
                    }
                }
                KeyCode::Left => {
                    col = if move_to(10, row, col - 1) {
                        col - 1
                    } else {
                        col
                    }
                }
                KeyCode::Right => {
                    col = if move_to(10, row, col + 1) {
                        col + 1
                    } else {
                        col
                    }
                }
                KeyCode::Char(' ') => {
                    end = explore(&mut board, 10, row, col);
                }
                KeyCode::Char('m') => {
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
    if board[(row * width + col) as usize] & 0x80 != 0 {
        return;
    } else {
        // get flg
        let flg = board[(row * width + col) as usize] >> 5 & 0x03;
        // clear old flag
        board[(row * width + col) as usize] &= 0x9f;
        // set new flag
        board[(row * width + col) as usize] |= (flg + 1) % 3 << 5;
    }
}

fn count(board: &Vec<u8>, width: i32, row: i32, col: i32) -> i32 {
    let mut m = vec![false; 8];
    m.push(has_mine(board, width, row - 1, col - 1));
    m.push(has_mine(board, width, row - 1, col));
    m.push(has_mine(board, width, row - 1, col + 1));
    m.push(has_mine(board, width, row, col - 1));
    m.push(has_mine(board, width, row, col + 1));
    m.push(has_mine(board, width, row + 1, col - 1));
    m.push(has_mine(board, width, row + 1, col));
    m.push(has_mine(board, width, row + 1, col + 1));
    return m.iter().filter(|x| **x).count() as i32;
}

fn has_mine(board: &Vec<u8>, width: i32, row: i32, col: i32) -> bool {
    if 0 > row || row >= width || 0 > col || col >= width {
        return false;
    }
    return board[(row * width + col) as usize] & 0x01 == 1;
}

fn explore(board: &mut Vec<u8>, width: i32, row: i32, col: i32) -> bool {
    if 0 > row || row >= width || 0 > col || col >= width || board[(row * width + col) as usize] & 0x80 != 0{
        return false;
    }
    let c = count(board, width, row, col) as u8;
    board[(row * width + col) as usize] |= c << 1;
    board[(row * width + col) as usize] |= 0x80;
    if board[(row * width + col) as usize] & 0x01 == 1 {
        return true;
    }
    if c == 0 {
        explore(board, width, row - 1, col - 1);
        explore(board, width, row - 1, col);
        explore(board, width, row - 1, col + 1);
        explore(board, width, row, col - 1);
        explore(board, width, row, col + 1);
        explore(board, width, row + 1, col - 1);
        explore(board, width, row + 1, col);
        explore(board, width, row + 1, col + 1);
    }
    return false;
}

fn move_to(width: i32, row: i32, col: i32) -> bool {
    if 0 > row || row >= width || 0 > col || col >= width {
        return false;
    }
    return true;
}

fn print_game(board: &Vec<u8>, width: i32, row: i32, col: i32) {
    let mut i: i32 = 0;
    for b in board {
        let mut txt = format!(
            "[{}]",
            match b >> 5 {
                0 => String::from("*").white(),
                1 => String::from("P").red(),
                2 => String::from("?").yellow(),
                4 => {
                    if b & 0x01 == 1 {
                        String::from("X").red()
                    } else if (b >> 1) & 0x0f == 0 {
                        String::from(" ").white()
                    } else {
                        format!("{}", (b >> 1) & 0x0f).white()
                    }
                }
                _ => panic!(),
            }
        );
        if i == row * width + col {
            txt = txt.on_blue().to_string();
        }
        print!("{}", txt);
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
        .map(|x| if mines_pos.contains(&x) { 0x01 } else { 0x00 })
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
        print_game(&start_game(10, 5), 10, 0, 0);
    }
}
