/// --- Day 5: If You Give A Seed A Fertilizer ---
///
/// You take the boat and find the gardener right where you were told he would be: managing a giant
/// "garden" that looks more to you like a farm.
///
/// "A water source? Island Island is the water source!" You point out that Snow Island isn't
/// receiving any water.
///
/// "Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with
/// dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few
/// days... weeks... oh no." His face sinks into a look of horrified realization.
///
/// "I've been so busy making sure everyone here has food that I completely forgot to check why we
/// stopped getting more sand! There's a ferry leaving soon that is headed over in that direction -
/// it's much faster than your boat. Could you please go check it out?"
///
/// You barely have time to agree to this request when he brings up another. "While you wait for the
/// ferry, maybe you can help us with our food production problem. The latest Island Island Almanac
/// just arrived and we're having trouble making sense of it."
///
/// The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists
/// what type of soil to use with each kind of seed, what type of fertilizer to use with each kind
/// of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed,
/// soil, fertilizer and so on is identified with a number, but numbers are reused by each category
/// - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.
///
/// For example:
///
/// seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
///
/// soil-to-fertilizer map:
/// 0 15 37
/// 37 52 2
/// 39 0 15
///
/// fertilizer-to-water map:
/// 49 53 8
/// 0 11 42
/// 42 0 7
/// 57 7 4
///
/// water-to-light map:
/// 88 18 7
/// 18 25 70
///
/// light-to-temperature map:
/// 45 77 23
/// 81 45 19
/// 68 64 13
///
/// temperature-to-humidity map:
/// 0 69 1
/// 1 0 69
///
/// humidity-to-location map:
/// 60 56 37
/// 56 93 4
///
/// The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.
///
/// The rest of the almanac contains a list of maps which describe how to convert numbers from a
/// source category into numbers in a destination category. That is, the section that starts with
/// seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the
/// destination). This lets the gardener and his team know which soil to use with which seeds, which
/// water to use with which fertilizer, and so on.
///
/// Rather than list every source number and its corresponding destination number one by one, the
/// maps describe entire ranges of numbers that can be converted. Each line within a map contains
/// three numbers: the destination range start, the source range start, and the range length.
///
/// Consider again the example seed-to-soil map:
///
/// 50 98 2
/// 52 50 48
///
/// The first line has a destination range start of 50, a source range start of 98, and a range
/// length of 2. This line means that the source range starts at 98 and contains two values: 98 and
/// 99. The destination range is the same length, but it starts at 50, so its two values are 50 and
/// 51. With this information, you know that seed number 98 corresponds to soil number 50 and that
/// seed number 99 corresponds to soil number 51.
///
/// The second line means that the source range starts at 50 and contains 48 values: 50, 51, ...,
/// 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values:
/// 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.
///
/// Any source numbers that aren't mapped correspond to the same destination number. So, seed number
/// 10 corresponds to soil number 10.
///
/// So, the entire list of seed numbers and their corresponding soil numbers looks like this:
///
/// seed  soil
/// 0     0
/// 1     1
/// ...   ...
/// 48    48
/// 49    49
/// 50    52
/// 51    53
/// ...   ...
/// 96    98
/// 97    99
/// 98    50
/// 99    51
///
/// With this map, you can look up the soil number required for each initial seed number:
///
///   - Seed number 79 corresponds to soil number 81.
///   - Seed number 14 corresponds to soil number 14.
///   - Seed number 55 corresponds to soil number 57.
///   - Seed number 13 corresponds to soil number 13.
///
/// The gardener and his team want to get started as soon as possible, so they'd like to know the
/// closest location that needs a seed. Using these maps, find the lowest location number that
/// corresponds to any of the initial seeds. To do this, you'll need to convert each seed number
/// through other categories until you can find its corresponding location number. In this example,
/// the corresponding types are:
///
///   - Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location
///     82.
///   - Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location
///     43.
///   - Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location
///     86.
///   - Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location
///     35.
///
/// So, the lowest location number in this example is 35.
///
/// What is the lowest location number that corresponds to any of the initial seed numbers?
///
/// --- Part Two ---
///
/// Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it
/// looks like the seeds: line actually describes ranges of seed numbers.
///
/// The values on the initial seeds: line come in pairs. Within each pair, the first value is the
/// start of the range and the second value is the length of the range. So, in the first line of the
/// example above:
///
/// seeds: 79 14 55 13
///
/// This line describes two ranges of seed numbers to be planted in the garden. The first range
/// starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts
/// with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.
///
/// Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.
///
/// In the above example, the lowest location number can be obtained from seed number 82, which
/// corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and
/// location 46. So, the lowest location number is 46.
///
/// Consider all of the initial seed numbers listed in the ranges on the first line of the almanac.
/// What is the lowest location number that corresponds to any of the initial seed numbers?
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::ops::Range;

const INPUT: &str = include_str!("../input/day_05");

pub fn run() {
    let (_, almanac) = Almanac::parse(INPUT).expect("parsing failed");

    let locations = almanac.get_locations();
    let lowest_location = locations
        .iter()
        .min()
        .expect("there was no lowest location");
    println!("The lowest location number is: {}", lowest_location);
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<Vec<AlmanacMapEntry>>,
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Almanac> {
        let (input, seeds) = delimited(
            tag("seeds: "),
            separated_list1(space1, complete::u64),
            tuple((newline, newline)),
        )(input)?;

        let seed_ranges = seeds
            .chunks_exact(2)
            .filter_map(|array| match array {
                &[start, size] => Some(start..start + size),
                _ => None,
            })
            .collect();

        let almanac_map = preceded(
            tuple((
                alt((
                    tag("seed-to-soil"),
                    tag("soil-to-fertilizer"),
                    tag("fertilizer-to-water"),
                    tag("water-to-light"),
                    tag("light-to-temperature"),
                    tag("temperature-to-humidity"),
                    tag("humidity-to-location"),
                )),
                tag(" map:"),
                newline,
            )),
            separated_list1(newline, AlmanacMapEntry::parse),
        );

        let (input, maps) = separated_list1(tuple((newline, newline)), almanac_map)(input)?;

        Ok((
            input,
            Almanac {
                seeds,
                seed_ranges,
                maps,
            },
        ))
    }

    fn get_locations(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| {
                // start with the seed number, and go through all the almanac maps in the list
                let mut n = *seed;
                for map in self.maps.iter() {
                    // look for a match in the entries of the almanac map
                    n = match map.iter().filter_map(|entry| entry.map(&n)).next() {
                        // either an entry is found in the almanac map
                        Some(mapped) => mapped,
                        // or it remains the same number
                        None => n,
                    }
                }
                return n;
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacMapEntry {
    destination: Range<u64>,
    source: Range<u64>,
}

impl AlmanacMapEntry {
    fn parse(input: &str) -> IResult<&str, AlmanacMapEntry> {
        let (input, (destination_start, source_start, range_size)) = tuple((
            complete::u64,
            preceded(space1, complete::u64),
            preceded(space1, complete::u64),
        ))(input)?;

        Ok((
            input,
            AlmanacMapEntry {
                destination: destination_start..destination_start + range_size,
                source: source_start..source_start + range_size,
            },
        ))
    }

    fn map(&self, n: &u64) -> Option<u64> {
        if !self.source.contains(n) {
            return None;
        }

        Some(self.destination.start + (n - self.source.start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_almanac_map_entry_parse_1() {
        let input = "50 98 2";

        let expected_entry = AlmanacMapEntry {
            destination: 50..52,
            source: 98..100,
        };

        let (remainder, actual_entry) = AlmanacMapEntry::parse(input).unwrap();

        assert_eq!(actual_entry, expected_entry);
        assert_eq!(remainder, "");
    }

    #[test]
    fn test_almanac_map_entry_parse_2() {
        let input = "0 69 1";

        let expected_entry = AlmanacMapEntry {
            destination: 0..1,
            source: 69..70,
        };

        let (remainder, actual_entry) = AlmanacMapEntry::parse(input).unwrap();

        assert_eq!(actual_entry, expected_entry);
        assert_eq!(remainder, "");
    }

    #[test]
    fn test_almanac_map_entry_parse_3() {
        let input = "1 0 69";

        let expected_entry = AlmanacMapEntry {
            destination: 1..70,
            source: 0..69,
        };

        let (remainder, actual_entry) = AlmanacMapEntry::parse(input).unwrap();

        assert_eq!(actual_entry, expected_entry);
        assert_eq!(remainder, "");
    }

    fn example_almanac() -> Almanac {
        Almanac {
            seeds: vec![79, 14, 55, 13],
            seed_ranges: vec![79..93, 55..68],
            maps: vec![
                vec![
                    AlmanacMapEntry {
                        destination: 50..52,
                        source: 98..100,
                    },
                    AlmanacMapEntry {
                        destination: 52..100,
                        source: 50..98,
                    },
                ],
                vec![
                    AlmanacMapEntry {
                        destination: 0..37,
                        source: 15..52,
                    },
                    AlmanacMapEntry {
                        destination: 37..39,
                        source: 52..54,
                    },
                    AlmanacMapEntry {
                        destination: 39..54,
                        source: 0..15,
                    },
                ],
                vec![
                    AlmanacMapEntry {
                        destination: 49..57,
                        source: 53..61,
                    },
                    AlmanacMapEntry {
                        destination: 0..42,
                        source: 11..53,
                    },
                    AlmanacMapEntry {
                        destination: 42..49,
                        source: 0..7,
                    },
                    AlmanacMapEntry {
                        destination: 57..61,
                        source: 7..11,
                    },
                ],
                vec![
                    AlmanacMapEntry {
                        destination: 88..95,
                        source: 18..25,
                    },
                    AlmanacMapEntry {
                        destination: 18..88,
                        source: 25..95,
                    },
                ],
                vec![
                    AlmanacMapEntry {
                        destination: 45..68,
                        source: 77..100,
                    },
                    AlmanacMapEntry {
                        destination: 81..100,
                        source: 45..64,
                    },
                    AlmanacMapEntry {
                        destination: 68..81,
                        source: 64..77,
                    },
                ],
                vec![
                    AlmanacMapEntry {
                        destination: 0..1,
                        source: 69..70,
                    },
                    AlmanacMapEntry {
                        destination: 1..70,
                        source: 0..69,
                    },
                ],
                vec![
                    AlmanacMapEntry {
                        destination: 60..97,
                        source: 56..93,
                    },
                    AlmanacMapEntry {
                        destination: 56..60,
                        source: 93..97,
                    },
                ],
            ],
        }
    }

    #[test]
    fn test_almanac_parse() {
        let input = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";

        assert_eq!(Almanac::parse(input), Ok(("", example_almanac())));
    }

    #[test]
    fn test_almanac_get_locations() {
        let expected_locations = vec![82, 43, 86, 35];

        assert_eq!(example_almanac().get_locations(), expected_locations);
    }
}
