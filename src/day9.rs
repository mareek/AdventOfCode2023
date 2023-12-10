pub struct Day9;

impl crate::day_trait::DaySolver for Day9 {
    fn day_of_month(&self) -> i32 {
        return 9;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let measures_collection = parse_input(file_content)?;
        let mut result = 0;
        for measures in measures_collection {
            result += extrapolate(&measures);
        }
        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let measures_collection = parse_input(file_content)?;
        let mut result = 0;
        for measures in measures_collection {
            let value = extrapolate_backward(&measures);
            result += value;
        }
        return Some(format!("{result}"));
    }
}

fn parse_input(file_content: &str) -> Option<Vec<Vec<i64>>> {
    let mut result = Vec::new();
    for line in file_content.lines() {
        let measures: Vec<i64> = line
            .split_whitespace()
            .map(|c| c.parse().ok())
            .collect::<Option<Vec<_>>>()?;
        result.push(measures);
    }
    return Some(result);
}

fn extrapolate(values: &Vec<i64>) -> i64 {
    if values.len() == 0 || values.iter().all(|v| *v == 0) {
        return 0;
    } else {
        let derived_values = derive(values);
        return values.last().unwrap() + extrapolate(&derived_values);
    }
}

fn derive(values: &Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 1..values.len() {
        result.push(values[i] - values[i - 1]);
    }
    return result;
}

fn extrapolate_backward(values: &Vec<i64>) -> i64 {
    if values.len() == 0 || values.iter().all(|v| *v == 0) {
        return 0;
    } else {
        let derived_values = derive(values);
        return values.first().unwrap() - extrapolate_backward(&derived_values);
    }
}