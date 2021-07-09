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
    unimplemented!();
}
pub fn lose(board: &DoubleArray, coords: &Vec<usize>) -> bool {
    unimplemented!();
}

pub fn get_cactus_number(board_solution: &DoubleArray) -> u16 {
    unimplemented!();
}

pub fn win(board: &DoubleArray, board_solution: &DoubleArray, cactus_number: u16) -> bool {
    unimplemented!();
}

pub fn main() {
    unimplemented!();
}
