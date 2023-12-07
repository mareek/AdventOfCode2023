use std::{cmp::*, fmt::format};

pub struct Day7;

impl crate::day_trait::DaySolver for Day7 {
    fn day_of_month(&self) -> i32 {
        return 7;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let hands = parse_input(file_content, false)?;
        let mut ordered_hands = hands.clone();
        ordered_hands.sort();

        let mut result: usize = 0;

        for i in 0..ordered_hands.len() {
            let hand = &ordered_hands[i];
            result += hand.bid * (i + 1);
        }

        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let hands = parse_input(file_content, true)?;
        let mut ordered_hands = hands.clone();
        ordered_hands.sort();

        let mut result: usize = 0;

        for i in 0..ordered_hands.len() {
            let hand = &ordered_hands[i];
            result += hand.bid * (i + 1);
        }

        return Some(format!("{result}"));
    }
}

fn parse_input(file_content: &str, j_is_for_joker: bool) -> Option<Vec<Hand>> {
    return file_content
        .lines()
        .map(|line| parse_hand(line, j_is_for_joker))
        .collect::<Option<Vec<_>>>();
}

fn parse_hand(line: &str, j_is_for_joker: bool) -> Option<Hand> {
    let mut line_parts = line.split_whitespace();
    let cards: Vec<Card> = line_parts
        .next()?
        .chars()
        .map(|name| Card {
            name,
            j_is_for_joker,
        })
        .collect();
    let bid: usize = line_parts.next()?.parse().ok()?;
    return Some(Hand {
        cards,
        bid,
        j_is_for_joker,
    });
}

struct Card {
    name: char,
    j_is_for_joker: bool,
}

struct Hand {
    cards: Vec<Card>,
    bid: usize,
    j_is_for_joker: bool,
}

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn get_value(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl Card {
    fn get_value(&self) -> usize {
        return match self.name {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' if !self.j_is_for_joker => 11,
            'J' if self.j_is_for_joker => 1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!("Unkown card : '{}'", self.name),
        };
    }

    fn all_cards(j_is_for_joker: bool) -> [Card; 13] {
        return [
            Card {
                name: 'A',
                j_is_for_joker,
            },
            Card {
                name: 'K',
                j_is_for_joker,
            },
            Card {
                name: 'Q',
                j_is_for_joker,
            },
            Card {
                name: 'J',
                j_is_for_joker,
            },
            Card {
                name: 'T',
                j_is_for_joker,
            },
            Card {
                name: '9',
                j_is_for_joker,
            },
            Card {
                name: '8',
                j_is_for_joker,
            },
            Card {
                name: '7',
                j_is_for_joker,
            },
            Card {
                name: '6',
                j_is_for_joker,
            },
            Card {
                name: '5',
                j_is_for_joker,
            },
            Card {
                name: '4',
                j_is_for_joker,
            },
            Card {
                name: '3',
                j_is_for_joker,
            },
            Card {
                name: '2',
                j_is_for_joker,
            },
        ];
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.get_value().cmp(&other.get_value());
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.get_value().partial_cmp(&other.get_value());
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

impl Eq for Card {}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let card_groups = Card::all_cards(self.j_is_for_joker)
            .iter()
            .filter(|c| !(c.j_is_for_joker && c.name == 'J'))
            .map(|c| self.cards.iter().filter(|hc| *hc == c).count())
            .filter(|count| *count > 0)
            .collect::<Vec<_>>();

        let joker_count = self
            .cards
            .iter()
            .filter(|c| c.j_is_for_joker && c.name == 'J')
            .count();

        if joker_count == 0 {
            return match card_groups.len() {
                1 => HandType::FiveOfAKind,
                2 if card_groups.iter().any(|count| *count == 4) => HandType::FourOfAKind,
                2 => HandType::FullHouse,
                3 if card_groups.iter().any(|count| *count == 3) => HandType::ThreeOfAKind,
                3 => HandType::TwoPair,
                4 => HandType::OnePair,
                _ => HandType::HighCard,
            };
        } else {
            let max_group_length: &usize = card_groups.iter().max().unwrap_or_else(|| &0);
            return match max_group_length + joker_count {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 if card_groups.iter().filter(|c| **c >= 2).count() == 2 => HandType::FullHouse,
                3 => HandType::ThreeOfAKind,
                2 => HandType::OnePair,
                _ => HandType::HighCard,
            };
        }
    }

    fn get_cumuled_cards_score(&self) -> usize {
        return (self.cards[0].get_value() * 100000000)
            + (self.cards[1].get_value() * 1000000)
            + (self.cards[2].get_value() * 10000)
            + (self.cards[3].get_value() * 100)
            + (self.cards[4].get_value());
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self
            .get_hand_type()
            .get_value()
            .cmp(&other.get_hand_type().get_value());
        if hand_type_cmp == Ordering::Equal {
            return self
                .get_cumuled_cards_score()
                .cmp(&other.get_cumuled_cards_score());
        } else {
            return hand_type_cmp;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_type_cmp = self
            .get_hand_type()
            .get_value()
            .cmp(&other.get_hand_type().get_value());
        if hand_type_cmp == Ordering::Equal {
            return Some(
                self.get_cumuled_cards_score()
                    .cmp(&other.get_cumuled_cards_score()),
            );
        } else {
            return Some(hand_type_cmp);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.get_hand_type().get_value() == other.get_hand_type().get_value()
            && self.get_cumuled_cards_score() == other.get_cumuled_cards_score();
    }
}

impl Eq for Hand {}

impl Clone for Hand {
    fn clone(&self) -> Hand {
        return Hand {
            cards: self
                .cards
                .iter()
                .map(|c| Card {
                    name: c.name.clone(),
                    j_is_for_joker: self.j_is_for_joker,
                })
                .collect(),
            bid: self.bid,
            j_is_for_joker: self.j_is_for_joker,
        };
    }
}
