/// --- Day 3: Gear Ratios ---
///
/// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you
/// up to the water source, but this is as far as he can bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I
/// wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before
/// I can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the engine, but nobody can
/// figure out which one. If you can add up all the part numbers in the engine schematic, it should
/// be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation of the engine.
/// There are lots of numbers and symbols you don't really understand, but apparently any number
/// adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum.
/// (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
///
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
///
/// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol:
/// 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a
/// part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger. What is the sum of all of the part
/// numbers in the engine schematic?
///
/// --- Part Two ---
///
/// The engineer finds the missing part and installs it in the engine! As the engine springs to
/// life, you jump in the closest gondola, finally ready to ascend to the water source.
///
/// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the
/// gondola has a phone labeled "help", so you pick it up and the engineer answers.
///
/// Before you can explain the situation, she suggests that you look out the window. There stands
/// the engineer, holding a phone in one hand and waving with the other. You're going so slowly that
/// you haven't even left the station. You exit the gondola.
///
/// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any
/// * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of
/// multiplying those two numbers together.
///
/// This time, you need to find the gear ratio of every gear and add them all up so that the
/// engineer can figure out which gear needs to be replaced.
///
/// Consider the same engine schematic again:
///
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
///
/// In this schematic, there are two gears. The first is in the top left; it has part numbers 467
/// and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is
/// 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.)
/// Adding up all of the gear ratios produces 467835.
///
/// What is the sum of all of the gear ratios in your engine schematic?
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_03");

pub fn run() {
    let schematic = parse_schematic(INPUT);

    let parts = map_parts(&schematic);

    let sum_of_part_numbers: u32 = parts.iter().flat_map(Part::get_numbers).sum();
    println!(
        "The sum of all the part numbers in the engine schematic is: {}",
        sum_of_part_numbers
    );
}

type Point = (i32, i32);
type Schematic = HashMap<Point, String>;

#[derive(Debug)]
struct Part {
    #[allow(dead_code)]
    position: Point,
    identity: String,
    numbers: Vec<u32>,
}

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.identity == other.identity
            && self.numbers.len() == other.numbers.len()
            && self.numbers.iter().all(|num| other.numbers.contains(num))
    }
}

impl Part {
    fn new(position: Point, identity: String) -> Part {
        Part {
            position,
            identity,
            numbers: Vec::new(),
        }
    }

    fn get_numbers(&self) -> &Vec<u32> {
        &self.numbers
    }
}

fn parse_schematic(input: &str) -> Schematic {
    let mut schematic = Schematic::new();
    let mut current_number = None;
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                current_number = match current_number {
                    None => Some(((x as i32, y as i32), ch.to_string())),
                    Some((pos, mut s)) => {
                        s.push(ch);
                        Some((pos, s))
                    }
                };
            } else {
                match current_number {
                    Some((pos, s)) => {
                        schematic.insert(pos, s);
                        current_number = None;
                    }
                    None => {}
                }
                if ch != '.' {
                    schematic.insert((x as i32, y as i32), ch.to_string());
                }
            }
        }
        match current_number {
            Some((pos, s)) => {
                schematic.insert(pos, s);
                current_number = None;
            }
            None => {}
        }
    }

    schematic
}

fn map_parts(schematic: &Schematic) -> Vec<Part> {
    // split the schematic into numbers and parts
    let mut numbers = Vec::new();
    let mut parts = HashMap::new();
    for (&point, s) in schematic.iter() {
        match s.parse() {
            Ok(number) => {
                // if the string is parseable into a number, add it to the numbers
                numbers.push((point, s, number));
            }
            Err(..) => {
                // else add it to the parts
                parts.insert(point, Part::new(point, s.clone()));
            }
        }
    }

    // map numbers to parts
    for ((x_n, y_n), s, number) in numbers.into_iter() {
        // iterate over all possible positions a part can
        for x in (x_n - 1)..=(x_n + 1) {
            for y in (y_n - 1)..=(y_n + s.len() as i32) {
                if let Some(part) = parts.get_mut(&(x, y)) {
                    // put the part number into the associated part's list
                    part.numbers.push(number);
                }
            }
        }
    }

    return parts.into_values().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_schematic() -> Schematic {
        let mut expected_schematic = Schematic::new();
        expected_schematic.insert((0, 0), String::from("467"));
        expected_schematic.insert((0, 5), String::from("114"));
        expected_schematic.insert((1, 3), String::from("*"));
        expected_schematic.insert((2, 2), String::from("35"));
        expected_schematic.insert((2, 6), String::from("633"));
        expected_schematic.insert((3, 6), String::from("#"));
        expected_schematic.insert((4, 0), String::from("617"));
        expected_schematic.insert((4, 3), String::from("*"));
        expected_schematic.insert((5, 5), String::from("+"));
        expected_schematic.insert((5, 7), String::from("58"));
        expected_schematic.insert((6, 2), String::from("592"));
        expected_schematic.insert((7, 6), String::from("755"));
        expected_schematic.insert((8, 3), String::from("$"));
        expected_schematic.insert((8, 5), String::from("*"));
        expected_schematic.insert((9, 1), String::from("664"));
        expected_schematic.insert((9, 5), String::from("598"));

        expected_schematic
    }

    fn example_parts() -> Vec<Part> {
        vec![
            Part {
                position: (1, 3),
                identity: "*".to_string(),
                numbers: vec![35, 467],
            },
            Part {
                position: (3, 6),
                identity: "#".to_string(),
                numbers: vec![633],
            },
            Part {
                position: (4, 3),
                identity: "*".to_string(),
                numbers: vec![617],
            },
            Part {
                position: (5, 5),
                identity: "+".to_string(),
                numbers: vec![592],
            },
            Part {
                position: (8, 3),
                identity: "$".to_string(),
                numbers: vec![664],
            },
            Part {
                position: (8, 5),
                identity: "*".to_string(),
                numbers: vec![598, 755],
            },
        ]
    }

    #[test]
    fn test_parse_schematic_1() {
        let input = "\
           467..114..\n\
           ...*......\n\
           ..35..633.\n\
           ......#...\n\
           617*......\n\
           .....+.58.\n\
           ..592.....\n\
           ......755.\n\
           ...$.*....\n\
           .664.598..";

        assert_eq!(parse_schematic(input), example_schematic());
    }

    #[test]
    fn test_parse_schematic_2() {
        let input = "1";

        let mut expected_schematic = Schematic::new();
        expected_schematic.insert((0, 0), String::from("1"));

        assert_eq!(parse_schematic(input), expected_schematic);
    }

    #[test]
    fn test_map_parts() {
        let actual_parts = map_parts(&example_schematic());
        let expected_parts = example_parts();

        // check whether the lengths of the parts list are equal
        assert_eq!(actual_parts.len(), example_parts().len());
        // check if all the parts in the actual parts are in the expected parts
        assert!(actual_parts
            .iter()
            .all(|part| expected_parts.contains(part)));
    }
}
