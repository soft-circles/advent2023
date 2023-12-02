use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "puzzle.pest"]
pub struct PuzzleParser;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

impl Color {
    pub fn from_str(input: &str) -> Self {
        match input {
            "red" => Self::RED,
            "green" => Self::GREEN,
            "blue" => Self::BLUE,
            _ => panic!("Invalid color {}", input),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Cube {
    color: Color,
    amount: u32,
}

impl Cube {
    pub fn new(color_input: &str, amount: u32) -> Self {
        match color_input {
            "red" => Self {
                color: Color::RED,
                amount,
            },
            "green" => Self {
                color: Color::GREEN,
                amount,
            },
            "blue" => Self {
                color: Color::BLUE,
                amount,
            },
            _ => panic!("invalid color: {}", color_input),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct ValidSet {
    blue: Cube,
    red: Cube,
    green: Cube,
}

impl ValidSet {
    pub fn new(blue_amount: u32, red_amount: u32, green_amount: u32) -> Self {
        Self {
            blue: Cube::new("blue", blue_amount),
            red: Cube::new("red", red_amount),
            green: Cube::new("green", green_amount),
        }
    }
}

fn is_valid_set(set: &ValidSet, rule_set: &ValidSet) -> bool {
    set.blue.amount <= rule_set.blue.amount
        && set.red.amount <= rule_set.red.amount
        && set.green.amount <= rule_set.green.amount
}

fn get_valid_sets(game: &Game) -> Vec<ValidSet> {
    let mut valid_sets: Vec<ValidSet> = vec![];
    for set in game.sets.iter() {
        // 0: BLUE, 1: RED, 2: GREEN
        let mut amts = [0, 0, 0];
        for cube in set {
            match cube.color {
                Color::RED => {
                    amts[1] = cube.amount;
                }
                Color::GREEN => {
                    amts[2] = cube.amount;
                }
                Color::BLUE => {
                    amts[0] = cube.amount;
                }
            }
        }
        valid_sets.push(ValidSet::new(amts[0], amts[1], amts[2]));
        for val in amts.iter_mut() {
            *val = 0;
        }
    }
    valid_sets
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Vec<Cube>>,
}

fn parse_games(pairs: Pair<'_, Rule>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    let mut id: u32 = 0;
    let mut sets: Vec<Vec<Cube>> = vec![];
    for record in pairs.into_inner() {
        match record.as_rule() {
            Rule::game => {
                for game in record.into_inner() {
                    match game.as_rule() {
                        Rule::set => {
                            let mut new_set: Vec<Cube> = vec![];
                            for cube in game.into_inner() {
                                let mut cube_struct: Cube = Cube {
                                    color: Color::RED,
                                    amount: 0,
                                };
                                for cube_props in cube.into_inner() {
                                    match cube_props.as_rule() {
                                        Rule::color => {
                                            cube_struct.color =
                                                Color::from_str(cube_props.as_str());
                                        }
                                        Rule::amount => {
                                            cube_struct.amount =
                                                cube_props.as_str().parse().unwrap();
                                        }
                                        _ => (),
                                    }
                                }
                                new_set.push(cube_struct);
                            }
                            sets.push(new_set);
                        }
                        Rule::id => {
                            id = game.as_str().parse().expect("Failed to parse");
                        }
                        _ => (),
                    }
                }
                games.push(Game {
                    id,
                    sets: sets.clone(),
                });
                id = 0;
                sets = vec![];
            }
            _ => (),
        }
    }
    games
}

const RULE_SET_PART1: ValidSet = ValidSet {
    red: Cube {
        color: Color::RED,
        amount: 12,
    },
    green: Cube {
        color: Color::GREEN,
        amount: 13,
    },
    blue: Cube {
        color: Color::BLUE,
        amount: 14,
    },
};

pub fn solution1(input: &str) -> u32 {
    let parsed_file = PuzzleParser::parse(Rule::file, input);
    let games = parse_games(parsed_file.unwrap().next().unwrap());

    let mut id_total = 0;
    for game in games.iter() {
        if get_valid_sets(game)
            .iter()
            .all(|set| is_valid_set(set, &RULE_SET_PART1))
        {
            id_total += game.id;
        }
    }
    id_total
}

// Solution 2

fn get_max_from_sets(sets: &Vec<ValidSet>) -> ValidSet {
    // 0: BLUE, 1: RED, 2: GREEN
    let mut maxes = [0, 0, 0];
    for set in sets.iter() {
        match maxes[0].lt(&set.blue.amount) {
            true => maxes[0] = set.blue.amount,
            false => (),
        }
        match maxes[1].lt(&set.red.amount) {
            true => maxes[1] = set.red.amount,
            false => (),
        }
        match maxes[2].lt(&set.green.amount) {
            true => maxes[2] = set.green.amount,
            false => (),
        }
    }
    ValidSet::new(maxes[0], maxes[1], maxes[2])
}

fn get_power_of_set(set: &ValidSet) -> u32 {
    set.blue.amount * set.red.amount * set.green.amount
}

pub fn solution2(input: &str) -> u32 {
    let parsed_file = PuzzleParser::parse(Rule::file, input);
    let games = parse_games(parsed_file.unwrap().next().unwrap());

    let mut cube_powers = 0;
    for game in games.iter() {
        let valid_sets = get_valid_sets(game);
        let maxes_in_set = get_max_from_sets(&valid_sets);
        cube_powers += get_power_of_set(&maxes_in_set);
    }
    cube_powers
}

#[cfg(test)]
mod test {

    use super::*;

    fn sample_input() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
    }

    #[test]
    fn correctly_parses() {
        assert_eq!(
            PuzzleParser::parse(Rule::file, sample_input()).is_ok(),
            true
        );
    }

    fn create_valid_set() -> ValidSet {
        ValidSet::new(14, 12, 13)
    }

    #[test]
    fn is_valid_set_works() {
        let set = ValidSet::new(3, 4, 0);
        assert_eq!(is_valid_set(&set, &create_valid_set()), true);
    }

    #[test]
    fn is_valid_set_works2() {
        let set = ValidSet::new(15, 14, 3);
        assert_eq!(is_valid_set(&set, &create_valid_set()), false);
    }

    #[test]
    fn parse_games_works() {
        let parsed = PuzzleParser::parse(Rule::file, sample_input())
            .expect("Could not parse input")
            .next()
            .unwrap();
        assert_eq!(parse_games(parsed).len(), 5);
    }

    #[test]
    fn parse_games_works2() {
        let parsed = PuzzleParser::parse(Rule::file, sample_input())
            .expect("Could not parse input")
            .next()
            .unwrap();
        assert_eq!(
            *parse_games(parsed).first().unwrap(),
            Game {
                id: 1,
                sets: vec![
                    vec![
                        Cube {
                            amount: 3,
                            color: Color::BLUE
                        },
                        Cube {
                            amount: 4,
                            color: Color::RED
                        }
                    ],
                    vec![
                        Cube {
                            amount: 1,
                            color: Color::RED
                        },
                        Cube {
                            amount: 2,
                            color: Color::GREEN
                        },
                        Cube {
                            amount: 6,
                            color: Color::BLUE
                        },
                    ],
                    vec![Cube {
                        amount: 2,
                        color: Color::GREEN
                    },]
                ]
            }
        );
    }

    #[test]
    fn get_valid_sets_works() {
        let game = Game {
            id: 1,
            sets: vec![
                vec![
                    Cube {
                        color: Color::RED,
                        amount: 3,
                    },
                    Cube {
                        color: Color::GREEN,
                        amount: 5,
                    },
                    Cube {
                        color: Color::BLUE,
                        amount: 2,
                    },
                ],
                vec![
                    Cube {
                        color: Color::RED,
                        amount: 1,
                    },
                    Cube {
                        color: Color::GREEN,
                        amount: 2,
                    },
                ],
            ],
        };

        let expected = vec![
            ValidSet {
                blue: Cube {
                    color: Color::BLUE,
                    amount: 2,
                },
                green: Cube {
                    color: Color::GREEN,
                    amount: 5,
                },
                red: Cube {
                    color: Color::RED,
                    amount: 3,
                },
            },
            ValidSet {
                red: Cube {
                    color: Color::RED,
                    amount: 1,
                },
                green: Cube {
                    color: Color::GREEN,
                    amount: 2,
                },
                blue: Cube {
                    color: Color::BLUE,
                    amount: 0,
                },
            },
        ];

        assert_eq!(get_valid_sets(&game), expected);
    }

    #[test]
    fn solution1_works() {
        assert_eq!(solution1(sample_input()), 8);
    }

    #[test]
    fn get_max_from_sets_works() {
        let game = Game {
            id: 1,
            sets: vec![
                vec![
                    Cube {
                        color: Color::RED,
                        amount: 3,
                    },
                    Cube {
                        color: Color::GREEN,
                        amount: 5,
                    },
                    Cube {
                        color: Color::BLUE,
                        amount: 2,
                    },
                ],
                vec![
                    Cube {
                        color: Color::RED,
                        amount: 1,
                    },
                    Cube {
                        color: Color::GREEN,
                        amount: 2,
                    },
                ],
            ],
        };
        assert_eq!(
            get_max_from_sets(&get_valid_sets(&game)),
            ValidSet::new(2, 3, 5)
        );
    }

    #[test]
    fn get_power_works() {
        assert_eq!(get_power_of_set(&ValidSet::new(6, 4, 2)), 48);
    }

    #[test]
    fn solution2_works() {
        assert_eq!(solution2(sample_input()), 2286);
    }
}
