/// --- Day 2: Cube Conundrum ---
///
/// You're launched high into the atmosphere! The apex of your trajectory just barely reaches the
/// surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's
/// quite cold, but you don't see much snow. An Elf runs over to greet you.
///
/// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll
/// be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't
/// get many visitors up here; would you like to play a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
/// Each time you play this game, he will hide a secret number of cubes of each color in the bag,
/// and your goal is to figure out information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab
/// a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a
/// few times per game.
///
/// You play several games and record the information from each game (your puzzle input). Each game
/// is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated
/// list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
///
/// For example, the record of a few games might look like this:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first
/// set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue
/// cubes; the third set is only 2 green cubes.
///
/// The Elf would first like to know which games would have been possible if the bag contained only
/// 12 red cubes, 13 green cubes, and 14 blue cubes?
///
/// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with
/// that configuration. However, game 3 would have been impossible because at one point the Elf
/// showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the
/// Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been
/// possible, you get 8.
///
/// Determine which games would have been possible if the bag had been loaded with only 12 red
/// cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
///
/// --- Part Two ---
///
/// The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure
/// why the water stopped; however, he can show you how to get to the water source to check it out
/// for yourself. It's just up ahead!
///
/// As you continue your walk, the Elf poses a second question: in each game you played, what is the
/// fewest number of cubes of each color that could have been in the bag to make the game possible?
///
/// Again consider the example games from earlier:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
///  -  In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
///     If any color had even one fewer cube, the game would have been impossible.
///  -  Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
///  -  Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
///  -  Game 4 required at least 14 red, 3 green, and 15 blue cubes.
///  -  Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
///
/// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied
/// together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560,
/// 630, and 36, respectively. Adding up these five powers produces the sum 2286.
///
/// For each game, find the minimum set of cubes that must have been present. What is the sum of the
/// power of these sets?
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::IResult;
use std::num::ParseIntError;

const INPUT: &str = include_str!("../input/day_02");

pub fn run() {
    let games: Vec<Game> = INPUT
        .lines()
        .filter_map(|line| match Game::parse(line) {
            Ok((_, game)) => Some(game),
            _ => None,
        })
        .collect();

    let possible_game_id_sum: u32 = games
        .iter()
        .filter_map(|game| {
            match game.legal_game() {
                true => Some(game.id), // add legal games
                false => None,         // ignore illegal games
            }
        })
        .sum();

    println!(
        "The sum of the games that are possible is: {}",
        possible_game_id_sum
    );
}

#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Round {
    fn parse(input: &str) -> IResult<&str, Round> {
        let extract_round = separated_list0(
            tag(", "),
            pair(digit1, alt((tag(" red"), tag(" green"), tag(" blue")))),
        );

        fn convert_to_round(cubes: Vec<(&str, &str)>) -> Result<Round, ParseIntError> {
            let mut round = Round {
                red: 0,
                green: 0,
                blue: 0,
            };

            for (a, colour) in cubes {
                let amount = a.parse()?;

                if colour == " red" {
                    round.red = amount;
                }
                if colour == " green" {
                    round.green = amount;
                }
                if colour == " blue" {
                    round.blue = amount;
                }
            }

            return Ok(round);
        }

        map_res(extract_round, convert_to_round)(input)
    }
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Game> {
        let extract_values = pair(
            delimited(tag("Game "), digit1, tag(": ")),
            separated_list0(tag("; "), Round::parse),
        );

        fn convert_to_game((number, rounds): (&str, Vec<Round>)) -> Result<Game, ParseIntError> {
            Ok(Game {
                id: number.parse()?,
                rounds,
            })
        }

        map_res(extract_values, convert_to_game)(input)
    }

    fn legal_game(&self) -> bool {
        // return true if none of the rounds has amounts above the expected amounts
        self.rounds
            .iter()
            .find(|round| round.red > 12 || round.green > 13 || round.blue > 14)
            .is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_game_1() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let expected_game = Game {
            id: 1,
            rounds: vec![
                Round {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Round {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };

        assert_eq!(Game::parse(line), Ok(("", expected_game)));
    }

    #[test]
    fn test_convert_to_game_2() {
        let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected_game = Game {
            id: 2,
            rounds: vec![
                Round {
                    red: 0,
                    green: 2,
                    blue: 1,
                },
                Round {
                    red: 1,
                    green: 3,
                    blue: 4,
                },
                Round {
                    red: 0,
                    green: 1,
                    blue: 1,
                },
            ],
        };

        assert_eq!(Game::parse(line), Ok(("", expected_game)));
    }

    #[test]
    fn test_convert_to_game_3() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let expected_game = Game {
            id: 3,
            rounds: vec![
                Round {
                    red: 20,
                    green: 8,
                    blue: 6,
                },
                Round {
                    red: 4,
                    green: 13,
                    blue: 5,
                },
                Round {
                    red: 1,
                    green: 5,
                    blue: 0,
                },
            ],
        };

        assert_eq!(Game::parse(line), Ok(("", expected_game)));
    }

    #[test]
    fn test_convert_to_game_4() {
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let expected_game = Game {
            id: 4,
            rounds: vec![
                Round {
                    red: 3,
                    green: 1,
                    blue: 6,
                },
                Round {
                    red: 6,
                    green: 3,
                    blue: 0,
                },
                Round {
                    red: 14,
                    green: 3,
                    blue: 15,
                },
            ],
        };

        assert_eq!(Game::parse(line), Ok(("", expected_game)));
    }

    #[test]
    fn test_convert_to_game_5() {
        let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_game = Game {
            id: 5,
            rounds: vec![
                Round {
                    red: 6,
                    green: 3,
                    blue: 1,
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 2,
                },
            ],
        };

        assert_eq!(Game::parse(line), Ok(("", expected_game)));
    }
}
