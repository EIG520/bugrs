#[derive(Clone)]
pub struct BitBug<const WIDTH: usize, const SIZE: usize> {
    pub tiles: TileGrid<WIDTH, SIZE>,
    pos: (usize, usize),
    dir: Direction,
    pub steps: u64
}

impl<const WIDTH: usize, const SIZE: usize> Default for BitBug<WIDTH, SIZE> {
    fn default() -> Self {
        Self {
            tiles: TileGrid::<WIDTH, SIZE>::default(),
            pos: (1,1),
            dir: Direction::Up,
            steps: 0
        }
    }
}

impl<const WIDTH: usize, const SIZE: usize> From<String> for BitBug<WIDTH, SIZE> {
    fn from(value: String) -> Self {
        Self {
            tiles: TileGrid::<WIDTH, SIZE>::from(value),
            pos: (1,1),
            dir: Direction::Up,
            steps: 0
        }
    }
}

impl<const WIDTH: usize, const SIZE: usize> ToString for BitBug<WIDTH, SIZE> {
    fn to_string(&self) -> String {
        self.tiles.to_string()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left, // 00 => (-1, 0)
    Up,   // 01 => (0, -1)
    Down, // 10 => (0,  1)
    Right // 11 => (1,  0)
}

impl Direction {
    pub fn move_coordinates(&self, coords: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Left => ( coords.0 - 1,  coords.1),
            Direction::Up   => ( coords.0,  coords.1 - 1),
            Direction::Down => ( coords.0,  coords.1 + 1),
            Direction::Right=> ( coords.0 + 1,  coords.1)
        }
    }
}

impl<const WIDTH: usize, const SIZE: usize> BitBug<WIDTH, SIZE> {
    pub fn take_step(&mut self) {
        self.steps += 1;
        self.tiles.set(self.pos.0, self.pos.1, self.tiles.get(self.pos.0, self.pos.1) + 1);

        let mut mv = self.dir;
        let mut score = self.tile_at_dir(&self.dir);

        for d in [Direction::Down, Direction::Right, Direction::Up, Direction::Left] {
            if self.tile_at_dir(&d) < score {
                score = self.tile_at_dir(&d);
                mv = d;
            }
        }


        self.pos = mv.move_coordinates(self.pos);
        self.dir = mv;
    }

    pub fn tile_at_dir(&self, dir: &Direction) -> Tile {
        let oset = dir.move_coordinates(self.pos);
        self.tiles.get(oset.0, oset.1)
    }

    pub fn is_done(&self) -> bool {
        self.pos == (WIDTH - 2, SIZE / WIDTH - 2)
    }

    pub fn unseen(&self) -> bool {
        self.tiles.get(self.pos.0, self.pos.1) == 0
    }

    pub fn pos(&self) -> (usize, usize) {
        self.pos
    }

    pub fn set_wall(&mut self, x: usize, y: usize) -> bool {
        self.tiles.set_wall(x, y)
    }
}


// pub struct MicroGrid<const WIDTH: usize, const SIZE: usize> {
//     type1_board: u128,
//     type2_board: u128,
//     type3_board: u128,
// }

type Tile = u16;
const TILE_MAX: Tile = std::u16::MAX;

#[derive(Clone, Copy)]
pub struct TileGrid<const WIDTH: usize, const SIZE: usize> {
    tiles: [Tile; SIZE],
    pub walls: u16,
}

impl<const WIDTH: usize, const SIZE: usize> TileGrid<WIDTH, SIZE> {
    pub fn set(&mut self, x: usize, y: usize, value: Tile) {
        self.tiles[x + y * WIDTH] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * WIDTH]
    }

    // umm hello??? based department???
    pub fn set_wall(&mut self, x: usize, y: usize) -> bool {
        if self.get(x,y) < TILE_MAX - 2 { self.walls += 1; }

        let mut type_ones = 0;
        let mut type_twos = 0;
        let mut type_threes = vec![];

        for (dy, dx) in [(0,0),(1,0),(2,0),(0,1),(2,1),(0,2),(1,2),(2,2)] {
            let pos = (x + dx - 1, y + dy - 1);
            let val = self.get(x + dx - 1, y + dy - 1);

            match TILE_MAX - val {
                0 => {type_ones += 1},
                1 => {type_twos += 1},
                2 => {type_threes.push(pos)},
                _ => {}
            }
        }

        if type_ones > 0 && type_twos > 0 { return false; }
        if type_ones > 0 {
            self.set(x, y, TILE_MAX);
            
            for &(x2, y2) in &type_threes {
                self.set_wall(x2, y2);
            }
            return true;
        }
        if type_twos > 0 {
            self.set(x, y, TILE_MAX - 1);
            
            for &(x2, y2) in &type_threes {
                self.set_wall(x2, y2);
            }
            return true;
        }

        self.set(x,y, TILE_MAX -2);
        true
    }
}

impl<const WIDTH: usize, const SIZE: usize> ToString for TileGrid<WIDTH, SIZE> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for y in 0..(SIZE/WIDTH) {
            s = format!("{s}\n");
            for x in 0..(WIDTH) {
                s = format!("{s}{}", match self.get(x,y) {
                    x if x > TILE_MAX - 3 => {"#"}
                    _ => {" "}
                })
            }
        }

        s = format!("{s}\n{}", self.walls);

        s
    }
}

impl<const WIDTH: usize, const SIZE: usize> From<String> for TileGrid<WIDTH, SIZE> {
    fn from(disp: String) -> Self {
        let mut slf = TileGrid::default();
        let srows = disp.split("\n");

        for (y,row) in srows.enumerate().skip(1).take(SIZE / WIDTH - 2) {
            for (x, c) in row.chars().enumerate().skip(1).take(WIDTH - 2) {
                if c == '#' {
                    slf.set_wall(x, y);
                }
            }
        }

        slf
    }
}
impl<const WIDTH: usize, const SIZE: usize> Default for TileGrid<WIDTH, SIZE> {
    fn default() -> Self {
        let mut grid = TileGrid { tiles: [0; SIZE], walls: 0 };

        for x in 0..(WIDTH) {
            grid.set(x, 0, TILE_MAX - 1);
            grid.set(x, SIZE / WIDTH - 1, TILE_MAX);
        }

        for y in 0..(SIZE / WIDTH) {
            grid.set(0, y, TILE_MAX);
            grid.set(WIDTH - 1, y, TILE_MAX - 1);
        }

        grid
    }
}