use std::f32::{INFINITY, NEG_INFINITY};
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Error
{
    None,
    Duplicate,
    Input
}

#[derive(Clone, Copy, PartialEq)]
enum Player
{
    None = 0,
    X = 1, // first
    O = -1 // second
}

impl Player
{
    pub fn opposite(&self) -> Player
    {
        if *self == Player::None { return Player::None; } // ? needed
        return if *self == Player::X { Player::O } else { Player::X }
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

/*
    0   1   2   6
---===========----
    1 | 2 | 3   3
---===========----
    4 | 5 | 6   4
---===========----
    7 | 8 | 9   5
---===========----
                7
*/

/*
    0   1   2   6
---===========----
    0 | 1 | 2   3
---===========----
    3 | 4 | 5   4
---===========----
    6 | 7 | 8   5
---===========----
                7
*/

#[derive(Clone, Copy, PartialEq)]
struct Board
{
    to_move:Player,
    grid:[[Player; 3]; 3],
    line_scores:[i8; 8],
}

impl Board
{
    pub fn evaluate_board(&self) -> Player
    {
        for score in self.line_scores
        {
            if score ==  3 { return Player::X };
            if score == -3 { return Player::O };
        }
        Player::None
    }

    pub fn make_move(&mut self, cell:i8)
    {
        let cell_index = cell - 1;
        let row_index = cell_index / 3;
        let col_index = cell_index % 3;

        // !
        if self.grid[row_index as usize][col_index as usize] != Player::None { return };

        self.grid[row_index as usize][col_index as usize] = self.to_move;

        let value = self.to_move as i8;
        match cell
        {
            1 => { self.line_scores[0] += value; self.line_scores[3] += value; self.line_scores[7] += value; }
            2 => { self.line_scores[1] += value; self.line_scores[3] += value; }
            3 => { self.line_scores[2] += value; self.line_scores[3] += value; self.line_scores[6] += value; }
            4 => { self.line_scores[0] += value; self.line_scores[4] += value; }
            5 => { self.line_scores[1] += value; self.line_scores[4] += value; self.line_scores[6] += value; self.line_scores[7] += value }
            6 => { self.line_scores[2] += value; self.line_scores[4] += value; }
            7 => { self.line_scores[0] += value; self.line_scores[5] += value; self.line_scores[6] += value; }
            8 => { self.line_scores[1] += value; self.line_scores[5] += value; }
            9 => { self.line_scores[2] += value; self.line_scores[5] += value; self.line_scores[7] += value; }
            _ => {}
        }

        self.to_move = self.to_move.opposite();
    }
}

impl std::fmt::Display for Board
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // let temp = "()".to_owned() + self.line_scores[0].to_string().as_str() + self.grid[0][0].to_string().as_str();
        let r1 = format!(" {} │ {} │ {}\n", self.grid[0][0], self.grid[0][1], self.grid[0][2]);
        let r2 = "───┼───┼───\n";
        let r3 = format!(" {} │ {} │ {}\n", self.grid[1][0], self.grid[1][1], self.grid[1][2]);
        let r4 = "───┼───┼───\n";
        let r5 = format!(" {} │ {} │ {}\n", self.grid[2][0], self.grid[2][1], self.grid[2][2]);
        let r6 = format!("To move: {}\n", self.to_move);
        let temp = format!("{}{}{}{}{}{}", r1, r2, r3, r4, r5, r6);

        write!(f, "{}", temp)
    }
}

fn test_input(id:String) -> Error
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
        };
        played[cell_index as usize] = true;
    }
    Error::None
}

fn ab_minimax(board:Board, depth:u8, mut alpha:i8, mut beta:i8, maximizing_player:bool) -> i8
{
    if depth == 9 || board.evaluate_board() != Player::None
    {
        return board.evaluate_board() as i8;
    }

    if maximizing_player
    {
        let mut max_value = NEG_INFINITY as i8;
        for cell_index in 0 .. 9
        {
            let row_index = cell_index / 3;
            let col_index = cell_index % 3;
            if board.grid[row_index as usize][col_index as usize] == Player::None
            {
                let mut next_board = board.clone();
                next_board.make_move(cell_index + 1);
                println!("{}", next_board);
                let value = ab_minimax(next_board, depth + 1, alpha, beta, false);
                max_value = std::cmp::max(max_value, value);
                alpha = std::cmp::max(alpha, max_value);
                if beta <= alpha
                {
                    break;
                }
            }
        }
        return max_value;
    }
    else
    {
        let mut min_value = INFINITY as i8;
        for cell_index in 0 .. 9
        {
            let row_index = cell_index / 3;
            let col_index = cell_index % 3;
            if board.grid[row_index as usize][col_index as usize] == Player::None
            {
                let mut next_board = board.clone();
                next_board.make_move(cell_index + 1);
                println!("{}", next_board);
                let value = ab_minimax(next_board, depth + 1, alpha, beta, true);
                min_value = std::cmp::min(min_value, value);
                beta = std::cmp::min(beta, min_value);
                if beta <= alpha
                {
                    break;
                }
            }
        }
        return min_value;
    }
}

fn build_board(id: String) -> Board
{
    let mut temp_board = Board
    {
        to_move:Player::X,
        grid:[[Player::None; 3]; 3],
        line_scores:[0;8]
    };

    for cell in id.chars()
    {
        temp_board.make_move(cell.to_digit(10).unwrap() as i8);
        if temp_board.evaluate_board() != Player::None { return temp_board; }
    }

    println!("{}", temp_board);

    temp_board
}

fn main() -> std::io::Result<()>
{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        let current_line = line.unwrap();
        println!("{}", current_line);

        let error = test_input(current_line.clone());

        match error
        {
            Error::None => {
                let temp_board = build_board(current_line);
                println!("{}", temp_board);
                println!("{:?}", temp_board.line_scores);
            },
            Error::Input => {
                println!("ERROR: Invalid Input");
            },
            Error::Duplicate => {
                println!("ERROR: Duplicate Input");
            }
        }
    }

    println!("{}", ab_minimax(build_board("12".to_string()), 0, NEG_INFINITY as i8, INFINITY as i8, true));

    Ok(())
}
