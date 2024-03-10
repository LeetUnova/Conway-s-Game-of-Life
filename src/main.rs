use std::{thread, time::Duration, u8::MAX};
use rand::prelude::random;

fn main() {
    const WIDTH: usize = 150;
    const HEIGHT: usize = 40;

    let mut grid: [[u8; WIDTH]; HEIGHT] = randomize_grid();
    
    loop {
        let crowd: [[u8; WIDTH]; HEIGHT] = create_crowd(grid);
        grid = apply_crowd(grid, crowd);

        if is_empty(grid) {
            break;
        }

        display_grid(grid);

        thread::sleep(Duration::from_millis(100));
    }
}

fn is_empty<const W: usize, const H: usize>(grid: [[u8; W]; H]) -> bool {
    for row in 0..H {
        for col in 0..W {
            if grid[row][col] != 0 {
                return false;
            }
        }
    }

    true
}

fn randomize_grid<const W: usize, const H: usize>() -> [[u8; W]; H] {
    let mut randomized: [[u8; W]; H] = [[0; W]; H];

    for row in 0..H {
        for col in 0..W {
            randomized[row][col] = if random::<u8>() > MAX / 2 {
                1
            } else {
                0
            };
        }
    }

    randomized
}

fn display_grid<const W: usize, const H: usize>(grid: [[u8; W]; H]) {
    let mut lines: String = "".to_string();

    for row in 0..grid.len() {
        let mut line: String = "".to_string();
        
        for col in 0..grid[row].len() {
            line += if grid[row][col] == 0 {
                " "
            } else {
                "#"
            };
        }

        lines += &line;
        
        if row != grid.len() - 1 {
            lines += "\n";
        }
    }

    println!("{}[{}A{}", 27 as char, H, lines);
}

fn apply_crowd<const W: usize, const H: usize>(grid: [[u8; W]; H], crowd: [[u8; W]; H]) -> [[u8; W]; H] {
    let mut applied: [[u8; W]; H] = [[0; W]; H];

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let n: u8 = crowd[row][col];

            if n == 3 {
                applied[row][col] = 1;
                continue;
            }

            if n == 2 {
                applied[row][col] = grid[row][col];
                continue;
            }

            applied[row][col] = 0;
        }
    }

    applied
}

fn create_crowd<const W: usize, const H: usize>(grid: [[u8; W]; H]) -> [[u8; W]; H] {
    let mut crowd: [[u8; W]; H] = [[0; W]; H];

    for row in 0..H {
        for col in 0..W {
            crowd[row][col] = check_cell(grid, row, col);
        }
    }

    crowd
}

fn check_cell<const W: usize, const H: usize>(grid: [[u8; W]; H], row: usize, col: usize) -> u8 {
    let top: usize = if row > 0 {
        row - 1
    } else {
        row
    };

    let bottom: usize = if row < H - 1 {
        row + 1
    } else {
        row
    };

    let left: usize = if col > 0 {
        col - 1
    } else {
        col
    };

    let right: usize = if col < W - 1 {
        col + 1
    } else {
        col
    };

    let mut cell: u8 = 0;

    for y in top..(bottom + 1) {
        for x in left..(right + 1) {
            if y == row && x == col {
                continue;
            }

            cell += grid[y][x];
        }
    }

    cell
}
