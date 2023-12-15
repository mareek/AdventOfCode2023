pub struct Day15;

impl crate::day_trait::DaySolver for Day15 {
    fn day_of_month(&self) -> i32 {
        return 15;
    }

    fn solve_first_problem(&self, file_content: &str) -> Option<String> {
        let mut result = 0;
        for instructions in parse_instructions_1(file_content) {
            result += hash_instructions(instructions);
        }
        return Some(format!("{result}"));
    }

    fn solve_second_problem(&self, file_content: &str) -> Option<String> {
        let mut boxes: Vec<Vec<Instruction>> = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Vec::new());
        }

        let instructions = parse_instructions_2(file_content);
        for instruction in instructions.iter() {
            let box_index = hash_instructions(instruction.label.chars().collect());
            let boite = &mut boxes[box_index as usize];
            let existing_lens_index = boite.iter().position(|i| i.label == instruction.label);
            match (instruction.focal_length, existing_lens_index) {
                (None, None) => {}
                (None, Some(index)) => {
                    boite.remove(index);
                }
                (Some(_), None) => {
                    boite.push(instruction.clone());
                }
                (Some(_), Some(index)) => {
                    boite[index] = instruction.clone();
                }
            }
        }

        /* One plus the box number of the lens in question.
        The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
        The focal length of the lens.*/
        let mut result: u32 = 0;
        for i in 0..boxes.len() {
            let boite = &boxes[i];
            let box_position = 1 + i as u32;
            for j in 0..boite.len() {
                let lens_position = 1 + j as u32;
                let lens = &boite[j];
                result += box_position * lens_position * lens.focal_length?
            }
        }

        return Some(format!("{result}"));
    }
}

#[derive(Clone)]
struct Instruction {
    label: String,
    focal_length: Option<u32>,
}

impl Instruction {
    fn parse(raw_instruction: &str) -> Option<Instruction> {
        if raw_instruction.contains('-') {
            let mut splited_instruction = raw_instruction.split('-');
            let label = String::from(splited_instruction.next()?);
            return Some(Instruction {
                label,
                focal_length: None,
            });
        } else if raw_instruction.contains('=') {
            let mut splited_instruction = raw_instruction.split('=');
            let label = String::from(splited_instruction.next()?);
            let focal_length = splited_instruction.next()?.parse::<u32>().ok();
            return Some(Instruction {
                label,
                focal_length,
            });
        } else {
            return None;
        }
    }
}

fn parse_instructions_1(file_content: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in file_content.lines() {
        for instruction in line.split(',') {
            result.push(instruction.chars().collect());
        }
    }
    return result;
}

fn parse_instructions_2(file_content: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in file_content.lines() {
        for raw_instruction in line.split(',') {
            match Instruction::parse(raw_instruction) {
                Some(instruction) => result.push(instruction),
                None => {}
            }
        }
    }
    return result;
}

fn hash_instructions(instructions: Vec<char>) -> u32 {
    /* Determine the ASCII code for the current character of the string.
    Increase the current value by the ASCII code you just determined.
    Set the current value to itself multiplied by 17.
    Set the current value to the remainder of dividing itself by 256. */
    let mut result = 0;
    for chr in instructions {
        let char_code = chr as u32;
        result += char_code;
        result *= 17;
        result %= 256;
    }
    return result;
}
