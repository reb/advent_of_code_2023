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
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_03");

pub fn run() {
    println!("Not implemented yet");
    unimplemented!();
}

type Point = (i32, i32);
type Schematic = HashMap<Point, String>;

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
    }

    schematic
}

fn find_part_numbers(schematic: &Schematic) -> Vec<u32> {
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
                parts.insert(point, s);
            }
        }
    }

    // iterate over the numbers and try to find parts that lie next to them
    numbers
        .into_iter()
        .filter_map(|((x_n, y_n), s, number)| {
            // iterate over all possible positions a part can
            for x in (x_n - 1)..=(x_n + 1) {
                for y in (y_n - 1)..=(y_n + s.len() as i32) {
                    if parts.get(&(x, y)).is_some() {
                        // if there is a part, this is a part number
                        return Some(number);
                    }
                }
            }
            // otherwise it is not, and should be discarded
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_schematic() -> Schematic {
        let mut expected_schematic = HashMap::new();
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

    #[test]
    fn test_parse_schematic() {
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
    fn test_find_part_numbers() {
        let expected_part_numbers = vec![35, 467, 592, 598, 617, 633, 664, 755];
        let mut actual_part_numbers = find_part_numbers(&example_schematic());
        // since the function doesn't have a requirement on the order of the return, sort it to
        // make comparisons easier
        actual_part_numbers.sort();

        assert_eq!(actual_part_numbers, expected_part_numbers);
    }
}
