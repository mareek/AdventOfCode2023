pub struct Day16;

impl crate::day_trait::DaySolver for Day16 {
    fn day_of_month(&self) -> i32 {
        return 16;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let map = Map::parse(file_content);
        let result = map.measure_beam_energy(-1, 0, Direction::E);
        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let map = Map::parse(file_content);
        let mut best = 0_usize;

        //from top
        for x in 0..map.width() {
            let energy = map.measure_beam_energy(x, -1, Direction::S);
            if best < energy {
                best = energy;
            }
        }

        //from bottom
        for x in 0..map.width() {
            let energy = map.measure_beam_energy( x, map.height(), Direction::N);
            if best < energy {
                best = energy;
            }
        }

        //from left
        for y in 0..map.height() {
            let energy = map.measure_beam_energy(-1, y, Direction::S);
            if best < energy {
                best = energy;
            }
        }

        //from right
        for y in 0..map.height() {
            let energy = map.measure_beam_energy(map.width(), y, Direction::S);
            if best < energy {
                best = energy;
            }
        }

        return Some(format!("{best}"));
    }
}

fn init_heat_map(map: &Map) -> Vec<Vec<Vec<Direction>>> {
    let mut heat_map: Vec<Vec<Vec<Direction>>> = Vec::new();
    for _ in 0..map.height() {
        let mut row = Vec::new();
        for _ in 0..map.width() {
            row.push(Vec::new());
        }
        heat_map.push(row);
    }
    heat_map
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Mirror {
    Horizontal,
    Vertical,
    BendForward,
    BendBackward,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

struct Map {
    mirrors: Vec<Vec<Option<Mirror>>>,
}

impl Mirror {
    fn parse(chr: char) -> Option<Mirror> {
        match chr {
            '-' => Some(Self::Horizontal),
            '|' => Some(Self::Vertical),
            '/' => Some(Self::BendForward),
            '\\' => Some(Self::BendBackward),
            _ => None,
        }
    }

    fn compute_direction(&self, direction: Direction) -> Vec<Direction> {
        let mut result = Vec::new();
        match self {
            Mirror::BendBackward => result.push(match direction {
                Direction::N => Direction::W,
                Direction::S => Direction::E,
                Direction::E => Direction::S,
                Direction::W => Direction::N,
            }),
            Mirror::BendForward => result.push(match direction {
                Direction::N => Direction::E,
                Direction::S => Direction::W,
                Direction::E => Direction::N,
                Direction::W => Direction::S,
            }),
            Mirror::Horizontal => {
                if direction == Direction::N || direction == Direction::S {
                    result.push(Direction::E);
                    result.push(Direction::W);
                } else {
                    result.push(direction)
                }
            }
            Mirror::Vertical => {
                if direction == Direction::E || direction == Direction::W {
                    result.push(Direction::N);
                    result.push(Direction::S);
                } else {
                    result.push(direction)
                }
            }
        }
        return result;
    }
}

impl Direction {
    fn next_move(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Self::N => (x, y - 1),
            Self::S => (x, y + 1),
            Self::E => (x + 1, y),
            Self::W => (x - 1, y),
        }
    }

    fn get_label(&self) -> char {
        match self {
            Self::N => '^',
            Self::S => 'v',
            Self::E => '>',
            Self::W => '<',
        }
    }
}

impl Map {
    fn parse(file_content: &str) -> Map {
        let mut mirrors = Vec::new();
        for line in file_content.lines() {
            let row = line.chars().map(|c| Mirror::parse(c)).collect();
            mirrors.push(row);
        }
        return Map { mirrors };
    }

    fn height(&self) -> isize {
        self.mirrors.len() as isize
    }

    fn width(&self) -> isize {
        self.mirrors[0].len() as isize
    }

    fn launch_beam(
        &self,
        x_prev: isize,
        y_prev: isize,
        dir: Direction,
        heat_map: &mut Vec<Vec<Vec<Direction>>>,
        recursion: u32,
    ) {
        let (x, y) = dir.next_move(x_prev, y_prev);
        if x < 0
            || self.width() <= x
            || y < 0
            || self.height() <= y
            || heat_map[y as usize][x as usize].contains(&dir)
            || 10_000 < recursion
        {
            return;
        }

        heat_map[y as usize][x as usize].push(dir);

        match self.mirrors[y as usize][x as usize] {
            None => self.launch_beam(x, y, dir, heat_map, recursion + 1),
            Some(mirror) => {
                for new_dir in mirror.compute_direction(dir).iter() {
                    self.launch_beam(x, y, *new_dir, heat_map, recursion + 1);
                }
            }
        }
    }

    fn measure_beam_energy(&self, x_start: isize, y_start: isize, dir_start: Direction) -> usize {
        let mut heat_map = init_heat_map(self);
        self.launch_beam(x_start, y_start, dir_start, &mut heat_map, 0);

        return heat_map
            .iter()
            .map(|r| r.iter().filter(|d| d.len() > 0).count())
            .sum();
    }

    #[allow(dead_code)]
    fn print_heat_map(&self, heat_map: &Vec<Vec<Vec<Direction>>>) {
        println!("");
        for y in 0..self.height() as usize {
            for x in 0..self.width() as usize {
                let cell = &heat_map[y][x];
                match cell[..] {
                    [] => print!("."),
                    [single] => print!("{}", single.get_label()),
                    _ => print!("{}", format!("{}", cell.len())),
                };
            }
            print!("\n");
        }
    }
}
