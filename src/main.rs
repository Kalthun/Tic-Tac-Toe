use std::f32::{INFINITY, NEG_INFINITY};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Clone, Copy, PartialEq)]
enum Player
{
    None = 0,
    X = 10, // first
    O = -10 // second
}

impl Player
{
    pub fn opposite(&self) -> Player
    {
        if *self == Player::X { return Player::O }
        if *self == Player::O { return Player::X }
        Player::None
    }
}

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

#[derive(Clone, Copy, PartialEq)]
struct State
{
    to_move:Player,
    moves_played:u8,
    last_move:u8,
    board:[Player;9],
    line_scores:[i8;8],
}

impl State
{
    pub fn is_terminal(&self) -> bool
    {
        if self.moves_played == 9 { return true; }

        for score in self.line_scores
        {
            if score == 30 || score == -30 { return true; }
        }

        false
    }

    pub fn evaluate_board(&self) -> i8
    {
        for score in self.line_scores
        {
            if score ==  30 { return Player::X as i8 - self.moves_played as i8}
            if score == -30 { return Player::O as i8 + self.moves_played as i8}
        }
        Player::None as i8
    }

    pub fn is_open(&self, cell:u8) -> bool
    {
        let cell_index = cell - 1;
        self.board[cell_index as usize] == Player::None
    }

    pub fn make_move(&mut self, cell:u8) // ! return a bool
    {
        // ! Already Full
        if !self.is_open(cell) { return };

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

impl std::fmt::Display for State
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // let temp = "()".to_owned() + self.line_scores[0].to_string().as_str() + self.grid[0][0].to_string().as_str();
        let r1 = "┌───┬───┬───┬───┬───┐\n";
        let r2 = format!("│{:3}│{:3}│{:3}│{:3}│{:3}│\n", self.line_scores[0], self.line_scores[1], self.line_scores[2], self.line_scores[3], self.line_scores[7]);
        let r3 = "├───┼───┴───┴───┼───┤\n";
        let r4 = format!("│TIC│ {} │ {} │ {} │{:3}│\n", self.board[0], self.board[1], self.board[2], self.line_scores[4]);
        let r5 = "├───┤───┼───┼───├───┤\n";
        let r6 = format!("│TAC│ {} │ {} │ {} │{:3}│\n", self.board[3], self.board[4], self.board[5], self.line_scores[5]);
        let r7 = "├───┤───┼───┼───├───┤\n";
        let r8 = format!("│TOE│ {} │ {} │ {} │{:3}│\n", self.board[6], self.board[7], self.board[8], self.line_scores[6]);
        let r9 = "└───┴───────────┴───┘\n";
        let i1 = format!("To Move: {}\n", self.to_move);
        let i2 = format!("Moves Played: {}\n", self.moves_played);
        let i3 = format!("Last Move: {}\n", self.last_move);
        let i4 = format!("Evaluation: {}\n", self.evaluate_board());

        // TODO: change how the end of game is displayed

        let mut state_as_string = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}", r1, r2, r3, r4, r5, r6, r7, r8, r9, i1, i2, i3, i4);

        if self.is_terminal() { state_as_string = format!("{}{}", state_as_string, "GAME OVER\n") };

        write!(f, "{}", state_as_string)
    }
}

fn build_state(id: String) -> State
{
    let mut temp_board = State
    {
        to_move:Player::X,
        moves_played:0,
        last_move:0,
        board:[Player::None;9],
        line_scores:[0;8]
    };

    for cell in id.chars()
    {
        temp_board.make_move(cell.to_digit(10).unwrap() as u8);
        if temp_board.is_terminal() { break; }
    }

    println!("{}", temp_board);

    temp_board
}

fn ab_minimax(board:State, mut alpha:i8, mut beta:i8) -> (i8, u8)
{
    if board.is_terminal()
    {
        return (board.evaluate_board(), board.last_move);
    }

    let mut best_value = if board.to_move == Player::X { NEG_INFINITY as i8 } else { INFINITY as i8 };
    let mut best_move:u8 = 0;

    for cell in 1 .. 10
    {
        if board.is_open(cell)
        {
            let mut next_board = board;
            next_board.make_move(cell);
            let value = ab_minimax(next_board, alpha, beta);

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
        }
        if beta <= alpha { break; }
    }

    (best_value, best_move)
}

enum Error
{
    None,
    Duplicate,
    Input
}

// TODO: test_move

fn test_id(id:String) -> Error
{
    let mut played = [false; 9];
    for cell in id.chars()
    {
        if !cell.is_numeric() || cell == '0'
        {
            return Error::Input;
        }

        let cell_index = cell.to_digit(10).unwrap() - 1;
        if played[cell_index as usize]
        {
            return Error::Duplicate;
        }
        played[cell_index as usize] = true;
    }
    Error::None
}

fn main() -> std::io::Result<()>
{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

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

    println!("=Alpha Beta Minimax Testing=\n");
    let board1 = build_state("7958".to_string());
    let case1 = ab_minimax(board1, NEG_INFINITY as i8, INFINITY as i8);
    println!("{}", case1.0);
    println!("{}", case1.1);

    let mut file = File::create("output.txt").unwrap();
    let _ = file.write(board1.to_string().as_bytes());

    Ok(())

}
