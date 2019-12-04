use std::collections::HashSet;
type Piece = Vec<(i8, i8, char)>;

#[derive(Clone)]
struct Board {
    board: [Option<char>; 64],
    pieces: Vec<(Piece, (usize, usize))>
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [None; 64],
            pieces: Vec::new()
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
    // 1      
    // x        .
    // .x.x  .x.x
    let L1 = vec![(0, 0, 'x'), (0, -1, '.'), (1, -1, 'x'), (2, -1, '.'), (3, -1, 'x')];
    let L1 = rotations(L1);
    let L2 = vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (3, 0, 'x'), (3, 1, '.')];
    let L2 = rotations(L2);
    let mut L = HashSet::new();
    for x in L1.iter().chain(L2.iter()) {
        L.insert(x);
    }


    // 2
    // x       .
    // .x.   x.x
    //   x   .
    let lightning1 = vec![(0, 0, 'x'), (0, -1, '.'), (1, -1, 'x'), (2, -1, '.'), (2, -2, 'x')];
    let lightning1 = rotations(lightning1);
    let lightning2 = vec![(0, 0, '.'), (0, -1, 'x'), (-1, -1, '.'), (-2, -1, 'x'), (-2, -2, '.')];
    let lightning2 = rotations(lightning2);
    let mut lightning = HashSet::new();
    for x in lightning1.iter().chain(lightning2.iter()) {
        lightning.insert(x);
    }

    // 3
    // .x.  .S.
    //  .    .
    //  x    x
    let big_t1 = vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (1, -1, '.'), (1, -2, 'x')];
    let big_t1 = rotations(big_t1);
    let big_t2 = vec![(0, 0, '.'), (1, 0, 'S'), (2, 0, '.'), (1, -1, '.'), (1, -2, 'x')];
    let big_t2 = rotations(big_t2);
    let mut big_t = HashSet::new();
    for x in big_t1.iter().chain(big_t2.iter()) {
        big_t.insert(x);
    }

    // 4
    //  x    .
    // x.x  .x.
    // .      x
    let weirdo1 = vec![(0, 0, '.'), (0, 1, 'x'), (1, 1, '.'), (2, 1, 'x'), (1, 2, 'x')];
    let weirdo1 = rotations(weirdo1);
    let weirdo2 = vec![(0, 0, 'x'), (0, 1, '.'), (-1, 1, 'x'), (-2, 1, '.'), (-1, 2, '.')];
    let weirdo2 = rotations(weirdo2);
    let mut weirdo = HashSet::new();
    for x in weirdo1.iter().chain(weirdo2.iter()) {
        weirdo.insert(x);
    }

    // 5
    // x.x  R.x
    //  x    x
    let small_t1 = vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (1, -1, 'x')];
    let small_t1 = rotations(small_t1);
    let small_t2 = vec![(0, 0, 'R'), (1, 0, '.'), (2, 0, 'x'), (1, -1, 'x')];
    let small_t2 = rotations(small_t2);
    let mut small_t = HashSet::new();
    for x in small_t1.iter().chain(small_t2.iter()) {
        small_t.insert(x);
    }

    // 6
    // .x.  .x.
    // x      D
    // .      .
    let v1 = vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (0, -1, 'x'), (0, -2, '.')];
    let v1 = rotations(v1);
    let v2 = vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (2, -1, 'D'), (2, -2, '.')];
    let v2 = rotations(v2);
    let mut v = HashSet::new();
    for x in v1.iter().chain(v2.iter()) {
        v.insert(x);
    }

    // 7
    //  .    x
    // .x.  x.x
    //  .    x
    let plus1 = vec![(0, 0, 'x'), (1, 0, '.'), (0, 1, '.'), (0, -1, '.'), (-1, 0, '.')];
    let plus1 = rotations(plus1);
    let plus2 = vec![(0, 0, '.'), (1, 0, 'x'), (0, 1, 'x'), (0, -1, 'x'), (-1, 0, 'x')];
    let plus2 = rotations(plus2);
    let mut plus = HashSet::new();
    for x in plus1.iter().chain(plus2.iter()) {
        plus.insert(x);
    }

    // 8
    // x.x.x  x.x.E
    let line1 = vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (4, 0, 'x')];
    let line1 = rotations(line1);
    let line2 = vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (4, 0, 'E')];
    let line2 = rotations(line2);
    let mut line = HashSet::new();
    for x in line1.iter().chain(line2.iter()) {
        line.insert(x);
    }


    // 9
    // .x.    x.x
    //   x.  x.
    let s1 = vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (2, -1, 'x'), (3, -1, '.')];
    let s1 = rotations(s1);
    let s2 = vec![(0, 0, 'x'), (-1, 0, '.'), (-2, 0, 'x'), (-2, -1, '.'), (-3, -1, 'x')];
    let s2 = rotations(s2);
    let mut s = HashSet::new();
    for x in s1.iter().chain(s2.iter()) {
        s.insert(x);
    }

    // 10
    // x.x.   x.x.
    //  x       .
    let hammer1 = vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (1, -1, 'x')];
    let hammer1 = rotations(hammer1);
    let hammer2 = vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (3, 0, '.'), (2, -1, '.')];
    let hammer2 = rotations(hammer2);
    let mut hammer = HashSet::new();
    for x in hammer1.iter().chain(hammer2.iter()) {
        hammer.insert(x);
    }

    // 11
    // R.x   x.x
    // . .   . .
    let big_c1 = vec![(0, 0, 'R'), (1, 0, '.'), (2, 0, 'x'), (0, -1, '.'), (2, -1, '.')];
    // let big_c1 = vec![(0, 0, 'R'), (1, 0, '1'), (2, 0, '2'), (0, -1, '3'), (2, -1, '4')];
    let big_c1 = rotations(big_c1);

    // let big_c2 = vec![(0, 0, 'x'), (1, 0, '.'), (2, 0, 'x'), (0, -1, '.'), (2, -1, '.')];
    let big_c2 = vec![];
    let big_c2 = rotations(big_c2);
    let mut big_c = HashSet::new();
    for x in big_c1.iter().chain(big_c2.iter()) {
        big_c.insert(x);
    }

    // 12
    // S.    .x
    //  x.  .x
    //   x  x
    let stairs1 = vec![(0, 0, 'S'), (1, 0, '.'), (1, -1, 'x'), (2, -1, '.'), (2, -2, 'x')];
    let stairs1 = rotations(stairs1);
    let stairs2 = vec![(0, 0, 'x'), (-1, 0, '.'), (-1, -1, 'x'), (-2, -1, '.'), (-2, -2, 'x')];
    let stairs2 = rotations(stairs2);
    let mut stairs = HashSet::new();
    for x in stairs1.iter().chain(stairs2.iter()) {
        stairs.insert(x);
    }

    // 13
    // D.x  .x.
    //  x.  x.
    let gun1 = vec![(0, 0, 'D'), (1, 0, '.'), (2, 0, 'x'), (1, -1, 'x'), (2, -1, '.')];
    let gun1 = rotations(gun1);
    let gun2 = vec![(0, 0, '.'), (1, 0, 'x'), (2, 0, '.'), (0, -1, 'x'), (1, -1, '.')];
    let gun2 = rotations(gun2);
    let mut gun = HashSet::new();
    for x in gun1.iter().chain(gun2.iter()) {
        gun.insert(x);
    }

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
        pieces: Vec::new()
    };
    solution.print();

    let mut possibles = vec![Board::new()];

    let board = possibles.pop().unwrap();

    for y in 0..8 {
        for x in 0..8 {
            for piece in &big_c {
                if piece.len() == 0 {
                    continue;
                }
                let mut curr_board = board.clone();
                if curr_board.place_piece(x, y, piece) {
                    if curr_board.check_solution(&solution) {
                        possibles.push(curr_board);
                    }
                }
            }
        }
    }

    for p in possibles {
        print!("-------\n");
        p.print();
    }
}
