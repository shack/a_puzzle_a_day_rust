use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::iter::zip;
use clap::Parser;
use colored::{Colorize,Color};
use itertools;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Piece {
    id: char,
    data: Vec<Vec<char>>,
}

impl Piece {
    fn width(&self) -> usize {
        return self.data[0].len();
    }

    fn height(&self) -> usize {
        return self.data.len();
    }

    fn coords(&self) -> itertools::Product<std::ops::Range<usize>, std::ops::Range<usize>> {
        return itertools::iproduct!(0..self.height(), 0..self.width());
    }

    fn from(s: &[&str]) -> Piece {
        let res = s[0].find(|c| c != '.').unwrap();
        let mut res = Piece {
            id: s[0].bytes().nth(res).unwrap() as char,
            data: vec![],
        };
        for line in s {
            res.data.push(line.chars().collect());
        }
        return res;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for r in &self.data {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
    }

    fn rev(&self) -> Piece {
        let mut res = Piece {
            id: self.id,
            data: vec![],
        };
        for r in &self.data {
            res.data.push(r.clone());
            res.data.last_mut().unwrap().reverse();
        }
        return res;
    }

    fn transpose(&self) -> Piece {
        let mut res = Piece {
            id: self.id,
            data: vec![],
        };
        for c in 0..self.width() {
            let mut row = vec![];
            for r in 0..self.height() {
                row.push(self.data[r][c]);
            }
            res.data.push(row);
        }
        return res;
    }

    fn rotate(&self) -> Piece {
        return self.rev().transpose();
    }

    fn generate_positions(&self) -> HashSet<Piece> {
        let mut res = HashSet::new();
        let rev = self.rev();
        for p in vec![self, &rev] {
            let mut q = p.clone();
            for _ in 0..4 {
                let r = q.rotate();
                res.insert(q);
                q = r;
            }
        }
        return res;
    }

    fn fit(&self, b: &Piece, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        if r + self.height() > b.height() || c + self.width() > b.width() {
            return res;
        }
        for (pr, pc) in self.coords() {
            let rr = r + pr;
            let cc = c + pc;
            if self.data[pr][pc] != '.' {
                if b.data[rr][cc] != '.' {
                    return vec![];
                }
                else {
                    res.push((rr, cc));
                }
            }
        }
        return res;
    }

}

const PIECES : [&[&str]; 8]  = [
    &[ "F..", "F..", "FFF" ],
    &[ "TTTT", ".T.." ],
    &[ "SS..", ".SSS" ],
    &[ "QQQ", "QQQ" ],
    &[ "Z..", "ZZZ", "..Z" ],
    &[ "L...", "LLLL" ],
    &[ "U.U", "UUU" ],
    &[ "BB.", "BBB" ]
];

const COLORS : [Color; 8] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
    Color::BrightBlack,
];

const BOARD : [&str; 7] = [
    "......#",
    "......#",
    ".......",
    ".......",
    ".......",
    ".......",
    "...####",
];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    month: usize,

    #[arg(short, long, default_value = "true")]
    color: bool,
}

struct Board {
    block_map: HashMap<char, String>,
    pieces: Vec<Vec<Piece>>,
    board: Piece,
    day: usize,
    month: usize,
    n: usize,
    calls: usize,
}

impl Board {
    fn new(args: &Args) -> Board {
        let mut board = Piece::from(&BOARD);
        let mut pieces = vec![];
        let mut block_map = HashMap::new();
        let color_enabled = args.color && "X".color(Color::Red).to_string().len() > 1;

        for (p, c) in zip(&PIECES, COLORS) {
            let piece = Piece::from(p);
            let pos : Vec<Piece> = piece.generate_positions().into_iter().collect();
            pieces.push(pos);
            if color_enabled {
                block_map.insert(piece.id, "██".color(c).to_string());
            }
        }

        let d = args.day - 1;
        let m = args.month - 1;
        board.data[m / 6][m % 6] = 'M';
        board.data[2 + d / 7][d % 7] = 'D';
        return Board { block_map, pieces, board,
            day: args.day, month: args.month, n: 1, calls: 0 };
    }

    fn print(&self) {
        for r in &self.board.data {
            for c in r {
                match c {
                    'M' => print!("{:0>2}", self.month),
                    'D' => print!("{:0>2}", self.day),
                    '#' => print!("  "),
                    _   => if let Some(s) = self.block_map.get(c) {
                            print!("{}", s);
                        } else {
                            print!("{}{}", c, c);
                        }
                }
            }
            println!("");
        }
    }

    fn _solve_dfs(&mut self, pieces: &Vec<Vec<Piece>>, piece_id: usize) {
        self.calls += 1;
        if piece_id == self.pieces.len() {
            println!("#{}:", self.n);
            self.print();
            self.n += 1;
            return;
        }
        for (r, c) in self.board.coords() {
            for p in &pieces[piece_id] {
                let occ = &p.fit(&self.board, r, c);
                if occ.len() == 0 {
                    continue;
                }
                for &(rr, cc) in occ.iter() {
                    self.board.data[rr][cc] = p.id;
                }
                self._solve_dfs(pieces, piece_id + 1);
                for &(rr, cc) in occ.iter() {
                    self.board.data[rr][cc] = '.';
                }
            }
        }
    }

    fn solve_dfs(&mut self) {
        self.n = 1;
        self.calls = 0;
        self._solve_dfs(&self.pieces.clone(), 0);
        println!("Calls: {}", self.calls);
    }

}

fn main() {
    let args = Args::parse();
    let mut board = Board::new(&args);
    board.solve_dfs();
}
