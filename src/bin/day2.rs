//! Day 2: Cube Conundrum

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    Finish, IResult, Parser,
};
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Cubes {
    blue: u32,
    green: u32,
    red: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Cubes>,
}

impl Game {
    fn is_possible(&self, available_cubes: &Cubes) -> bool {
        self.sets.iter().all(|cubes| {
            cubes.blue <= available_cubes.blue
                && cubes.green <= available_cubes.green
                && cubes.red <= available_cubes.red
        })
    }
}

impl FromStr for Game {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_game(s).finish() {
            Ok((_, game)) => Ok(game),
            Err(nom::error::Error { code, input }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    preceded(
        tag("Game "),
        tuple((
            nom::character::complete::u32,
            preceded(tag(":"), separated_list1(tag(";"), parse_set)),
        )),
    )
    .map(|(id, sets)| Game { id, sets })
    .parse(input)
}

fn parse_set(input: &str) -> IResult<&str, Cubes> {
    separated_list1(
        tag(","),
        delimited(
            space0,
            separated_pair(
                nom::character::complete::u32,
                tag(" "),
                alt((tag("blue"), tag("green"), tag("red"))),
            ),
            space0,
        ),
    )
    .map(|values| {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        values.into_iter().for_each(|(n, color)| match color {
            "blue" => blue += n,
            "green" => green += n,
            "red" => red += n,
            _ => unreachable!(),
        });
        Cubes { blue, green, red }
    })
    .parse(input)
}

fn solve(input: &str, available_cubes: Cubes) -> u32 {
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse games.");
    games
        .into_iter()
        .filter_map(|game| {
            if game.is_possible(&available_cubes) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn main() {
    let problem = fs::read_to_string("src/bin/day2.txt").expect("Failed to read file.");
    println!(
        "{}",
        solve(
            &problem,
            Cubes {
                blue: 14,
                green: 13,
                red: 12
            }
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            solve(
                example,
                Cubes {
                    blue: 14,
                    green: 13,
                    red: 12
                }
            ),
            8
        );
    }
}
