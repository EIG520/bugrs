#[derive(Clone)]
pub struct BitBug<const WIDTH: usize, const SIZE: usize> {
    tiles: TileGrid<WIDTH, SIZE>,
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

    pub fn tile_at_dir(&self, dir: &Direction) -> u32 {
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

#[derive(Clone, Copy)]
pub struct TileGrid<const WIDTH: usize, const SIZE: usize> {
    tiles: [u32; SIZE],
}

impl<const WIDTH: usize, const SIZE: usize> TileGrid<WIDTH, SIZE> {
    pub fn set(&mut self, x: usize, y: usize, value: u32) {
        self.tiles[x + y * WIDTH] = value;
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.tiles[x + y * WIDTH]
    }

    // umm hello??? based department???
    pub fn set_wall(&mut self, x: usize, y: usize) -> bool {
        let mut type_ones = 0;
        let mut type_twos = 0;
        let mut type_threes = vec![];

        for dy in 0..=2 {
            for dx in 0..=2 {
                let pos = (x + dx - 1, y + dy - 1);
                let val = self.get(x + dx - 1, y + dy - 1);

                match std::u32::MAX - val {
                    0 => {type_ones += 1},
                    1 => {type_twos += 1},
                    2 => {type_threes.push(pos)},
                    _ => {}
                }
            }
        }

        if type_ones > 0 && type_twos > 0 { return false; }
        if type_ones > 0 {
            self.set(x, y, std::u32::MAX);
            
            for &(x2, y2) in &type_threes {
                self.set_wall(x2, y2);
            }
            return true;
        }
        if type_twos > 0 {
            self.set(x, y, std::u32::MAX - 1);
            
            for &(x2, y2) in &type_threes {
                self.set_wall(x2, y2);
            }
            return true;
        }

        self.set(x,y, std::u32::MAX -2);
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
                    x if x > std::u32::MAX - 3 => {"#"}
                    _ => {" "}
                })
            }
        }

        s
    }
}

impl<const WIDTH: usize, const SIZE: usize> Default for TileGrid<WIDTH, SIZE> {
    fn default() -> Self {
        let mut grid = TileGrid { tiles: [0; SIZE] };

        for x in 0..(WIDTH) {
            grid.set(x, 0, std::u32::MAX - 1);
            grid.set(x, SIZE / WIDTH - 1, std::u32::MAX);
        }

        for y in 0..(SIZE / WIDTH) {
            grid.set(0, y, std::u32::MAX);
            grid.set(WIDTH - 1, y, std::u32::MAX - 1);
        }

        grid
    }
}