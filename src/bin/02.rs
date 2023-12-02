use std::{collections::HashMap, hash::Hash};

use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.split('\n');
    let mut sum = 0;
    for game in games {
        let mut game_split = game.split(':');
        let re = Regex::new(r"(\d+)").unwrap();
        let answer: u32 = re
            .find(game_split.next().unwrap())
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let packs = game_split.next().unwrap().split(';');
        let mut game_is_valid = true;
        for pack in packs {
            let mut pack_is_valid = true;
            for cube in pack.split(',') {
                let mut cube_split = cube.split(' ');
                let _ = cube_split.next().unwrap();
                let count: u32 = cube_split.next().unwrap().parse().unwrap();
                let color = cube_split.next().unwrap();
                let valid = match color {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => false,
                };

                if !valid {
                    pack_is_valid = false;
                    break;
                } else {
                    continue;
                };
            }
            if !pack_is_valid {
                game_is_valid = false;
                break;
            } else {
                continue;
            }
        }

        if game_is_valid {
            sum += answer;
        } else {
            continue;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.split('\n');
    let mut sum = 0;
    for game in games {
        let mut game_split = game.split(':');
        let re = Regex::new(r"(\d+)").unwrap();
        let answer: u32 = re
            .find(game_split.next().unwrap())
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let packs = game_split.next().unwrap().split(';');
        let mut cube_hashmap: HashMap<&str, u32> = HashMap::new();

        for pack in packs {
            for cube in pack.split(',') {
                println!("hashmap:{:?}", cube_hashmap);
                let mut cube_split = cube.split(' ');
                let _ = cube_split.next().unwrap();
                let count: u32 = cube_split.next().unwrap().parse().unwrap();
                let color = cube_split.next().unwrap();
                let min_value = cube_hashmap.get(color);
                match min_value {
                    Some(min_value) => {
                        // println!("{:?}", min_value);
                    }
                    None => {
                        cube_hashmap.insert(color, count);
                        continue;
                    }
                }
                if *min_value.unwrap() < count {
                    cube_hashmap.insert(color, count);
                }
            }
        }

        let mut mul = 1;
        for (key, value) in cube_hashmap.iter() {
            //println!("{}{}", key, value);
            mul *= value;
        }
        sum += mul;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
