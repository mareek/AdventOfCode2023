pub struct Day1;

impl crate::day_trait::DaySolver for Day1 {
    fn day_of_month(&self) -> i32 {
        return 1;
    }

    fn solve_first_problem(&self, file_content: &str) -> String {
        let mut sum = 0;
        for line in file_content.lines() {
            let mut has_digit = false;
            let mut first_digit = '0';
            let mut last_digit = '0';
            for char in line.chars().filter(|c| c.is_digit(10)) {
                if !has_digit {
                    has_digit = true;
                    first_digit = char;
                }
                last_digit = char;
            }

            if has_digit {
                let str_value: String = [first_digit, last_digit].iter().collect();
                let value: i32 = str_value.parse::<i32>().unwrap();
                sum += value;
            }
        }

        return format!("{sum}");
    }

    fn solve_second_problem(&self, file_content: &str) -> String {
        let literals = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut sum = 0;
        for line in file_content.lines() {
            let mut has_digit = false;
            let mut first_digit = '0';
            let mut first_digit_position = line.len() + 1;
            let mut has_last_digit = false;
            let mut last_digit = '0';
            let mut last_digit_position = 0;
            for i in 0..10 {
                match line.find(digits[i]) {
                    Some(pos) if !has_digit || pos < first_digit_position => {
                        has_digit = true;
                        first_digit_position = pos;
                        first_digit = digits[i];
                    }
                    _ => {}
                }
                match line.find(literals[i]) {
                    Some(pos) if !has_digit || pos < first_digit_position => {
                        has_digit = true;
                        first_digit_position = pos;
                        first_digit = digits[i];
                    }
                    _ => {}
                }
                match line.rfind(digits[i]) {
                    Some(pos) if !has_last_digit || last_digit_position < pos => {
                        has_last_digit = true;
                        last_digit_position = pos;
                        last_digit = digits[i];
                    }
                    _ => {}
                }
                match line.rfind(literals[i]) {
                    Some(pos) if !has_last_digit || last_digit_position < pos => {
                        has_last_digit = true;
                        last_digit_position = pos;
                        last_digit = digits[i];
                    }
                    _ => {}
                }
            }

            if has_digit {
                let str_value: String = [first_digit, last_digit].iter().collect();
                let value: i32 = str_value.parse::<i32>().unwrap();
                sum += value;
            }
        }

        return format!("{sum}");
    }
}
