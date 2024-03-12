use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, PartialEq)]
enum Player
{
    None = 0,
    X = 1, // first
    O = -1 // second
}

struct Board
{
    to_play: Player,
    grid:[[Player; 3]; 3],
    scores: [i8; 7],
}

impl Board
{
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

/*  0   1   2   3
---===========----
    1 | 2 | 3   4
---===========----
    4 | 5 | 6   5
---===========----
    7 | 8 | 9   6
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
    let mut to_play = Player::X;

    let mut grid = [[Player::None; 3]; 3];
    let mut row = 0;
    let mut col = 0;

    for (play, cell) in id.chars().enumerate()
    {
        let cell_value = cell.to_digit(10).unwrap() - 1;
        row = cell_value / 3;
        col = cell_value % 3;
        grid[row as usize][col as usize] = to_play;
        to_play = if to_play == Player::X { Player::O } else { Player::X };
    }

    todo!()

    // return Board
    // {
    //     to_play: to_play,
    //     grid: grid,

    // }
}



fn main() -> std::io::Result<()>
{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines()
    {
        let current_line = line.unwrap();
    }

    todo!()
}
