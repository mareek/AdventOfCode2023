pub struct Day14;

impl crate::day_trait::DaySolver for Day14 {
    fn day_of_month(&self) -> i32 {
        return 14;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let mut platform = Platform::parse(file_content)?;
        platform.tilt_north();
        let result = platform.compute_north_load();
        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let original_platform = Platform::parse(file_content)?;
        let mut previous_states: Vec<Platform> = Vec::new();
        let mut cycle_info: Option<(usize, usize)> = None;
        let mut platform = original_platform.clone();
        for i in 0..1000 {
            platform.cycle();
            if previous_states.iter().any(|p| *p == platform) {
                let offset = previous_states
                    .iter()
                    .take_while(|p| **p != platform)
                    .count();
                let cycle_len = i - offset;
                cycle_info = Some((offset, cycle_len));
                break;
            } else {
                previous_states.push(platform.clone());
            }
        }
        match cycle_info {
            None => return None,
            Some((offset, cycle_len)) => {
                platform = original_platform.clone();
                println!("offset : {offset}, cycle: {cycle_len}");
                let cycle_count = (1_000_000_000 - offset) % cycle_len;
                for _ in 0..(offset+cycle_count) {
                    platform.cycle();
                }
                let result = platform.compute_north_load();
                return Some(format!("{result}"));
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Rock {
    Round,
    Square,
    Empty,
}

#[derive(PartialEq, Eq)]
struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Rock {
    fn parse(chr: &char) -> Option<Rock> {
        match chr {
            '.' => Some(Rock::Empty),
            'O' => Some(Rock::Round),
            '#' => Some(Rock::Square),
            _ => None,
        }
    }
}

impl Clone for Platform {
    fn clone(&self) -> Self {
        let mut result = Vec::new();
        for row in self.rocks.iter() {
            let mut new_row = Vec::new();
            for rock in row.iter() {
                new_row.push(*rock);
            }
            result.push(new_row);
        }
        return Platform { rocks: result };
    }
}

impl Platform {
    fn width(&self) -> usize {
        self.rocks[0].len()
    }

    fn height(&self) -> usize {
        self.rocks.len()
    }

    fn parse(file_content: &str) -> Option<Platform> {
        let mut rows = Vec::new();
        for line in file_content.lines() {
            let row = line
                .chars()
                .map(|c| Rock::parse(&c))
                .collect::<Option<Vec<Rock>>>()?;
            rows.push(row);
        }

        return Some(Platform { rocks: rows });
    }

    fn tilt_north(&mut self) {
        for row_start in 1..self.height() {
            for row in (1..=row_start).rev() {
                let prev_row = row - 1;
                for col in 0..self.width() {
                    if self.rocks[prev_row][col] == Rock::Empty
                        && self.rocks[row][col] == Rock::Round
                    {
                        self.rocks[prev_row][col] = Rock::Round;
                        self.rocks[row][col] = Rock::Empty;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for row_start in (1..self.height()).rev() {
            for row in 0..row_start {
                let prev_row = row + 1;
                for col in 0..self.width() {
                    if self.rocks[prev_row][col] == Rock::Empty
                        && self.rocks[row][col] == Rock::Round
                    {
                        self.rocks[prev_row][col] = Rock::Round;
                        self.rocks[row][col] = Rock::Empty;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for col_start in 1..self.width() {
            for col in (1..=col_start).rev() {
                let prev_col = col - 1;
                for row in 0..self.height() {
                    if self.rocks[row][prev_col] == Rock::Empty
                        && self.rocks[row][col] == Rock::Round
                    {
                        self.rocks[row][prev_col] = Rock::Round;
                        self.rocks[row][col] = Rock::Empty;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for col_start in (1..self.width()).rev() {
            for col in 0..col_start {
                let prev_col = col + 1;
                for row in 0..self.height() {
                    if self.rocks[row][prev_col] == Rock::Empty
                        && self.rocks[row][col] == Rock::Round
                    {
                        self.rocks[row][prev_col] = Rock::Round;
                        self.rocks[row][col] = Rock::Empty;
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn compute_north_load(&self) -> usize {
        let mut result = 0;
        for i in 0..self.height() {
            let factor = self.height() - i;
            let row_count = self.rocks[i].iter().filter(|r| **r == Rock::Round).count();
            result += factor * row_count;
        }
        return result;
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("");
        for row in self.rocks.iter() {
            for rock in row {
                match rock {
                    Rock::Empty => print!("."),
                    Rock::Round => print!("O"),
                    Rock::Square => print!("#"),
                };
            }
            print!("\n");
        }
        println!("");
    }
}
