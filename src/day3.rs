pub struct Day3;

impl crate::day_trait::DaySolver for Day3 {
    fn day_of_month(&self) -> i32 {
        return 3;
    }

    fn solve_first_problem(&self, file_content: &str) -> String {
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        parse_engine_schematics(file_content, &mut symbols, &mut part_numbers);

        let result: i32 = part_numbers
            .iter()
            .filter(|p| p.is_real_part_number(&symbols))
            .map(|p| p.number)
            .sum();

        return format!("{result}");
    }

    fn solve_second_problem(&self, file_content: &str) -> String {
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        parse_engine_schematics(file_content, &mut symbols, &mut part_numbers);

        let mut result = 0;
        for symbol in symbols {
            match symbol.get_gear_ratio(&part_numbers) {
                Some(ratio) => result += ratio,
                None => {}
            }
        }

        return format!("{result}");
    }
}

fn parse_engine_schematics(
    file_content: &str,
    symbols: &mut Vec<Symbol>,
    part_numbers: &mut Vec<PartNumber>,
) {
    let mut row: usize = 0;
    for line in file_content.lines() {
        let mut col: usize = 0;
        let mut is_parsing = false;
        let mut parse_start: usize = 0;
        for char in line.chars() {
            let mut is_digit = false;
            match char {
                '.' => {}
                c if c.is_digit(10) => {
                    is_digit = true;
                    if !is_parsing {
                        is_parsing = true;
                        parse_start = col;
                    }
                }
                c => symbols.push(Symbol {
                    symbol: c,
                    row,
                    col,
                }),
            }

            if is_parsing && !is_digit {
                let len = col - parse_start;
                let number_chars: String = line.chars().skip(parse_start).take(len).collect();
                let number: i32 = number_chars.parse().unwrap();
                part_numbers.push(PartNumber {
                    number,
                    row,
                    first_col: parse_start,
                    last_col: col - 1,
                });
                is_parsing = false;
            }

            col += 1;
        }

        if is_parsing {
            let len = col - parse_start;
            let number_chars: String = line.chars().skip(parse_start).take(len).collect();
            let number: i32 = number_chars.parse().unwrap();
            part_numbers.push(PartNumber {
                number,
                row,
                first_col: parse_start,
                last_col: col - 1,
            });
        }

        row += 1;
    }
}

struct Symbol {
    symbol: char,
    row: usize,
    col: usize,
}

struct PartNumber {
    number: i32,
    row: usize,
    first_col: usize,
    last_col: usize,
}

impl PartNumber {
    fn is_real_part_number(&self, symbols: &Vec<Symbol>) -> bool {
        return symbols.iter().any(|s| self.is_adjacent(s.row, s.col));
    }

    fn is_adjacent(&self, row: usize, col: usize) -> bool {
        let top = if self.row == 0 { 0 } else { self.row - 1 };
        let left = if self.first_col == 0 {
            0
        } else {
            self.first_col - 1
        };
        return top <= row && row <= (self.row + 1) && left <= col && col <= (self.last_col + 1);
    }
}

impl Symbol {
    fn get_gear_ratio(&self, part_numbers: &Vec<PartNumber>) -> Option<i32> {
        let adjacent_parts: Vec<&PartNumber> = part_numbers
            .iter()
            .filter(|p| p.is_adjacent(self.row, self.col))
            .collect();

        return match adjacent_parts.len() {
            2 => Some(adjacent_parts[0].number * adjacent_parts[1].number),
            _ => None,
        };
    }
}
