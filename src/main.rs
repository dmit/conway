use std::env;
use std::time::Duration;

#[derive(Copy, Clone, Debug)]
enum Cell {
    Dead,
    Live,
}

impl Cell {
    fn n(self) -> u8 {
        match self {
            Cell::Dead => 0,
            Cell::Live => 1,
        }
    }
}

#[derive(Clone)]
struct World {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl World {
    fn new(width: usize, height: usize) -> Result<World, String> {
        if width < 3 || height < 3 {
            return Err("the world cannot be smaller than 3x3".to_string());
        }
        let row = vec![Cell::Dead; width];
        let cells = vec![row; height];
        let world = World {
            cells,
            width,
            height,
        };
        Ok(world)
    }

    #[rustfmt::skip]
    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let t = if y == 0 { self.height - 1 } else { y - 1 };
        let b = if y == self.height - 1 { 0 } else { y + 1 };
        let l = if x == 0 { self.width - 1 } else { x - 1 };
        let r = if x == self.width - 1 { 0 } else { x + 1 };
        self.cells[t][l].n() +
        self.cells[t][x].n() +
        self.cells[t][r].n() +
        self.cells[y][l].n() +
        self.cells[y][r].n() +
        self.cells[b][l].n() +
        self.cells[b][x].n() +
        self.cells[b][r].n()
    }

    fn set(&mut self, x: usize, y: usize, value: Cell) {
        self.cells[y][x] = value;
    }

    /// Any live cell with fewer than two live neighbors dies, as if by underpopulation.
    /// Any live cell with two or three live neighbors lives on to the next generation.
    /// Any live cell with more than three live neighbors dies, as if by overpopulation.
    /// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
    fn advance(&mut self, tmp: &mut World) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let num_neighbors = self.count_neighbors(x, y);

                #[rustfmt::skip]
                let new_cell = match (*cell, num_neighbors) {
                    (Cell::Live, 0..=1) => Cell::Dead,
                    (Cell::Live, 2..=3) => Cell::Live,
                    (Cell::Live, _    ) => Cell::Dead,
                    (Cell::Dead, 3    ) => Cell::Live,
                    _                   => *cell,
                };
                tmp.set(x, y, new_cell);
            }
        }

        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                *cell = tmp.cells[y][x];
            }
        }
    }

    fn as_string(&self) -> String {
        let mut s = String::with_capacity(self.height * (self.width + 1));
        for row in self.cells.iter() {
            for cell in row {
                let symbol = match cell {
                    Cell::Dead => '.',
                    Cell::Live => 'O',
                };
                s.push(symbol);
            }
            s.push('\n');
        }
        s
    }
}

fn main() -> Result<(), String> {
    let width = env::args()
        .nth(1)
        .map(|n| n.parse::<usize>().expect("invalid width"))
        .unwrap_or(40);
    let height = env::args()
        .nth(2)
        .map(|n| n.parse::<usize>().expect("invalid height"))
        .unwrap_or(20);
    let generations = env::args()
        .nth(3)
        .map(|n| n.parse::<u64>().expect("invalid generations"))
        .unwrap_or(10);
    let delay = env::args()
        .nth(4)
        .map(|n| n.parse::<u64>().expect("invalid delay"))
        .unwrap_or(500);

    let mut world = World::new(width, height)?;
    let mut tmp = world.clone();

    // create a glider
    // ..O
    // O.O
    // .OO
    let mx = width / 2;
    let my = height / 2;
    world.set(mx + 1, my - 1, Cell::Live);
    world.set(mx - 1, my, Cell::Live);
    world.set(mx + 1, my, Cell::Live);
    world.set(mx, my + 1, Cell::Live);
    world.set(mx + 1, my + 1, Cell::Live);

    println!("{}", world.as_string());
    for _ in 0..generations {
        std::thread::sleep(Duration::from_millis(delay));
        world.advance(&mut tmp);
        println!("{}", world.as_string());
    }

    Ok(())
}

