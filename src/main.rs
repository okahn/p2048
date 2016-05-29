extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

type Square = u16;
type Board = [Square; 16];

enum Dir {
    Up,
    Down,
    Left,
    Right
}

/* Shift the board in the given direction. */
fn shift_board(dir: Dir, b: &Board) -> Board {
    let mut new_board = [0; 16];
    let i1 = |i| match dir {
        Dir::Left  => (4*i, 4*i+1, 4*i+2, 4*i+3),
        Dir::Right => (4*i+3, 4*i+2, 4*i+1, 4*i),
        Dir::Up    => (i, 4+i, 8+i, 12+i),
        Dir::Down  => (12+i, 8+i, 4+i, i),
    };
    let i2 = |i, j| match dir {
        Dir::Left  => 4*i + j,
        Dir::Right => 4*i + 3-j,
        Dir::Up    => 4*j + i,
        Dir::Down  => 4*(3-j) + i,
    };
    for i in 0..4 {
        let line = shift_line([b[i1(i).0],
                               b[i1(i).1],
                               b[i1(i).2],
                               b[i1(i).3]]);
        for j in 0..4 {
            new_board[i2(i, j)] = line[j];
        }
    }
    return new_board;
}

/* Shift a single game line to the left. */
fn shift_line(line: [Square; 4]) -> [Square; 4] {
    let mut new_line = [0; 4];
    let mut acc = vec![];
    let mut out = vec![];
    for &x in &line {
        if x != 0 {
            acc.push(x);
        }
    }
    while !acc.is_empty() {
        if acc.len() >= 2 && acc[0] == acc[1] {
            out.push(acc.remove(0) + 1);
            acc.remove(0);
        } else {
            out.push(acc.remove(0));
        }
    }
    let mut i = 0;
    for x in out {
        new_line[i] = x;
        i += 1;
    }
    return new_line;
}

/* Randomly place a new piece. */
fn place(b: &Board) -> Board {
    let mut rng = rand::thread_rng();
    let mut ret = [0; 16];
    let mut acc = vec![];
    for i in 0..16 {
        ret[i] = b[i];
        if b[i] == 0 {
            acc.push(i);
        }
    }
    let between = Range::new(0, acc.len());
    let x : f64 = rng.gen();
    let new = if x < 0.9 { 1 } else { 2 };
    let i = between.ind_sample(&mut rng);
    ret[acc[i]] = new;
    return ret;
}

fn main() {
    println!("{:?}", shift_board(Dir::Left , &[0, 0, 1, 1,
                                               0, 1, 0, 1,
                                               1, 0, 1, 1,
                                               1, 2, 2, 1]));
}
