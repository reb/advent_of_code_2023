/// --- Day 1: Trebuchet?! ---
///
/// Something is wrong with global snow production, and you've been selected to take a look. The
/// Elves have even given you a map; on it, they've used stars to mark the top fifty locations that
/// are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations, you need to check
/// all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
/// calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one
/// star. Good luck!
///
/// You try to ask why they can't just use a weather machine ("not powerful enough") and where
/// they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of
/// questions") and hang on did you just say the sky ("of course, where do you think snow comes
/// from") when you realize that the Elves are already loading you into a trebuchet ("please hold
/// still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their calibration document (your
/// puzzle input) has been amended by a very young Elf who was apparently just excited to show off
/// her art skills. Consequently, the Elves are having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally
/// contained a specific calibration value that the Elves now need to recover. On each line, the
/// calibration value can be found by combining the first digit and the last digit (in that order)
/// to form a single two-digit number.
///
/// For example:
///
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
///
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these
/// together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the calibration values?
///
/// --- Part Two ---
///`
/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out
/// with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid
/// "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each
/// line. For example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these
/// together produces 281.
///
/// What is the sum of all of the calibration values?

const INPUT: &str = include_str!("../input/day_01");

pub fn run() {
    let calibration_value_sum: u32 = INPUT.lines().filter_map(extract_calibration_value).sum();

    println!(
        "The sum of all the calibration values is: {}",
        calibration_value_sum
    );

    let written_calibration_value_sum: u32 = INPUT
        .lines()
        .filter_map(extract_written_calibration_value)
        .sum();

    println!(
        "The sum of all the calibration values, including written ones, is: {}",
        written_calibration_value_sum
    );
}

fn extract_calibration_value(line: &str) -> Option<u32> {
    let first = line.chars().filter(|c| c >= &'0' && c <= &'9').next();
    let last = line.chars().rev().filter(|c| c >= &'0' && c <= &'9').next();

    Some(first?.to_digit(10)? * 10 + last?.to_digit(10)?)
}

fn convert_to_u32(i: usize, c: char, line: &str) -> Option<u32> {
    if c >= '0' && c <= '9' {
        return Some(c.to_digit(10)?);
    }
    if line[i..].starts_with("one") {
        return Some(1);
    }
    if line[i..].starts_with("two") {
        return Some(2);
    }
    if line[i..].starts_with("three") {
        return Some(3);
    }
    if line[i..].starts_with("four") {
        return Some(4);
    }
    if line[i..].starts_with("five") {
        return Some(5);
    }
    if line[i..].starts_with("six") {
        return Some(6);
    }
    if line[i..].starts_with("seven") {
        return Some(7);
    }
    if line[i..].starts_with("eight") {
        return Some(8);
    }
    if line[i..].starts_with("nine") {
        return Some(9);
    }
    None
}

fn extract_written_calibration_value(line: &str) -> Option<u32> {
    // also check for the occurrence of substrings at every position

    let first = line
        .char_indices()
        .filter_map(|(i, c)| convert_to_u32(i, c, line))
        .next();

    let last = line
        .char_indices()
        .rev()
        .filter_map(|(i, c)| convert_to_u32(i, c, line))
        .next();

    Some(first? * 10 + last?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_calibration_value_1() {
        // 1abc2
        assert_eq!(extract_calibration_value("1abc2"), Some(12));
    }

    #[test]
    fn test_extract_calibration_value_2() {
        // pqr3stu8vwx
        assert_eq!(extract_calibration_value("pqr3stu8vwx"), Some(38));
    }

    #[test]
    fn test_extract_calibration_value_3() {
        // a1b2c3d4e5f
        assert_eq!(extract_calibration_value("a1b2c3d4e5f"), Some(15));
    }

    #[test]
    fn test_extract_calibration_value_4() {
        // treb7uchet
        assert_eq!(extract_calibration_value("treb7uchet"), Some(77));
    }

    #[test]
    fn test_written_extract_calibration_value_1() {
        // two1nine
        assert_eq!(extract_written_calibration_value("two1nine"), Some(29));
    }

    #[test]
    fn test_written_extract_calibration_value_2() {
        // eightwothree
        assert_eq!(extract_written_calibration_value("eightwothree"), Some(83));
    }

    #[test]
    fn test_written_extract_calibration_value_3() {
        // abcone2threexyz
        assert_eq!(
            extract_written_calibration_value("abcone2threexyz"),
            Some(13)
        );
    }

    #[test]
    fn test_written_extract_calibration_value_4() {
        // xtwone3four
        assert_eq!(extract_written_calibration_value("xtwone3four"), Some(24));
    }

    #[test]
    fn test_written_extract_calibration_value_5() {
        // 4nineeightseven2
        assert_eq!(
            extract_written_calibration_value("4nineeightseven2"),
            Some(42)
        );
    }

    #[test]
    fn test_written_extract_calibration_value_6() {
        // zoneight234
        assert_eq!(extract_written_calibration_value("zoneight234"), Some(14));
    }

    #[test]
    fn test_written_extract_calibration_value_7() {
        // 7pqrstsixteen
        assert_eq!(extract_written_calibration_value("7pqrstsixteen"), Some(76));
    }
}
