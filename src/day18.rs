use std::result;

pub struct Day18;

impl crate::day_trait::DaySolver for Day18 {
    fn day_of_month(&self) -> i32 {
        return 18;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let mut trench: Vec<TrenchCell> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for instruction in file_content.lines().map(Instruction::parse) {
            (x, y) = instruction?.dig_trench(x, y, &mut trench);
        }
        let mut map = Map::from_trench(&trench)?;
        map.fill(1, 1);
        //map.print();

        let result = map.compute_dug_area();
        return Some(format!("{result}"));
    }

    #[allow(unused_variables)]
    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let instructions = file_content
            .lines()
            .map(Instruction::parse_alternate)
            .collect::<Option<Vec<Instruction>>>()?;

        //return Some(format!("PGCM ={}x{}", find_pgcm_horizontal(&instructions), find_pgcm_vertical(&instructions)));
        
        return Some(String::from("Distances are too long and my approach doesn't fork"));

        let mut trench: Vec<TrenchCell> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for instruction in instructions {
            //println!("{:?} {}", instruction.dir, instruction.len)
            (x, y) = instruction.dig_trench(x, y, &mut trench);
        }
        let mut map = Map::from_trench(&trench)?;
        map.fill(1, 1);
        //map.print();

        let result = map.compute_dug_area();
        return Some(format!("{result}"));
    }
}

fn find_pgcm_horizontal(instructions: &Vec<Instruction>) -> i64 {
    let mut result = 1;
    for i in 2..100_000 {
        let mut is_ok = true;
        for instruction in instructions
            .iter()
            .filter(|ins| ins.dir == Direction::E || ins.dir == Direction::W)
        {
            if (instruction.len % i) != 0 {
                is_ok = false;
                break;
            }
        }

        if is_ok {
            result = i;
        }
    }

    return result;
}

fn find_pgcm_vertical(instructions: &Vec<Instruction>) -> i64 {
    let mut result = 1;
    for i in 2..100_000 {
        let mut is_ok = true;
        for instruction in instructions
            .iter()
            .filter(|ins| ins.dir == Direction::N || ins.dir == Direction::S)
        {
            if (instruction.len % i) != 0 {
                is_ok = false;
                break;
            }
        }

        if is_ok {
            result = i;
        }
    }

    return result;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct TrenchCell {
    x: i64,
    y: i64,
}

struct Instruction {
    dir: Direction,
    len: i64,
}

struct Map {
    top: i64,
    left: i64,
    cells: Vec<Vec<bool>>,
}

impl Map {
    fn from_trench(trench: &Vec<TrenchCell>) -> Option<Map> {
        let top = trench.iter().map(|c| c.y).min()?;
        let left = trench.iter().map(|c| c.x).min()?;
        let bottom = trench.iter().map(|c| c.y).max()?;
        let right = trench.iter().map(|c| c.x).max()?;

        let mut cells = Vec::new();
        for _ in top..=bottom {
            let mut row = Vec::new();
            for _ in left..=right {
                row.push(false);
            }
            cells.push(row);
        }

        for cell in trench.iter() {
            let x = (cell.x - left) as usize;
            let y = (cell.y - top) as usize;
            cells[y][x] = true;
        }

        return Some(Map { top, left, cells });
    }

    fn bottom(&self) -> i64 {
        self.top + (self.cells.len() as i64) - 1
    }

    fn right(&self) -> i64 {
        self.left + (self.cells[0].len() as i64) - 1
    }

    fn get_cell(&self, x: i64, y: i64) -> bool {
        if x < self.left || self.right() < x || y < self.top || self.bottom() < y {
            panic!("(x:{x}, y:{y}) is out of bounds");
        }

        self.cells[(y - self.top) as usize][(x - self.left) as usize]
    }

    fn set_cell(&mut self, x: i64, y: i64, value: bool) {
        if x < self.left || self.right() < x || y < self.top || self.bottom() < y {
            panic!("(x:{x}, y:{y}) is out of bounds");
        }

        self.cells[(y - self.top) as usize][(x - self.left) as usize] = value;
    }

    fn fill(&mut self, x_start: i64, y_start: i64) {
        let mut coord_stack = Vec::new();
        coord_stack.push((x_start, y_start));
        while coord_stack.len() > 0 {
            let (x, y) = coord_stack.pop().unwrap();
            if x < self.left || self.right() < x || y < self.top || self.bottom() < y {
                continue;
            }

            if !self.get_cell(x, y) {
                self.set_cell(x, y, true);
                for i in -1..=1 {
                    for j in -1..=1 {
                        if !(i == 0 && j == 0) {
                            coord_stack.push((x + i, y + j));
                        }
                    }
                }
            }
        }
    }

    fn compute_dug_area(&self) -> i64 {
        let mut result = 0;
        for row in self.cells.iter() {
            for cell in row.iter() {
                if *cell {
                    result += 1
                }
            }
        }
        return result;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                if *cell {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

impl Instruction {
    fn parse(line: &str) -> Option<Instruction> {
        //R 6 (#70c710)
        let mut splitted = line.split_whitespace();
        let dir = Direction::parse(splitted.next()?)?;
        let len: i64 = splitted.next()?.parse().ok()?;
        return Some(Instruction { dir, len });
    }

    fn parse_alternate(line: &str) -> Option<Instruction> {
        //R 6 (#70c710)
        let hex_stuff = line.split_whitespace().last()?;
        let len = i64::from_str_radix(&hex_stuff[2..7], 16).ok()?;
        let dir = Direction::parse_hex(&hex_stuff[7..8])?;
        return Some(Instruction { dir, len });
    }

    fn dig_trench(&self, x_prev: i64, y_prev: i64, trench: &mut Vec<TrenchCell>) -> (i64, i64) {
        let (mut x, mut y) = (x_prev, y_prev);
        for _ in 0..self.len {
            (x, y) = self.dir.next_move(x, y);
            trench.push(TrenchCell { x, y });
        }

        return (x, y);
    }
}

impl Direction {
    fn parse(chr: &str) -> Option<Direction> {
        match chr {
            "U" => Some(Direction::N),
            "D" => Some(Direction::S),
            "L" => Some(Direction::W),
            "R" => Some(Direction::E),
            _ => None,
        }
    }

    fn parse_hex(chr: &str) -> Option<Direction> {
        //  0 means R, 1 means D, 2 means L, and 3 means U.
        match chr {
            "3" => Some(Direction::N),
            "1" => Some(Direction::S),
            "2" => Some(Direction::W),
            "0" => Some(Direction::E),
            _ => None,
        }
    }

    fn next_move(&self, x: i64, y: i64) -> (i64, i64) {
        match self {
            Direction::N => (x, y - 1),
            Direction::S => (x, y + 1),
            Direction::W => (x - 1, y),
            Direction::E => (x + 1, y),
        }
    }
}
