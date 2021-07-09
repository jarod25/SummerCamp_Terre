use std::io;

#[derive(Debug, PartialEq)]
pub enum GameState {
    Playing,
    Win,
    Lose
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Element {
    Unknown,
    Number(u8),
    Cactus,
    Corne,
    Clear
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    CheckOp,
    CorneOp,
}

pub type DoubleArray = Vec<Vec<Element>>;

pub fn elem_to_string(elem: &Element) -> String {
    return match elem {
        Element::Unknown => String::from("X"),
        Element::Number(n) => format!("{}", n),
        Element::Cactus => String::from("·"),
        Element::Corne => String::from("C"),
        Element::Clear => String::from(" "),
    }
}

pub fn update_board(mut board: DoubleArray, board_solution: &DoubleArray, coords: &Vec<usize>, action: &Operation,  flags_remaining : &mut u16) -> DoubleArray {
    let x = coords[0];
    let y = coords[1];

    if *action == Operation::CheckOp {
        match board_solution[x][y] {
            Element::Cactus => board[x][y] = Element::Cactus,
            Element::Number(u8) => board[x][y] = Element::Number(u8),
            Element::Clear => board[x][y] = Element::Clear,
            _ => {}
        }
    } else if *action == Operation::CorneOp {
        if board[x][y] == Element::Corne {
            board[x][y] = Element::Unknown;
            *flags_remaining += 1;
        } else if *flags_remaining > 0 {
            *flags_remaining -= 1;
            match board_solution[x][y] {
                Element::Cactus => board[x][y] = Element::Corne,
                Element::Number(_) => board[x][y] = Element::Corne,
                Element::Clear => board[x][y] = Element::Corne,
                _ => {}
            }
        } else {
            println!("Plus de cornes disponibles.");
        }
    }

    return board;
}

pub fn print_board(board: &DoubleArray) {
    print!("    | ");
    for i in 0..board.len() {
        print!("{} | ", format!("{}", i));
    }

    print!("\n");

    let size = (board.len() * 4) + 5;
    for _ in 0..size {
        print!("-");
    }

    print!("\n");

    for (i, row) in board.iter().enumerate() {
        print!("| {} |", i);
        for cell in row {
            print!(" {} |", elem_to_string(&cell));
        }
        print!("\n");
    }
}

pub fn get_input(prompt: &str) -> String {
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

pub fn parse_input(input: String) -> Vec<usize> {
    let coordinates : Vec<usize> = input.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

    return coordinates;
}

pub fn parse_second_input(input: String) -> Option<Operation> {
    return match input.trim().to_lowercase().as_ref() {
        "v" => Some(Operation::CheckOp),
        "c" => Some(Operation::CorneOp),
        other => {
            println!("{} is not a valid option", other);
            return None;
        }
    }
}
