pub struct Day12;

impl crate::day_trait::DaySolver for Day12 {
    fn day_of_month(&self) -> i32 {
        return 12;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let mut result = 0;
        for line in file_content.lines() {
            let row = Row::parse(line)?;
            let count_possible_states = row.count_possible_states();
            result += count_possible_states;
        }

        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let mut result = 0;
        for line in file_content.lines() {
            let row = Row::parse(line)?;
            let count_possible_states = row.count_possible_states();
            result += count_possible_states.pow(5);
        }

        return Some(format!("The brute force solution is way to slow : {result}"));
    }
}

struct Row {
    hot_springs: Vec<HotSpringState>,
    damaged_groups: Vec<u32>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum HotSpringState {
    Operational,
    Damaged,
    Unknown,
}

impl HotSpringState {
    fn from_char(chr: &char) -> HotSpringState {
        match chr {
            '.' => HotSpringState::Operational,
            '#' => HotSpringState::Damaged,
            _ => HotSpringState::Unknown,
        }
    }

    fn from_bit_mask(bit_mask: u64, index: &mut u32) -> HotSpringState {
        let result = if ((bit_mask >> *index) & 1) == 1 {
            HotSpringState::Operational
        } else {
            HotSpringState::Damaged
        };

        *index += 1;
        return result;
    }
}

impl Row {
    fn parse(line: &str) -> Option<Row> {
        // .??..??...?##. 1,1,3
        let mut splited = line.split(' ');
        let hot_springs: Vec<HotSpringState> = splited
            .next()?
            .chars()
            .map(|c| HotSpringState::from_char(&c))
            .collect();

        let damaged_groups: Vec<u32> = splited
            .next()?
            .split(',')
            .map(|c| c.parse().ok())
            .collect::<Option<Vec<_>>>()?;

        return Some(Row {
            hot_springs,
            damaged_groups,
        });
    }

    #[allow(dead_code)]
    fn parse_unfolded(line: &str) -> Option<Row> {
        // ???.### 1,1,3
        // ???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
        let mut splited = line.split(' ');
        let base_hot_springs: Vec<HotSpringState> = splited
            .next()?
            .chars()
            .map(|c| HotSpringState::from_char(&c))
            .collect();

        let mut hot_springs = Vec::new();
        for _ in 0..5 {
            if hot_springs.len() > 0 {
                hot_springs.push(HotSpringState::Unknown);
            }

            for hot_spring in base_hot_springs.iter() {
                hot_springs.push(*hot_spring);
            }
        }

        let base_damaged_groups: Vec<u32> = splited
            .next()?
            .split(',')
            .map(|c| c.parse().ok())
            .collect::<Option<Vec<_>>>()?;

        let mut damaged_groups = Vec::new();
        for _ in 0..5 {
            for group in base_damaged_groups.iter() {
                damaged_groups.push(*group);
            }
        }

        return Some(Row {
            hot_springs,
            damaged_groups,
        });
    }

    fn get_hypothesis(&self, bit_mask: u64) -> Vec<HotSpringState> {
        let mut result = Vec::new();
        let mut index = 0;
        for state in self.hot_springs.iter() {
            let final_state = match state {
                HotSpringState::Damaged => HotSpringState::Damaged,
                HotSpringState::Operational => HotSpringState::Operational,
                HotSpringState::Unknown => HotSpringState::from_bit_mask(bit_mask, &mut index),
            };

            result.push(final_state);
        }

        return result;
    }

    fn check_hypothesis(&self, hypothesis: &Vec<HotSpringState>) -> bool {
        //toto
        let mut in_group = false;
        let mut current_group_count: u32 = 0;
        let mut damaged_groups: Vec<u32> = Vec::new();

        for i in 0..hypothesis.len() {
            if hypothesis[i] == HotSpringState::Damaged {
                current_group_count += 1;
                in_group = true;
            } else if in_group {
                in_group = false;
                damaged_groups.push(current_group_count);
                current_group_count = 0;
            }
        }

        if in_group {
            damaged_groups.push(current_group_count);
        }

        return self.damaged_groups.len() == damaged_groups.len()
            && damaged_groups
                .iter()
                .zip(self.damaged_groups.iter())
                .all(|(a, b)| *a == *b);
    }

    fn count_possible_states(&self) -> u64 {
        let unknown_count = self
            .hot_springs
            .iter()
            .filter(|h| **h == HotSpringState::Unknown)
            .count() as u32;

        let hypothesis_count = 2_u64.pow(unknown_count);

        let mut result = 0;

        for hypothesis_index in 0..hypothesis_count {
            let hypothesis = self.get_hypothesis(hypothesis_index);
            if self.check_hypothesis(&hypothesis) {
                result += 1;
            }
        }

        return result;
    }
}
