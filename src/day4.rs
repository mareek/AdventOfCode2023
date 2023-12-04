pub struct Day4;

impl crate::day_trait::DaySolver for Day4 {
    fn day_of_month(&self) -> i32 {
        return 4;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let cards = file_content
            .lines()
            .map(parse_line)
            .collect::<Option<Vec<_>>>()?;

        let score: u32 = cards.iter().map(|c| c.compute_points()).sum();
        return Some(format!("{score}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let cards = file_content
            .lines()
            .map(parse_line)
            .collect::<Option<Vec<_>>>()?;
        let mut factors: Vec<u32> = cards.iter().map(|_| 1).collect();

        for card in cards {
            let score = card.count_winning_number_you_have();
            let card_number = card.number as usize;
            let factor = factors[card_number - 1];
            for i in card_number..(card_number + score) {
                if i < factors.len() {
                    factors[i] += factor;
                }
            }
        }

        let result: u32 = factors.iter().sum();
        return Some(format!("{result}"));
    }
}

fn parse_line(line: &str) -> Option<Card> {
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    let mut line_split = line.split(':');
    let label_part = line_split.next()?;
    let card_number: u32 = label_part.split_whitespace().last()?.parse().ok()?;

    let numbers_part = line_split.next()?;
    let numbers_split = &mut numbers_part.split('|');
    let winning_numbers: Vec<i32> = numbers_split
        .next()?
        .split_whitespace()
        .map(|e| e.parse().ok())
        .collect::<Option<Vec<_>>>()?;

    let numbers_you_have: Vec<i32> = numbers_split
        .next()?
        .split_whitespace()
        .map(|e| e.parse().ok())
        .collect::<Option<Vec<_>>>()?;

    return Some(Card {
        number: card_number,
        winning_numbers,
        numbers_you_have,
    });
}

struct Card {
    number: u32,
    winning_numbers: Vec<i32>,
    numbers_you_have: Vec<i32>,
}

impl Card {
    fn compute_points(&self) -> u32 {
        return match self.count_winning_number_you_have() {
            0 => 0,
            n => 2_u32.pow((n as u32) - 1),
        };
    }

    fn count_winning_number_you_have(&self) -> usize {
        return self
            .numbers_you_have
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();
    }
}
