pub struct Day11;

impl crate::day_trait::DaySolver for Day11 {
    fn day_of_month(&self) -> i32 {
        return 11;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let map = Map::parse(file_content)?;

        let mut total_distance: usize = 0;

        let expanded_map = map.expand();
        let galaxies = expanded_map.get_galaxies();
        for i in 0..galaxies.len() {
            let current = galaxies[i];
            for j in (i + 1)..galaxies.len() {
                let other = galaxies[j];
                total_distance += current.distance_from(&other);
            }
        }

        return Some(format!("{total_distance}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let map = Map::parse(file_content)?;
        let galaxies = map.get_galaxies();
        let empty_cols = map.get_empty_cols();
        let empty_rows = map.get_empty_rows();

        let mut total_distance: usize = 0;
        for i in 0..galaxies.len() {
            let current = galaxies[i];
            for j in (i + 1)..galaxies.len() {
                let other = galaxies[j];
                total_distance +=
                    current.expanded_distance_from(&other, &empty_cols, &empty_rows, 1000000);
            }
        }

        return Some(format!("{total_distance}"));
    }
}

struct Map {
    cells: Vec<Vec<bool>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Map {
    fn parse(file_content: &str) -> Option<Map> {
        let mut cells: Vec<Vec<bool>> = Vec::new();

        for line in file_content.lines() {
            let mut row: Vec<bool> = Vec::new();
            for char in line.chars() {
                row.push(char == '#');
            }

            if row.len() > 0 {
                cells.push(row);
            }
        }

        if cells.len() > 0 {
            return Some(Map { cells });
        } else {
            return None;
        }
    }

    fn expand(&self) -> Map {
        let mut cells: Vec<Vec<bool>> = Vec::new();
        let empty_cols = self.get_empty_cols();
        let expanded_width = self.get_width() + empty_cols.len();
        for y in 0..self.get_height() {
            let mut row: Vec<bool> = Vec::new();
            for x in 0..self.get_width() {
                row.push(self.get_cell(x, y));
                if empty_cols.contains(&x) {
                    row.push(false);
                }
            }
            cells.push(row);

            if self.row_is_empty(y) {
                let mut empty_row: Vec<bool> = Vec::new();
                for _ in 0..expanded_width {
                    empty_row.push(false);
                }

                cells.push(empty_row);
            }
        }

        return Map { cells };
    }

    fn row_is_empty(&self, row: usize) -> bool {
        return self.cells[row].iter().all(|c| !c);
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        return (0..self.get_height())
            .filter(|r| self.row_is_empty(*r))
            .collect();
    }

    fn col_is_empty(&self, col: usize) -> bool {
        return self.cells.iter().all(|row| !row[col]);
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        return (0..self.get_width())
            .filter(|c| self.col_is_empty(*c))
            .collect();
    }

    fn get_height(&self) -> usize {
        return self.cells.len();
    }

    fn get_width(&self) -> usize {
        return self.cells[0].len();
    }

    fn get_cell(&self, x: usize, y: usize) -> bool {
        return self.cells[y][x];
    }

    fn get_galaxies(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                if self.get_cell(x, y) {
                    result.push(Point { x, y });
                }
            }
        }

        return result;
    }
}

impl Point {
    fn distance_from(&self, other: &Point) -> usize {
        let horizontal_distance = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let vertical_distance = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        return horizontal_distance + vertical_distance;
    }

    fn expanded_distance_from(
        &self,
        other: &Point,
        empty_cols: &Vec<usize>,
        empty_rows: &Vec<usize>,
        exp_factor: usize,
    ) -> usize {
        let x_min = if other.x < self.x { other.x } else { self.x };
        let x_max = if other.x < self.x { self.x } else { other.x };
        let mut horizontal_distance = 0;
        for x in x_min..x_max {
            let dist = if empty_cols.contains(&x) {
                exp_factor
            } else {
                1
            };
            horizontal_distance += dist;
        }

        let y_min = if other.y < self.y { other.y } else { self.y };
        let y_max = if other.y < self.y { self.y } else { other.y };
        let mut vertical_distance = 0;
        for y in y_min..y_max {
            let dist = if empty_rows.contains(&y) {
                exp_factor
            } else {
                1
            };
            vertical_distance += dist;
        }

        return horizontal_distance + vertical_distance;
    }
}
