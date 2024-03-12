use std::fs::File;
use std::io::{BufRead, BufReader};

enum ERROR
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
            1 => { self.line_scores[0] += value; self.line_scores[3] += value; self.line_scores[7] += value }
            2 => { self.line_scores[1] += value; self.line_scores[3] += value }
            3 => { self.line_scores[2] += value; self.line_scores[3] += value; self.line_scores[6] += value }
            4 => { self.line_scores[0] += value; self.line_scores[4] += value; }
            5 => { self.line_scores[1] += value; self.line_scores[4] += value; self.line_scores[6] += value; self.line_scores[7] += value }
            6 => { self.line_scores[2] += value; self.line_scores[4] += value; }
            7 => { self.line_scores[0] += value; self.line_scores[5] += value; self.line_scores[6] += value }
            8 => { self.line_scores[1] += value; self.line_scores[5] += value; }
            9 => { self.line_scores[2] += value; self.line_scores[5] += value; self.line_scores[7] += value }
            _ => {}
        }

        self.to_move = if self.to_move == Player::X { Player::O } else { Player::X };
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
        let r5 = format!(" {} │ {} │ {}", self.grid[2][0], self.grid[2][1], self.grid[2][2]);
        let temp = format!("{}{}{}{}{}", r1, r2, r3, r4, r5);

        write!(f, "{}", temp)
    }
}

fn test_input(id:String) -> ERROR
{
    let mut played = [false; 9];
    for cell in id.chars()
    {
        if !cell.is_numeric() || cell == '0'
        {
            return ERROR::Input;
        }

        let cell_index = cell.to_digit(10).unwrap() - 1;
        if played[cell_index as usize] == true
        {
            return ERROR::Duplicate;
        };
        played[cell_index as usize] = true;
    }
    ERROR::None
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

    return temp_board;
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
            ERROR::None => {
                let temp_board = build_board(current_line);
                println!("{}", temp_board);
            },
            ERROR::Input => {
                println!("ERROR: Invalid Input");
            },
            ERROR::Duplicate => {
                println!("ERROR: Duplicate Input");
            }
        }

        println!("");
    }

    Ok(())
}
