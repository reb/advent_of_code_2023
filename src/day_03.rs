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
type Schematic = HashMap<Point, char>;

fn parse_schematic(input: &str) -> Schematic {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.char_indices()
                .filter(|(_, c)| c != &'.')
                .map(move |(y, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut expected_schematic = HashMap::new();
        expected_schematic.insert((0, 0), '4');
        expected_schematic.insert((0, 1), '6');
        expected_schematic.insert((0, 2), '7');
        expected_schematic.insert((0, 5), '1');
        expected_schematic.insert((0, 6), '1');
        expected_schematic.insert((0, 7), '4');
        expected_schematic.insert((1, 3), '*');
        expected_schematic.insert((2, 2), '3');
        expected_schematic.insert((2, 3), '5');
        expected_schematic.insert((2, 6), '6');
        expected_schematic.insert((2, 7), '3');
        expected_schematic.insert((2, 8), '3');
        expected_schematic.insert((3, 6), '#');
        expected_schematic.insert((4, 0), '6');
        expected_schematic.insert((4, 1), '1');
        expected_schematic.insert((4, 2), '7');
        expected_schematic.insert((4, 3), '*');
        expected_schematic.insert((5, 5), '+');
        expected_schematic.insert((5, 7), '5');
        expected_schematic.insert((5, 8), '8');
        expected_schematic.insert((6, 2), '5');
        expected_schematic.insert((6, 3), '9');
        expected_schematic.insert((6, 4), '2');
        expected_schematic.insert((7, 6), '7');
        expected_schematic.insert((7, 7), '5');
        expected_schematic.insert((7, 8), '5');
        expected_schematic.insert((8, 3), '$');
        expected_schematic.insert((8, 5), '*');
        expected_schematic.insert((9, 1), '6');
        expected_schematic.insert((9, 2), '6');
        expected_schematic.insert((9, 3), '4');
        expected_schematic.insert((9, 5), '5');
        expected_schematic.insert((9, 6), '9');
        expected_schematic.insert((9, 7), '8');

        assert_eq!(parse_schematic(input), expected_schematic);
    }
}
