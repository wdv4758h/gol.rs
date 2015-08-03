// Conway's Game of Life in Rust

struct Symbol {
    dead: char,
    live: char,
}

struct Board {
    symbol: Symbol,
    rows: usize,
    cols: usize,
    data: Vec<Vec<bool>>,
}

trait GOL {
    fn update(&mut self);
    fn display(&self);
}

impl GOL for Board {
    fn update(&mut self) {

        let mut newdata = vec![vec![false; self.rows+2]; self.cols+2];

        for i in 1..self.rows+1 {
            for j in 1..self.cols+1 {

                let neighbour = [ self.data[i-1][j-1],
                                  self.data[i-1][j],
                                  self.data[i-1][j+1],
                                  self.data[i][j-1],
                                  self.data[i][j+1],
                                  self.data[i+1][j-1],
                                  self.data[i+1][j],
                                  self.data[i+1][j+1],
                                ].iter()
                                 .filter(|status| **status )    // &&bool -> bool
                                 .count();

                newdata[i][j] = match (self.data[i][j], neighbour) {
                                    (false, 3) => true,
                                    (true,  2) => true,
                                    (true,  3) => true,
                                    _ => false,
                                };
            }
        }

        self.data = newdata;
    }

    fn display(&self) {

        // Clear entire screen
        // Move cursor to upper left corner
        print!("\x1b[2J\x1b[H");

        println!("{}", "Conway's Game of Life");

        for i in 1..self.rows+1 {

            for j in 1..self.cols+1 {
                print!("{}", if self.data[i][j] { self.symbol.live } else { self.symbol.dead });
            }

            println!("");
        }
    }
}

fn main() {
    use std::env;

    let rows: usize = 15;
    let cols: usize = 15;

    let mut board = Board {
                        symbol: Symbol { dead: '_', live: 'O' },
                        rows: rows, cols: cols,
                        data: vec![vec![false; rows+2]; cols+2],
                    };

    {
        // Init
        // I just set some bits randomly
        board.data[2][2] = true;
        board.data[2][3] = true;
        board.data[1][3] = true;
        board.data[3][3] = true;
        board.data[4][3] = true;
        board.data[4][5] = true;
        board.data[5][5] = true;
        board.data[6][7] = true;
        board.data[7][5] = true;
        board.data[8][7] = true;
        board.data[9][7] = true;
        board.data[9][8] = true;
        board.data[10][7] = true;
    }

    // parsing command line
    let iteration = env::args()
                        .skip(1)
                        .next()
                        .unwrap_or(String::new())
                        .parse()
                        .unwrap_or(0);

    for _ in 0..iteration+1 {
        board.display();
        board.update();
    }
}
