pub struct Day2;

impl crate::day_trait::DaySolver for Day2 {
    fn day_of_month(&self) -> i32 {
        return 2;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let games: Vec<Game> = file_content
            .lines()
            .filter(|l| l.len() > 8)
            .map(parse_game_line)
            .collect();

        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let result: i32 = games
            .iter()
            .filter(|g| g.get_max_red() <= 12 && g.get_max_green() <= 13 && g.get_max_blue() <= 14)
            .map(|g| g.id)
            .sum();
        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let games: Vec<Game> = file_content
            .lines()
            .filter(|l| l.len() > 8)
            .map(parse_game_line)
            .collect();

        let result: i32 = games
            .iter()
            .map(|g| g.get_max_red() * g.get_max_green() * g.get_max_blue())
            .sum();
        return Some(format!("{result}"));
    }
}

struct Game {
    id: i32,
    draws: Vec<Draw>,
}

struct Draw {
    red_count: i32,
    green_count: i32,
    blue_count: i32,
}

impl Game {
    pub fn get_max_red(&self) -> i32 {
        return self.draws.iter().map(|d| d.red_count).max().unwrap();
    }
    pub fn get_max_green(&self) -> i32 {
        return self.draws.iter().map(|d| d.green_count).max().unwrap();
    }
    pub fn get_max_blue(&self) -> i32 {
        return self.draws.iter().map(|d| d.blue_count).max().unwrap();
    }
}

fn parse_game_line(line: &str) -> Game {
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let split = &mut line.split(':');
    let game_label = split.next().unwrap();
    let id: i32 = game_label.split(' ').nth(1).unwrap().parse().unwrap();
    let draws_text = split.next().unwrap();

    let draws: Vec<Draw> = draws_text.split(';').map(parse_draw).collect();

    return Game { id, draws };
}

fn parse_draw(draw_text: &str) -> Draw {
    // 3 blue, 4 red
    let mut red_count = 0;
    let mut green_count = 0;
    let mut blue_count = 0;

    for color_group in draw_text.split(',').map(|g| g.trim()) {
        let mut group_split = color_group.split(' ');
        let count: i32 = group_split.next().unwrap().parse().unwrap();
        let color = group_split.next().unwrap();
        match color {
            "red" => red_count = count,
            "green" => green_count = count,
            "blue" => blue_count = count,
            _ => panic!("Unknown color: {color}"),
        }
    }
    return Draw {
        red_count,
        green_count,
        blue_count,
    };
}
