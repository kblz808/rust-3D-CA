use macroquad::prelude::*;

const GRID_SIZE: usize = 10;
const CELL_SIZE: f32 = 1.0;
const STATE_COUNT: u8 = 5;

struct Grid {
    cells: [[[bool; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
    state: [[[u8; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
    neighbor_rule: &'static str,
    survival_rule: Vec<u8>,
    spawn_rule: Vec<u8>,
}

impl Grid {
    fn new() -> Self {
        Self {
            cells: [[[false; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
            state: [[[STATE_COUNT; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
            neighbor_rule: "V",
            survival_rule: vec![4],
            spawn_rule: vec![4],
        }
    }

    fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                for z in 0..GRID_SIZE {
                    let state = match self.state[x][y][z] {
                        s if s <= 0 => 0,
                        s if s != 5 => s - 1,
                        _ => 5,
                    };

                    if state == 0 {
                        self.cells[x][y][z] = false;
                    }

                    let neighbors = self.count_neighbors(x, y, z, self.neighbor_rule);
                    if self.cells[x][y][z] {
                        if !self.survival_rule.contains(&neighbors) {}
                    } else {
                        if self.spawn_rule.contains(&neighbors) {
                            new_cells[x][y][z] = true;
                            self.state[x][y][z] = 5;
                        }
                    }
                }
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize, z: usize, rule: &'static str) -> u8 {
        2
    }
}

fn main() {
    println!("Hello, world!");
}
