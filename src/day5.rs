use std::ops::Range;

pub struct Day5;

impl crate::day_trait::DaySolver for Day5 {
    fn day_of_month(&self) -> i32 {
        return 5;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let seeds = parse_seeds(file_content)?;
        let rule_sets = parse_rule_sets(file_content)?;
        let final_locations: Vec<usize> = seeds
            .iter()
            .map(|s| compute_final_destination(*s, &rule_sets))
            .collect();

        let closest_location = final_locations.iter().min()?;

        return Some(format!("{closest_location}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let seeds = parse_seeds(file_content)?;
        let rule_sets = parse_rule_sets(file_content)?;

        let mut closest_location = usize::MAX;

        let mut i = 0;
        while i < (seeds.len() - 1) {
            let start = seeds[i];
            let length = seeds[i + 1];
            for offset in 0..length {
                let seed = start + offset;
                let final_destination = compute_final_destination(seed, &rule_sets);
                if final_destination < closest_location {
                    closest_location = final_destination;
                }
            }

            i += 2;
        }

        return Some(format!("{closest_location}"));
    }
}

fn parse_seeds(file_content: &str) -> Option<Vec<usize>> {
    // seeds: 79 14 55 13
    return file_content
        .lines()
        .next()?
        .split(':')
        .last()?
        .split_whitespace()
        .map(|s| s.parse().ok())
        .collect();
}

fn parse_rule_sets(file_content: &str) -> Option<Vec<MapRuleSet>> {
    //seed-to-soil map:
    //50 98 2
    //52 50 48
    let mut result: Vec<MapRuleSet> = Vec::new();
    let mut current_rules: Vec<MapRule> = Vec::new();
    let mut map_name = String::from("dummy");
    for line in file_content.lines().skip(2) {
        if line.is_empty() {
            result.push(MapRuleSet {
                name: map_name.clone(),
                rules: current_rules.clone(),
            });
        } else if line.contains("map:") {
            map_name = String::from(line);
            current_rules.clear();
        } else {
            let numbers: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().ok())
                .collect::<Option<Vec<_>>>()?;

            current_rules.push(MapRule {
                dest_start: numbers[0],
                source_start: numbers[1],
                length: numbers[2],
            });
        }
    }

    result.push(MapRuleSet {
        name: map_name.clone(),
        rules: current_rules.clone(),
    });

    return Some(result);
}

fn compute_final_destination(source: usize, rule_sets: &Vec<MapRuleSet>) -> usize {
    let mut position = source;
    for rule_set in rule_sets.iter() {
        position = rule_set.compute_destination(position);
    }
    return position;
}

struct MapRule {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl MapRule {
    fn source_range(&self) -> Range<usize> {
        return self.source_start..self.source_start + self.length;
    }

    fn compute_destination(&self, source: usize) -> Option<usize> {
        if self.source_range().contains(&source) {
            let offset = source - self.source_start;
            return Some(offset + self.dest_start);
        } else {
            return None;
        }
    }
}

impl Clone for MapRule {
    fn clone(&self) -> MapRule {
        return MapRule {
            dest_start: self.dest_start,
            source_start: self.source_start,
            length: self.length,
        };
    }
}

struct MapRuleSet {
    name: String,
    rules: Vec<MapRule>,
}

impl MapRuleSet {
    fn compute_destination(&self, source: usize) -> usize {
        for rule in self.rules.iter() {
            match rule.compute_destination(source) {
                Some(n) => return n,
                None => {}
            }
        }

        return source;
    }
}

impl Clone for MapRuleSet {
    fn clone(&self) -> MapRuleSet {
        return MapRuleSet {
            name: self.name.clone(),
            rules: self.rules.clone(),
        };
    }
}
