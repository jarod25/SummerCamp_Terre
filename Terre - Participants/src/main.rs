#![allow(unused)]
#![allow(dead_code)]
use rand::Rng;
use help::GameState;
use help::Element;
use help::DoubleArray;
use help::update_board;
use help::print_board;
use help::get_input;
use help::parse_input;
use help::parse_second_input;

mod help;

fn generate_board(size: usize) -> DoubleArray {
    let mut board = vec![vec![Element::Clear; size]; size];
    for _ in 0..size {
        let x = rand::thread_rng().gen_range(0..size);
        let y = rand::thread_rng().gen_range(0..size);
        board[y][x] =  Element::Cactus;
    }
    return board;
}
pub fn lose(board: &DoubleArray, coords: &Vec<usize>) -> bool {

}

pub fn get_cactus_number(board_solution: &DoubleArray) -> u16 {

}

pub fn win(board: &DoubleArray, board_solution: &DoubleArray, cactus_number: u16) -> bool {

}

pub fn main() {

}
