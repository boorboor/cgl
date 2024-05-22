use std::io::{Write, stdout};
use termion::{cursor, clear, style};
use termion::raw::IntoRawMode;
use std::collections::HashSet;

type Coord = (u16, u16);

struct World {
    cells: HashSet<Coord>,
    width: u16,
    height: u16,
}

impl World {
    fn new(width: u16, height: u16) -> Self {
        Self {
            cells: HashSet::new(),
            width,
            height,
        }
    }

    fn render(&self, stdout: &mut impl Write) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = (x, y);
                let symbol = if self.cells.contains(&cell) { "█" } else { " " };
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), symbol).unwrap();
            }
        }
    }

    fn simulate(&mut self) {
        let mut next_generation = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = (x, y);
                let live_neighbors = self.count_live_neighbors(cell);
                if self.cells.contains(&cell) && (live_neighbors == 2 || live_neighbors == 3) {
                    next_generation.insert(cell);
                } else if !self.cells.contains(&cell) && live_neighbors == 3 {
                    next_generation.insert(cell);
                }
            }
        }
        self.cells = next_generation;
    }

    fn count_live_neighbors(&self, cell: Coord) -> u8 {
        let (x, y) = cell;
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Don't count the cell itself
                }
                let nx = (x as i16 + dx) as u16;
                let ny = (y as i16 + dy) as u16;
                if nx < self.width && ny < self.height && self.cells.contains(&(nx, ny)) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (width, height) = termion::terminal_size().unwrap();
    let mut world = World::new(width, height);

    world.cells.insert((2, 1));
    world.cells.insert((3, 2));
    world.cells.insert((1, 3));
    world.cells.insert((2, 3));
    world.cells.insert((3, 3));
    world.cells.insert((1, 1));
    world.cells.insert((1, 2));
    world.cells.insert((1, 3));
    world.cells.insert((1, 3));

    loop {
        write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
        world.render(&mut stdout);
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100)); 
        world.simulate();
    }
}
