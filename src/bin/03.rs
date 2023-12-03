use std::{
    collections::{HashMap, HashSet},
    vec,
};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<u32>> = vec![];
    let mut number_index = 1;
    let mut number_hashmap = HashMap::new();
    for line in input.lines() {
        let mut row: Vec<u32> = vec![];
        let mut keep_going = false;
        let mut temp_number = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                keep_going = true;
                row.push(number_index);
                temp_number = temp_number * 10 + c.to_digit(10).unwrap();
            } else {
                if keep_going {
                    keep_going = false;
                    number_hashmap.insert(number_index, temp_number);
                    number_index += 1;
                    temp_number = 0;
                }
                if c == '.' {
                    row.push(0);
                } else {
                    row.push(u32::MAX);
                }
            }
        }
        if keep_going {
            number_hashmap.insert(number_index, temp_number);
            number_index += 1;
        }
        map.push(row);
    }
    // println!("{:?}", map);
    // println!("{:?}", number_hashmap);
    let mut sum = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            // println!("{} {}", i, j);
            if map[i][j] == u32::MAX {
                if i + 1 != map.len() {
                    sum.insert(map[i + 1][j]);
                    if j != 0 {
                        sum.insert(map[i + 1][j - 1]);
                    }
                    if j != map[i].len() {
                        sum.insert(map[i + 1][j + 1]);
                    }
                }
                if i != 0 {
                    sum.insert(map[i - 1][j]);
                    if j != 0 {
                        sum.insert(map[i - 1][j - 1]);
                    }
                    if j + 1 != map[i].len() {
                        sum.insert(map[i - 1][j + 1]);
                    }
                }
                if j != 0 {
                    sum.insert(map[i][j - 1]);
                }
                if j + 1 != map[i].len() {
                    sum.insert(map[i][j + 1]);
                }
            }

            // println!("{:?}", sum);
        }
    }
    let mut real_sum = 0;
    //println!("{:?}", number_hashmap);
    for i in sum.iter() {
        if let Some(x) = number_hashmap.get(i) {
            real_sum += x
        }
    }
    // println!("{:?}", sum);
    // println!("{:?}", real_sum);
    Some(real_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<u32>> = vec![];
    let mut number_index = 1;
    let mut number_hashmap = HashMap::new();
    for line in input.lines() {
        let mut row: Vec<u32> = vec![];
        let mut keep_going = false;
        let mut temp_number = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                keep_going = true;
                row.push(number_index);
                temp_number = temp_number * 10 + c.to_digit(10).unwrap();
            } else {
                if keep_going {
                    keep_going = false;
                    number_hashmap.insert(number_index, temp_number);
                    number_index += 1;
                    temp_number = 0;
                }
                if c == '.' {
                    row.push(0);
                } else if c == '*' {
                    row.push(u32::MAX);
                } else {
                    row.push(0);
                }
            }
        }
        if keep_going {
            number_hashmap.insert(number_index, temp_number);
            number_index += 1;
        }
        map.push(row);
    }
    // println!("{:?}", map);
    // println!("{:?}", number_hashmap);
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            //  println!("{} {}", i, j);
            let mut adjent = HashSet::new();
            if map[i][j] == u32::MAX {
                if i + 1 != map.len() {
                    adjent.insert(map[i + 1][j]);
                    if j != 0 {
                        adjent.insert(map[i + 1][j - 1]);
                    }
                    if j != map[i].len() {
                        adjent.insert(map[i + 1][j + 1]);
                    }
                }
                if i != 0 {
                    adjent.insert(map[i - 1][j]);
                    if j != 0 {
                        adjent.insert(map[i - 1][j - 1]);
                    }
                    if j + 1 != map[i].len() {
                        adjent.insert(map[i - 1][j + 1]);
                    }
                }
                if j != 0 {
                    adjent.insert(map[i][j - 1]);
                }
                if j + 1 != map[i].len() {
                    adjent.insert(map[i][j + 1]);
                }
            }
            adjent.remove(&0);
            // println!("{:?}", number_hashmap);
            //  println!("{:?}", adjent);
            let mut mul_adjent = 1;
            if adjent.len() == 2 {
                for i in adjent.iter() {
                    if let Some(x) = number_hashmap.get(i) {
                        mul_adjent *= x
                    }
                }
                sum += mul_adjent;
            }

            //println!("{:?}", sum);
        }
    }
    // let mut real_sum = 0;
    // println!("{:?}", number_hashmap);
    // for i in sum.iter() {
    //     if let Some(x) = number_hashmap.get(i) {
    //         real_sum += x
    //     }
    // }
    //println!("{:?}", sum);
    // println!("{:?}", real_sum);
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
