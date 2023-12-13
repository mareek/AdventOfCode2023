pub struct Day13;

impl crate::day_trait::DaySolver for Day13 {
    fn day_of_month(&self) -> i32 {
        return 13;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let mut result = 0;
        let patterns = parse_patterns(file_content);
        for pattern in patterns {
            for col_num in 1..pattern.get_width() {
                if pattern.has_reflection_starting_col(col_num) == Reflectivity::Perfect {
                    result += col_num;
                    break;
                }
            }
            for row_num in 1..pattern.get_height() {
                if pattern.has_reflection_starting_row(row_num) == Reflectivity::Perfect {
                    result += row_num * 100;
                    break;
                }
            }
        }

        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let mut result = 0;
        let patterns = parse_patterns(file_content);
        for pattern in patterns {
            for col_num in 1..pattern.get_width() {
                match pattern.has_reflection_starting_col(col_num) {
                    Reflectivity::WithSmudge(_) => {
                        result += col_num;
                        break;
                    }
                    _ => {}
                }
            }
            for row_num in 1..pattern.get_height() {
                match pattern.has_reflection_starting_row(row_num) {
                    Reflectivity::WithSmudge(_) => {
                        result += row_num * 100;
                        break;
                    }
                    _ => {}
                }
            }
        }

        return Some(format!("{result}"));
    }
}

fn parse_patterns(file_content: &str) -> Vec<Pattern> {
    let mut result = Vec::new();
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in file_content.lines() {
        let row: Vec<char> = line.chars().collect();
        if row.len() > 0 {
            rows.push(row);
        } else if rows.len() > 0 {
            result.push(Pattern { rows });
            rows = Vec::new();
        }
    }

    if rows.len() > 0 {
        result.push(Pattern { rows });
    }

    return result;
}

struct Pattern {
    rows: Vec<Vec<char>>,
}

#[derive(Eq, PartialEq)]
struct Smudge {
    row1: usize,
    col1: usize,
    row2: usize,
    col2: usize,
}

#[derive(Eq, PartialEq)]
enum Reflectivity {
    None,
    Perfect,
    WithSmudge(Smudge),
}

#[derive(Eq, PartialEq)]
enum LineReflectivity {
    None,
    Perfect,
    WithSmudge(usize),
}

impl Pattern {
    fn get_width(&self) -> usize {
        self.rows[0].len()
    }

    fn get_height(&self) -> usize {
        self.rows.len()
    }

    fn get_row(&self, row_num: usize) -> Vec<char> {
        self.rows[row_num].clone()
    }

    fn get_col(&self, col_num: usize) -> Vec<char> {
        self.rows.iter().map(|r| r[col_num]).collect()
    }

    fn has_reflection_starting_row(&self, row_num: usize) -> Reflectivity {
        if row_num == 0 {
            return Reflectivity::None;
        }

        let mut smudge: Option<Smudge> = None;

        for i in 0..self.get_height() {
            if i >= row_num || (i + row_num) >= self.get_height() {
                break;
            }
            let up_row_num = row_num - 1 - i;
            let down_row_num = row_num + i;
            let up_row = self.get_row(up_row_num);
            let down_row = self.get_row(down_row_num);

            match cells_are_equal(&up_row, &down_row) {
                LineReflectivity::Perfect => {}
                LineReflectivity::WithSmudge(col) if smudge == None => {
                    smudge = Some(Smudge {
                        row1: up_row_num,
                        col1: col,
                        row2: down_row_num,
                        col2: col,
                    });
                }
                _ => {
                    return Reflectivity::None;
                }
            };
            if cells_are_equal(&up_row, &down_row) == LineReflectivity::None {
                return Reflectivity::None;
            }
        }

        return match smudge {
            Some(s) => Reflectivity::WithSmudge(s),
            None => Reflectivity::Perfect,
        };
    }

    fn has_reflection_starting_col(&self, col_num: usize) -> Reflectivity {
        if col_num == 0 {
            return Reflectivity::None;
        }

        let mut smudge: Option<Smudge> = None;

        for i in 0..self.get_width() {
            if i >= col_num || (i + col_num) >= self.get_width() {
                break;
            }
            let left_col_num = col_num - 1 - i;
            let right_col_num = col_num + i;
            let left_col = self.get_col(left_col_num);
            let right_col = self.get_col(right_col_num);

            match cells_are_equal(&left_col, &right_col) {
                LineReflectivity::Perfect => {}
                LineReflectivity::WithSmudge(row) if smudge == None => {
                    smudge = Some(Smudge {
                        row1: row,
                        col1: left_col_num,
                        row2: row,
                        col2: right_col_num,
                    });
                }
                _ => {
                    return Reflectivity::None;
                }
            };
        }

        return match smudge {
            Some(s) => Reflectivity::WithSmudge(s),
            None => Reflectivity::Perfect,
        };
    }
}

fn cells_are_equal(cells1: &Vec<char>, cells2: &Vec<char>) -> LineReflectivity {
    if cells1.len() != cells2.len() {
        return LineReflectivity::None;
    }

    let mut smudge: Option<usize> = None;

    for i in 0..cells1.len() {
        if cells1[i] == cells2[i] {
            continue;
        } else if smudge == None {
            smudge = Some(i);
        } else {
            return LineReflectivity::None;
        }
    }
    return match smudge {
        None => LineReflectivity::Perfect,
        Some(s) => LineReflectivity::WithSmudge(s),
    };
}
