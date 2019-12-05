#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
type Piece = Vec<(i8, i8, char)>;

const ID_LIGHTNING: u8 = 0;
const ID_BIGT: u8 = 1;
const ID_L: u8 = 2;
const ID_BIGC: u8 = 3;
const ID_WEIRDO: u8 = 4;
const ID_SMALLT: u8 = 5;
const ID_V: u8 = 6;
const ID_PLUS: u8 = 7;
const ID_LINE: u8 = 8;
const ID_S: u8 = 9;
const ID_HAMMER: u8 = 10;
const ID_STAIRS: u8 = 11;
const ID_GUN: u8 = 12;

macro_rules! make_shape {
    ($side1:expr, $side2:expr) => {
        {
            let side1 = rotations($side1);
            let side2 = rotations($side2);
            let mut patterns = Vec::new();
            for x in side1.iter().chain(side2.iter()) {
                let pattern = x.clone();
                // pattern.sort();
                if !patterns.contains(&pattern) {
                    patterns.push(pattern);
                }
            }
            patterns
        }
    }
}

lazy_static! {
    // R.x   x.x
    // . .   . .
    static ref BIGC: Vec<Piece> = make_shape!(
            vec![(0, 0, 'R'), (1, 0, '.'), (2, 0, 'x'), (0, -1, '.'), (2, -1, '.')],
            vec![]
            // vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (0, -1, '.'), (2, -1, '.')]
            );

    // .x.  .S.
    //  .    .
    //  x    x
    static ref BIGT: Vec<Piece> = make_shape!(
            vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (1, -1, '.'), (1, -2, 'x')],
            vec![(0, 0, '.'), (1, 0, 'S'), (2, 0, '.'), (1, -1, '.'), (1, -2, 'x')]);

    // x        .
    // .x.x  .x.x
    static ref L: Vec<Piece> = make_shape!(
            vec![(0, 0, 'x'), (0, -1, '.'), (1, -1, 'x'), (2, -1, '.'), (3, -1, 'x')],
            vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (3, 0, 'x'), (3, 1, '.')]);

    // x       .
    // .x.   x.x
    //   x   .
    static ref LIGHTNING: Vec<Piece> = make_shape!(
            vec![(0, 0, 'x'), (0, -1, '.'), (1, -1, 'x'), (2, -1, '.'), (2, -2, 'x')],
            vec![(0, 0, '.'), (0, -1, 'x'), (-1, -1, '.'), (-2, -1, 'x'), (-2, -2, '.')]);

    //  x    .
    // x.x  .x.
    // .      x
    static ref WEIRDO: Vec<Piece> = make_shape!(
            vec![(0, 0, '.'), (0, 1, 'x'), (1, 1, '.'), (2, 1, 'x'), (1, 2, 'x')],
            vec![(0, 0, 'x'), (0, 1, '.'), (-1, 1, 'x'), (-2, 1, '.'), (-1, 2, '.')]);

    // x.x  R.x
    //  x    x
    static ref SMALLT: Vec<Piece> = make_shape!(
            vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (1, -1, 'x')],
            vec![(0, 0, 'R'), (1, 0, '.'), (2, 0, 'x'), (1, -1, 'x')]);

    // .x.  .x.
    // x      D
    // .      .
    static ref V: Vec<Piece> = make_shape!(
            vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (0, -1, 'x'), (0, -2, '.')],
            vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (2, -1, 'D'), (2, -2, '.')]);

    //  .    x
    // .x.  x.x
    //  .    x
    static ref PLUS: Vec<Piece> = make_shape!(
            vec![(0, 0, 'x'), (1, 0, '.'), (0, 1, '.'), (0, -1, '.'), (-1, 0, '.')],
            vec![(0, 0, '.'), (1, 0, 'x'), (0, 1, 'x'), (0, -1, 'x'), (-1, 0, 'x')]);

    // x.x.x  x.x.E
    static ref LINE: Vec<Piece> = make_shape!(
            vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (4, 0, 'x')],
            vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (4, 0, 'E')]);

    // .x.    x.x
    //   x.  x.
    static ref S: Vec<Piece> = make_shape!(
            vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (2, -1, 'x'), (3, -1, '.')],
            vec![(0, 0, 'x'), (-1, 0, '.'), (-2, 0, 'x'), (-2, -1, '.'), (-3, -1, 'x')]);

    // x.x.   x.x.
    //  x       .
    static ref HAMMER: Vec<Piece> = make_shape!(
            vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (1, -1, 'x')],
            vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (2, -1, '.')]);

    // S.    .x
    //  x.  .x
    //   x  x
    static ref STAIRS: Vec<Piece> = make_shape!(
            vec![(0, 0, 'S'), (1, 0, '.'), (1, -1, 'x'), (2, -1, '.'), (2, -2, 'x')],
            // vec![(0, 0, 'x'), (-1, 0, '.'), (-1, -1, 'x'), (-2, -1, '.'), (-2, -2, 'x')]
            vec![]
            );

    // D.x  .x.
    //  x.  x.
    static ref GUN: Vec<Piece> = make_shape!(
            vec![(0, 0, 'D'), (1, 0, '.'), (2, 0, 'x'), (1, -1, 'x'), (2, -1, '.')],
            // vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (0, -1, 'x'), (1, -1, '.')]
            vec![]
            );
}

#[derive(Clone)]
struct Board {
    board: [Option<char>; 64],

    /// IDs of the pieces left to place on the board
    pieces_left: HashSet<u8>,

    /// ID of the Piece, index in the patterns array, (x, y) coord
    pieces: Vec<(u8, usize, i8, i8)>
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [None; 64],
            pieces_left: (0..13).collect(),
            pieces: vec![]
        }
    }

    pub fn mark(&mut self, x: i8, y: i8, marking: char) {
        assert!(x < 8, "x too large to mark");
        assert!(y < 8, "y too large to mark");
        let index = x as usize * 8 + y as usize;
        assert!(self.board[index].is_none());
        self.board[index] = Some(marking);
    }

    pub fn try_mark(&self, x: i8, y: i8) -> bool {
        if x >= 8 || x < 0 { return false; }
        if y >= 8 || y < 0 { return false; }
        let index = x as usize * 8 + y as usize;
        self.board[index].is_none()
    }

    pub fn place_piece(&mut self, curr_x: i8, curr_y: i8, piece: &Piece) -> bool {
        for (x, y, _ch) in piece {
            let x = curr_x + x;
            let y = curr_y + y;
            if !self.try_mark(x, y) { 
                return false;
            }
        }

        for (x, y, ch) in piece {
            let x = curr_x + x;
            let y = curr_y + y;
            self.mark(x, y, *ch);
        }

        true
    }

    pub fn print(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let curr_char = match self.board[y * 8 + x] {
                    None => '_',
                    Some(x) => x,
                };
                print!("{} ", curr_char);
            }
            print!("\n");
        }
    }

    pub fn check_solution(&self, solution: &Board) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let index = x as usize * 8 + y as usize;
                if self.board[index].is_none() {
                    continue;
                }

                if self.board[index] != solution.board[index] {
                    return false;
                }
            }
        }

        return true;
    }
}

/// Generate the possible rotations for a given piece
fn rotations(piece: Piece) -> Vec<Piece> {
    let mut result = Vec::new();
    result.push(piece.clone());
    let rotate_180: Piece = piece.iter().map(|x| (x.0 * -1, x.1 * -1, x.2)).collect();
    result.push(rotate_180);
    let rotate_90: Piece = piece.iter().map(|x| (x.1, x.0 * -1, x.2)).collect();
    result.push(rotate_90.clone());
    let rotate_270: Piece = rotate_90.iter().map(|x| (x.0 * -1, x.1 * -1, x.2)).collect();
    result.push(rotate_270);
    result
}

fn main() {
    let solution_board = 
      [Some('x'),Some('.'),Some('D'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),
       Some('.'),Some('E'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),
       Some('R'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),  
       Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'), 
       Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),  
       Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'), 
       Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),
       Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x'),Some('.'),Some('x')];

    let solution = Board {
        board: solution_board,
        pieces: Vec::new(),
        pieces_left: HashSet::new()

    };
    solution.print();

    let mut possibles = vec![Board::new()];
    'next_board: loop {
        print!("{}\n", possibles.len());
        let board = possibles.pop().expect("No possibles?!");
        for piece_id in &board.pieces_left {
            for y in 0..8 {
                for x in 0..8 {
                    let patterns: &Vec<Piece> = match piece_id {
                        &ID_BIGC => &BIGC,
                        &ID_BIGT => &BIGT,
                        &ID_L => &L,
                        &ID_LIGHTNING => &LIGHTNING,
                        &ID_WEIRDO => &WEIRDO,
                        &ID_SMALLT => &SMALLT,
                        &ID_V => &V,
                        &ID_PLUS => &PLUS,
                        &ID_LINE => &LINE,
                        &ID_S => &S,
                        &ID_HAMMER => &HAMMER,
                        &ID_STAIRS => &STAIRS,
                        &ID_GUN => &GUN,
                        _ => unreachable!()
                    };

                    for (i, piece) in patterns.iter().enumerate() {
                        if piece.len() == 0 {
                            continue;
                        }

                        let mut curr_board = board.clone();
                        if curr_board.place_piece(x, y, piece) {
                            if curr_board.check_solution(&solution) {
                                curr_board.pieces_left.remove(&piece_id);
                                curr_board.pieces.push((*piece_id, i, x, y));
                                if curr_board.pieces_left.len() == 0 {
                                    print!("FOUND\n");
                                    curr_board.print();
                                    loop {}
                                }
                                possibles.push(curr_board);
                                // continue 'next_board;
                            }
                        }
                    }
                }
            }
        }
    }
}

