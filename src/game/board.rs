#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Corner,
    Black,
    White,
    King,
    Throne,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
    None,
}

pub trait HasColor {
    fn color(&self) -> Color;
}

impl HasColor for Tile {
    fn color(&self) -> Color {
        match &self {
            Tile::Empty | Tile::Corner | Tile::Throne => Color::None,
            Tile::Black => Color::Black,
            Tile::White | Tile::King => Color::White,
        }
    }
}

pub trait Captures {
    fn captures(&self, color: Color) -> bool;
}

impl Captures for Tile {
    fn captures(&self, color:Color) -> bool {
        match &self {
            Tile::Empty => false,
            Tile::Corner => true,
            Tile::Black => color == Color::White,
            Tile::White => color == Color::Black,
            Tile::King => color == Color::Black,
            Tile::Throne => true,
        }
    }
}

pub trait CapturesKing {
    fn captures_king(&self) -> bool;
}

impl CapturesKing for Tile {
    fn captures_king(&self) -> bool {
        match &self {
            Tile::Throne | Tile::Black => true,
            _ => false
        }
    }
}

pub trait Passable {
    fn passable(&self) -> bool;
}

impl Passable for Tile {
    fn passable(&self) -> bool {
        match &self {
            Tile::Empty | Tile::Throne | Tile::Corner => true,
            Tile::Black | Tile::White | Tile::King => false,
        }
    }
}

pub trait CanStandOn {
    fn can_stand_on(&self) -> bool;
}

impl CanStandOn for Tile {
    fn can_stand_on(&self) -> bool {
        match &self {
            Tile::Empty => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [[Tile; 11]; 11],
    pub winner: Color,
}

impl Board {
    pub fn new() -> Board {
        let mut new_board = [[Tile::Empty;11]; 11];

        // set the corner points
        new_board[0][0] = Tile::Corner;
        new_board[0][10] = Tile::Corner;
        new_board[10][0] = Tile::Corner;
        new_board[10][10] = Tile::Corner;
        // set the throne
        new_board[5][5] = Tile::Throne;

        Board { 
            board: new_board,
            winner: Color::None,
        }
    }

    pub fn from_string(str: String) -> Result<Board, String> {
        let mut new_board = [[Tile::Empty;11];11];

        // set the corner points
        new_board[0][0] = Tile::Corner;
        new_board[0][10] = Tile::Corner;
        new_board[10][0] = Tile::Corner;
        new_board[10][10] = Tile::Corner;
        // set the throne
        new_board[5][5] = Tile::Throne;

        fn char_to_u8(c: char) -> Result<u8, String> {
            match c {
                'B' => Ok(11),
                'A' => Ok(10),
                '9' => Ok(9),
                '8' => Ok(8),
                '7' => Ok(7),
                '6' => Ok(6),
                '5' => Ok(5),
                '4' => Ok(4),
                '3' => Ok(3),
                '2' => Ok(2),
                '1' => Ok(1),
                _ => Err(format!("Invalid character {}", c)),

            }
        }

        let mut row = 0;
        let mut col = 0;
        for (_i, c) in str.chars().enumerate() {
            match c {
                '/' => {
                    row += 1;
                    col = 0;
                },
                'b' => {
                    new_board[row][col] = Tile::Black;
                    col += 1;
                },
                'w' => {
                    new_board[row][col] = Tile::White; 
                    col += 1;
                },
                'k' => {
                    new_board[row][col] = Tile::King; 
                    col += 1;
                },
                _ => {
                    let u8_result = char_to_u8(c);
                    if u8_result.is_err() {
                        return Err(format!("Invalid character: {}", c));
                    }

                    col += usize::from(u8_result.unwrap());
                },
            }
        } 

        Ok(Board {
            board: new_board,
            winner: Color::None,
        })
    }

    pub fn to_string(&self) -> Result<String,String> {
        let mut str: String = String::from("");

        fn u8_to_char(num: u8) -> char {
            match num {
                11 => 'B',
                10 => 'A',
                9 => '9',
                8 => '8',
                7 => '7',
                6 => '6',
                5 => '5',
                4 => '4',
                3 => '3',
                2 => '2',
                1 => '1',
                _ => '0',
            }
        }

        for row in self.board.iter() {
            let mut empty_count: u8 = 0;
            for tile in row.iter() {
                match tile {
                    Tile::Empty | Tile::Corner | Tile::Throne => {
                        empty_count += 1;
                    },
                    _ => {
                        if empty_count > 0 {
                            str.push(u8_to_char(empty_count));
                        }
                        empty_count = 0;
                        match tile {
                            Tile::Black => str.push('b'),
                            Tile::White => str.push('w'),
                            Tile::King => str.push('k'),
                            _ => {return Err(format!("Invalid character {:?}", tile));},
                        };
                    }
                }
            }

            if empty_count > 0 {
                str.push(u8_to_char(empty_count));
            }
            str.push('/');
        }

        return Ok(str);
    }

    pub fn get_tile(&self, x:usize, y:usize) -> Result<Tile, String> {
        if x > 10 || y > 10 {
            return Err("Indexes must be between 0 and 10".to_string());
        }

        Ok(self.board[x][y])
    }

    pub fn set_tile(&mut self, new_tile:Tile, x:usize, y:usize) {
        self.board[x][y] = new_tile;
    }

    pub fn print_board(&self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    Tile::Empty => print!("."),
                    Tile::Corner | Tile::Throne => print!("X"),
                    Tile::Black => print!("B"),
                    Tile::White => print!("W"),
                    Tile::King => print!("K"),
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_string_conversion() {
        let mut board: Board = Board::new();
        board.set_tile(Tile::King, 5, 5);

        board.set_tile(Tile::Black, 4, 4);
        board.set_tile(Tile::Black, 5, 4);
        board.set_tile(Tile::Black, 6, 4);
        board.set_tile(Tile::Black, 7, 4);
        board.set_tile(Tile::Black, 8, 4);
        board.set_tile(Tile::Black, 9, 4);

        board.set_tile(Tile::White, 4, 6);
        board.set_tile(Tile::White, 5, 6);
        board.set_tile(Tile::White, 6, 6);
        board.set_tile(Tile::White, 7, 6);
        board.set_tile(Tile::White, 8, 6);
        board.set_tile(Tile::White, 9, 6);

        let string_conversion = board.to_string().unwrap();
        let new_board = Board::from_string(string_conversion).unwrap();

        assert_eq!(board.to_string().unwrap(), new_board.to_string().unwrap());

        assert_eq!(new_board.get_tile(5,5).unwrap(), Tile::King);

        assert_eq!(new_board.get_tile(4,4).unwrap(), Tile::Black);
        assert_eq!(new_board.get_tile(5,4).unwrap(), Tile::Black);
        assert_eq!(new_board.get_tile(6,4).unwrap(), Tile::Black);
        assert_eq!(new_board.get_tile(7,4).unwrap(), Tile::Black);
        assert_eq!(new_board.get_tile(8,4).unwrap(), Tile::Black);
        assert_eq!(new_board.get_tile(9,4).unwrap(), Tile::Black);

        assert_eq!(new_board.get_tile(4,6).unwrap(), Tile::White);
        assert_eq!(new_board.get_tile(5,6).unwrap(), Tile::White);
        assert_eq!(new_board.get_tile(6,6).unwrap(), Tile::White);
        assert_eq!(new_board.get_tile(7,6).unwrap(), Tile::White);
        assert_eq!(new_board.get_tile(8,6).unwrap(), Tile::White);
        assert_eq!(new_board.get_tile(9,6).unwrap(), Tile::White);
    }
}
