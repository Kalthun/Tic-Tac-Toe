/*
    COMP-4475
    Assignment 4
    James McDonagh
    1106211
*/

use std::f32::{INFINITY, NEG_INFINITY};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// * Player Enum
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Player
{
    None = 0,
    X = 10, // goes first
    O = -10 // goes second
}

// * Quick Swap
impl Player
{
    pub fn opposite(&self) -> Player
    {
        if *self == Player::X { return Player::O }
        if *self == Player::O { return Player::X }
        Player::None
    }
}

// * Nice Display
impl std::fmt::Display for Player
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Player::None => write!(f, " "),
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O")
        }
    }
}

// * State struct
#[derive(Clone, Hash, Eq, PartialEq)]
struct State
{
    id:String,
    board:[Player;9],
    line_scores:[i8;8],
    to_move:Player,
    moves_played:u8,
    last_move:u8,
}

impl State
{
    // * Checks if game is over
    pub fn is_terminal(&self) -> bool
    {
        if self.moves_played == 9 { return true; }

        for score in self.line_scores
        {
            if score == 30 || score == -30 { return true; }
        }

        false
    }

    // * Checks to see if a player has won
    pub fn evaluate_scores(&self) -> i8
    {
        for score in self.line_scores
        {
            // * We also factor in the number of moves player
            if score ==  30 { return Player::X as i8 - self.moves_played as i8}
            if score == -30 { return Player::O as i8 + self.moves_played as i8}
        }
        Player::None as i8
    }

    // * get all available moves
    pub fn get_moves(&self) -> Vec<u8>
    {
        let mut open_cells:Vec<u8> = Vec::new();

        for cell in 1 .. 10
        {
            if self.board[(cell - 1) as usize] == Player::None { open_cells.push(cell); }
        }

        open_cells
    }

    // * perform a move
    pub fn make_move(&mut self, cell:u8)
    {
        self.id.push_str(cell.to_string().as_str());

        self.board[(cell - 1) as usize] = self.to_move;

        let value = self.to_move as i8;
        match cell
        {
            1 => { self.line_scores[1] += value; self.line_scores[4] += value; self.line_scores[0] += value; }
            2 => { self.line_scores[2] += value; self.line_scores[4] += value; }
            3 => { self.line_scores[3] += value; self.line_scores[4] += value; self.line_scores[7] += value; }
            4 => { self.line_scores[1] += value; self.line_scores[5] += value; }
            5 => { self.line_scores[2] += value; self.line_scores[5] += value; self.line_scores[0] += value; self.line_scores[7] += value }
            6 => { self.line_scores[3] += value; self.line_scores[5] += value; }
            7 => { self.line_scores[1] += value; self.line_scores[6] += value; self.line_scores[7] += value; }
            8 => { self.line_scores[2] += value; self.line_scores[6] += value; }
            9 => { self.line_scores[3] += value; self.line_scores[6] += value; self.line_scores[0] += value; }
            _ => {}
        }

        self.to_move = self.to_move.opposite();
        self.moves_played += 1;
        self.last_move = cell;
    }
}

// * Nice Display
impl std::fmt::Display for State
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let r1 = "┌───┬───┬───┬───┬───┐\n";
        let r2 = format!("│{:3}│{:3}│{:3}│{:3}│{:3}│\n", self.line_scores[0], self.line_scores[1], self.line_scores[2], self.line_scores[3], self.line_scores[7]);
        let r3 = "├───┼───┴───┴───┼───┤\n";
        let r4 = format!("│TIC│ {} │ {} │ {} │{:3}│\n", self.board[0], self.board[1], self.board[2], self.line_scores[4]);
        let r5 = "├───┤───┼───┼───├───┤\n";
        let r6 = format!("│TAC│ {} │ {} │ {} │{:3}│\n", self.board[3], self.board[4], self.board[5], self.line_scores[5]);
        let r7 = "├───┤───┼───┼───├───┤\n";
        let r8 = format!("│TOE│ {} │ {} │ {} │{:3}│\n", self.board[6], self.board[7], self.board[8], self.line_scores[6]);
        let r9 = "└───┴───────────┴───┘\n";
        let id = format!("id: {}\n", self.id);
        let i1 = format!("To Move: {}\n", self.to_move);
        let i2 = format!("Moves Played: {}\n", self.id.len());
        let i3 = format!("Last Move: {}\n", self.id.chars().last().unwrap_or('0'));
        let i4 = format!("Evaluation: {}\n", self.evaluate_scores());
        let i5 = format!("AB Score: {}\n\n", ab_minimax(self, NEG_INFINITY as i8, INFINITY as i8).0);

        let mut state_as_string = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", r1, r2, r3, r4, r5, r6, r7, r8, r9, id, i1, i2, i3, i4, i5);

        if self.is_terminal()
        {
            state_as_string = format!("{}{}", state_as_string, "GAME OVER\n");
            if self.evaluate_scores() > 0 { state_as_string = format!("{}{}", state_as_string, "X Won!\n") };
            if self.evaluate_scores() < 0 { state_as_string = format!("{}{}", state_as_string, "O Won!\n") };
            if self.evaluate_scores() == 0 { state_as_string = format!("{}{}", state_as_string, "It's a Draw!\n") };
        };

        write!(f, "{}", state_as_string)
    }
}

// * Build a state with an id
fn build_state(id: String) -> State
{
    let mut temp_board = State
    {
        id:String::with_capacity(9),
        board:[Player::None;9],
        line_scores:[0;8],
        to_move:Player::X,
        moves_played:0, // Don't need anymore
        last_move:0 // Don't need anymore
    };

    for cell in id.chars()
    {
        temp_board.make_move(cell.to_digit(10).unwrap() as u8);
        if temp_board.is_terminal() { break; }
    }

    println!("{}", temp_board);

    temp_board
}

// * Alpha Beta Minimax
fn ab_minimax(board:&State, mut alpha:i8, mut beta:i8) -> (i8, u8)
{
    if board.is_terminal()
    {
        return (board.evaluate_scores(), board.last_move);
    }

    let mut best_value = if board.to_move == Player::X { NEG_INFINITY as i8 } else { INFINITY as i8 };
    let mut best_move:u8 = 0;

    for cell in board.get_moves()
    {
        let mut next_board = board.clone();
        next_board.make_move(cell);
        let value = ab_minimax(&next_board, alpha, beta);

        if board.to_move == Player::X && value.0 > best_value
        {
            best_value = value.0;
            best_move = cell;
            alpha = std::cmp::max(alpha, best_value);
        }

        if board.to_move == Player::O && value.0 < best_value
        {
            best_value = value.0;
            best_move = cell;
            beta = std::cmp::min(beta, best_value);
        }

        if beta <= alpha { break; }
    }
    (best_value, best_move)
}

#[derive(PartialEq)]
enum Error
{
    None,
    Duplicate,
    Input
}

fn test_cell(cell:char) -> Error
{
    if !cell.is_numeric() || cell == '0' { return Error::Input }
    Error::None
}

fn test_id(id:String) -> Error
{
    let mut played = [false; 9];
    for cell_as_char in id.chars()
    {
        let cell_error = test_cell(cell_as_char);
        if cell_error != Error::None { return cell_error }

        let cell = cell_as_char.to_digit(10).unwrap() as u8;

        if played[(cell - 1) as usize] { return Error::Duplicate; }
        played[(cell - 1) as usize] = true;
    }
    Error::None
}

fn main() -> std::io::Result<()>
{
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut output = File::create("output.txt").unwrap();

    for line in reader.lines()
    {
        let current_line = line.unwrap();
        println!("{}", current_line);

        let error = test_id(current_line.clone());

        match error
        {
            Error::None => {
                let _ = build_state(current_line);
            },
            Error::Input => {
                println!("ERROR: Invalid Input");
            },
            Error::Duplicate => {
                println!("ERROR: Duplicate Input");
            }
        }
    }

    let mut full = build_state("".to_string());
    let mut input = String::new();

    while !full.is_terminal()
    {
        print!("Make a move: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let choice = input.chars().next().unwrap().to_digit(10).unwrap() as u8;
        if choice != 0
        {
            full.make_move(choice);
        }
        else
        {
            full.make_move(ab_minimax(&full, NEG_INFINITY as i8, INFINITY as i8).1);
        }

        println!("{}", full);
        input.clear();
        let _ = output.write(full.to_string().as_bytes());
    }

    Ok(())

}
