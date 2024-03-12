use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player
{
    None = 0,
    X = 1, // first
    O = -1 // second
}

#[derive(Debug)]
struct Board
{
    to_play: Player,
    grid:[[Player; 3]; 3],
    scores: [i8; 7],
}

impl Board
{
    pub fn update_scores(&mut self, player:Player, cell:i8)
    {
        let value = player as i8;
        match cell
        {
            1 => { self.scores[0] += value; self.scores[3] += value; self.scores[7] += value }
            2 => { self.scores[1] += value; self.scores[3] += value }
            3 => { self.scores[2] += value; self.scores[3] += value; self.scores[6] += value }
            4 => { self.scores[0] += value; self.scores[4] += value; }
            5 => { self.scores[1] += value; self.scores[4] += value; self.scores[6] += value; self.scores[7] += value }
            6 => { self.scores[2] += value; self.scores[4] += value; }
            7 => { self.scores[0] += value; self.scores[5] += value; self.scores[6] += value }
            8 => { self.scores[1] += value; self.scores[5] += value; }
            9 => { self.scores[2] += value; self.scores[5] += value; self.scores[7] += value }
            _ => {}
        }
    }

    pub fn evaluate(&self) -> Player
    {
        for score in self.scores
        {
            if score ==  3 { return Player::X };
            if score == -3 { return Player::O };
        }
        Player::None
    }
}

// * Note: for traits
impl ToString for Board
{
    fn to_string(&self) -> String
    {
        todo!()
    }
}

/*  0   1   2   6
---===========----
    1 | 2 | 3   3
---===========----
    4 | 5 | 6   4
---===========----
    7 | 8 | 9   5
---===========----
                7
*/

/*  0   1   2   3
---===========----
    0 | 1 | 2   4
---===========----
    3 | 4 | 5   5
---===========----
    6 | 7 | 8   6
---===========----
                7
*/

fn build_board(id: String) -> Board
{
    let to_play = if id.len() % 2 == 0 { Player::X } else { Player::O };

    let mut grid = [[Player::None; 3]; 3];

    for (play, cell) in id.chars().enumerate()
    {
        let turn = play % 2;
        let cell_value = cell.to_digit(10).unwrap() - 1;
        let row = cell_value / 3;
        let col = cell_value % 3;
        grid[row as usize][col as usize] = if turn == 0 { Player::X } else { Player::O };
    }

    let mut scores = [0; 7];



    return Board
    {
        to_play: to_play,
        grid: grid,
        scores: scores
    }
}



fn main() -> std::io::Result<()>
{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        let current_line = line.unwrap();

        println!("{}", current_line);
        let tempoard = build_board(current_line);
        println!("{:?}", tempoard.grid);
    }

    Ok(())
}
