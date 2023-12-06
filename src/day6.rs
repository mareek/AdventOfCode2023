pub struct Day6;

impl crate::day_trait::DaySolver for Day6 {
    fn day_of_month(&self) -> i32 {
        return 6;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let races = parse_races(file_content)?;
        let mut result = 1;
        for race in races {
            let winning_strategies = race.get_winning_strategies_count();
            result *= winning_strategies;
        }
        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let race = parse_race(file_content)?;
        let winning_strategies_count = race.get_winning_strategies_count();
        return Some(format!("{winning_strategies_count}"));
    }
}

fn parse_race(file_content: &str) -> Option<Race> {
    let mut lines = file_content.lines();
    let time: i64 = lines
        .next()?
        .split(':')
        .last()?
        .split_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse()
        .ok()?;
    let distance: i64 = lines
        .next()?
        .split(':')
        .last()?
        .split_whitespace()
        .collect::<Vec<_>>()
        .concat()
        .parse()
        .ok()?;

    return Some(Race { time, distance });
}

fn parse_races(file_content: &str) -> Option<Vec<Race>> {
    // Time:      7  15   30
    // Distance:  9  40  200

    let mut lines = file_content.lines();
    let times: Vec<i64> = lines
        .next()?
        .split(':')
        .last()?
        .split_whitespace()
        .map(|n| n.parse().ok())
        .collect::<Option<Vec<_>>>()?;
    let distances: Vec<i64> = lines
        .next()?
        .split(':')
        .last()?
        .split_whitespace()
        .map(|n| n.parse().ok())
        .collect::<Option<Vec<_>>>()?;

    let mut result: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        result.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }

    return Some(result);
}

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn get_winning_strategies_count(&self) -> i64 {
        let mut result=0;
        for button_press_duration in 0..self.time {
            let distance = self.compute_distance(button_press_duration);
            if self.distance < distance {
                result+=1;
            }
        }
        return result;
    }

    fn compute_distance(&self, button_press_duration: i64) -> i64 {
        if button_press_duration <= 0 || self.time <= button_press_duration {
            return 0;
        } else {
            return button_press_duration * (self.time - button_press_duration);
        }
    }
}
