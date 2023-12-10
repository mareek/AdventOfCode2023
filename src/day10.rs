use std::collections::{HashMap, HashSet};

pub struct Day10;

impl crate::day_trait::DaySolver for Day10 {
    fn day_of_month(&self) -> i32 {
        return 10;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let map = Map::parse(file_content)?;
        let paths = map.get_paths();
        let mut distances: HashMap<Point, isize> = HashMap::new();
        for path in paths {
            let mut distance = 1;
            for pos in path {
                match distances.get(&pos) {
                    None => distances.insert(pos, distance),
                    Some(v) if &distance < v => distances.insert(pos, distance),
                    _ => None,
                };
                distance += 1;
            }
        }

        let longest_distance = distances.values().max()?;

        return Some(format!("{longest_distance}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let map = Map::parse(file_content)?;
        let paths = map.get_paths();
        let start = map.get_start()?;
        let mut loop_cells: HashSet<Point> = HashSet::new();
        loop_cells.insert(start);
        for point in paths[0].iter() {
            loop_cells.insert(*point);
        }

        let mut inner_cells: HashSet<Point> = HashSet::new();
        let mut prev = &start;
        for cur in paths[0].iter() {
            let right = get_right_position(cur, prev);
            map.add_inner_cells(&loop_cells, &right[0], &mut inner_cells, 0);
            map.add_inner_cells(&loop_cells, &right[1], &mut inner_cells, 0);
            prev = cur;
        }

        //map.print_clean(file_content, &loop_cells, &inner_cells);

        let result = inner_cells.len();
        return Some(format!("{result}"));
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

struct Cell {
    is_start: bool,
    connect_up: bool,
    connect_right: bool,
    connect_down: bool,
    connect_left: bool,
}

struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn parse(file_content: &str) -> Option<Map> {
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for line in file_content.lines() {
            let mut line_cells: Vec<Cell> = Vec::new();
            for char in line.chars() {
                line_cells.push(Cell::parse(&char));
            }

            if line_cells.len() > 0 {
                cells.push(line_cells);
            }
        }

        if cells.len() > 0 {
            return Some(Map { cells });
        } else {
            return None;
        }
    }

    #[allow(dead_code)]
    fn print_clean(
        &self,
        file_content: &str,
        loop_cells: &HashSet<Point>,
        marked_cells: &HashSet<Point>,
    ) {
        let mut y = 0;
        for line in file_content.lines() {
            let mut x = 0;
            for chr in line.chars() {
                let pos = Point { x, y };
                let cell = self.get_cell(x, y);
                if cell.is_start {
                    print!("S");
                } else if marked_cells.contains(&pos) {
                    print!("X");
                } else if loop_cells.contains(&pos) {
                    print!("{}", to_beter_char(&chr));
                } else {
                    print!(" ");
                }
                x += 1;
            }
            print!("\n");
            y += 1;
        }
    }

    fn get_height(&self) -> isize {
        return isize::try_from(self.cells.len()).unwrap();
    }

    fn get_width(&self) -> isize {
        return isize::try_from(self.cells[0].len()).unwrap();
    }

    fn get_cell(&self, x: isize, y: isize) -> &Cell {
        return &self.cells[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()];
    }

    fn get_start(&self) -> Option<Point> {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                let cell = self.get_cell(x, y);
                if cell.is_start {
                    return Some(Point { x, y });
                }
            }
        }

        return None;
    }

    fn get_possible_moves(&self, pos: &Point) -> [Point; 2] {
        let current_cell = self.get_cell(pos.x, pos.y);
        let mut result = Vec::new();
        if pos.y > 0 {
            let up_cell = self.get_cell(pos.x, pos.y - 1);
            if current_cell.connect_up || (current_cell.is_start && up_cell.connect_down) {
                result.push(Point {
                    x: pos.x,
                    y: pos.y - 1,
                });
            }
        }

        if pos.x < (self.get_width() - 1) {
            let right_cell = self.get_cell(pos.x + 1, pos.y);
            if current_cell.connect_right || (current_cell.is_start && right_cell.connect_left) {
                result.push(Point {
                    x: pos.x + 1,
                    y: pos.y,
                });
            }
        }

        if pos.y < (self.get_height() - 1) {
            let down_cell = self.get_cell(pos.x, pos.y + 1);
            if current_cell.connect_down || (current_cell.is_start && down_cell.connect_up) {
                result.push(Point {
                    x: pos.x,
                    y: pos.y + 1,
                });
            }
        }

        if pos.x > 0 {
            let left_cell = self.get_cell(pos.x - 1, pos.y);
            if current_cell.connect_left || (current_cell.is_start && left_cell.connect_right) {
                result.push(Point {
                    x: pos.x - 1,
                    y: pos.y,
                });
            }
        }

        return [result[0], result[1]];
    }

    fn get_paths(&self) -> [Vec<Point>; 2] {
        let start = self.get_start().unwrap();
        let connected_paths = self.get_possible_moves(&start);
        return [
            self.get_path(&start, &connected_paths[0]),
            self.get_path(&start, &connected_paths[1]),
        ];
    }

    fn get_path(&self, start: &Point, next: &Point) -> Vec<Point> {
        let mut previous = *start;
        let mut current = *next;
        let mut result = Vec::new();

        while current != *start {
            result.push(current);
            let connected_cell = self.get_possible_moves(&current);
            if connected_cell[0] != previous {
                previous = current;
                current = connected_cell[0];
            } else {
                previous = current;
                current = connected_cell[1];
            }
        }

        return result;
    }

    fn add_inner_cells(
        &self,
        loop_cells: &HashSet<Point>,
        pos: &Point,
        inner_cells: &mut HashSet<Point>,
        recursion: usize,
    ) {
        if recursion > 100 {
            return;
        }
        if !(0 <= pos.x && pos.x < self.get_width() && 0 <= pos.y && pos.y < self.get_height()) {
            return;
        }
        if loop_cells.contains(pos) || inner_cells.contains(pos) {
            return;
        }

        //println!("add inner cell x:{} y:{}", pos.x, pos.y);
        inner_cells.insert(*pos);

        for y in (pos.y - 1)..=(pos.y + 1) {
            for x in (pos.x - 1)..=(pos.x + 1) {
                self.add_inner_cells(loop_cells, &Point { x, y }, inner_cells, recursion + 1);
            }
        }
    }
}

fn get_right_position(cur: &Point, prev: &Point) -> [Point; 2] {
    if cur.x == prev.x {
        let x = if prev.y < cur.y { cur.x - 1 } else { cur.x + 1 };
        return [Point { x, y: cur.y }, Point { x, y: prev.y }];
    } else {
        //cur.y == prev.y
        let y = if prev.x < cur.x { cur.y + 1 } else { cur.y - 1 };
        return [Point { x: cur.x, y }, Point { x: prev.x, y }];
    }
}

impl Cell {
    fn parse(char: &char) -> Cell {
        return match char {
            'S' => Cell {
                is_start: true,
                connect_up: false,
                connect_right: false,
                connect_down: false,
                connect_left: false,
            },
            '|' => Cell {
                is_start: false,
                connect_up: true,
                connect_right: false,
                connect_down: true,
                connect_left: false,
            },
            '-' => Cell {
                is_start: false,
                connect_up: false,
                connect_right: true,
                connect_down: false,
                connect_left: true,
            },
            'L' => Cell {
                is_start: false,
                connect_up: true,
                connect_right: true,
                connect_down: false,
                connect_left: false,
            },
            'J' => Cell {
                is_start: false,
                connect_up: true,
                connect_right: false,
                connect_down: false,
                connect_left: true,
            },
            '7' => Cell {
                is_start: false,
                connect_up: false,
                connect_right: false,
                connect_down: true,
                connect_left: true,
            },
            'F' => Cell {
                is_start: false,
                connect_up: false,
                connect_right: true,
                connect_down: true,
                connect_left: false,
            },
            _ => Cell {
                is_start: false,
                connect_up: false,
                connect_right: false,
                connect_down: false,
                connect_left: false,
            },
        };
    }
}

fn to_beter_char(chr: &char) -> char {
    match chr {
        'L' => '┗',
        'J' => '┛',
        '7' => '┓',
        'F' => '┏',
        '|' => '┃',
        '-' => '━',
        c => c.clone(),
    }
}
