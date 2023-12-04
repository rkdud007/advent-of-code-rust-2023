advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for card in input.lines() {
        let mut card_split = card.split(':');
        let card_info = card_split.next().unwrap();
        let mut card_info_split = card_info.split(' ');
        let _ = card_info_split.next().unwrap();
        let mut card_number: i32 = 0;
        for card_info_split_ele in card_info_split {
            if !card_info_split_ele.is_empty() {
                card_number = card_info_split_ele.parse().unwrap();
                break;
            }
        }
        let card_suit = card_split.next().unwrap();
        let mut card_suit_split = card_suit.split('|');
        let winning_card_suit = card_suit_split.next().unwrap();
        let mut winning_card_vec: Vec<i32> = Vec::new();
        for winning_card in winning_card_suit.split(' ') {
            if !winning_card.is_empty() {
                winning_card_vec.push(winning_card.parse().unwrap());
            }
        }
        let your_card_suit = card_suit_split.next().unwrap();
        let mut your_card_vec: Vec<i32> = Vec::new();
        let mut your_winning = 0;
        for your_card in your_card_suit.split(' ') {
            if !your_card.is_empty() {
                your_card_vec.push(your_card.parse().unwrap());
                let your_card_number: i32 = your_card.parse().unwrap();
                if winning_card_vec.contains(&your_card_number) {
                    println!(
                        "number {} suit {} your:{:?} winneing:{:?}",
                        card_number, card_suit, your_card_vec, winning_card_vec
                    );
                    your_winning += 1;
                }
            }
        }
        println!(
            "number {} suit {} your:{:?} winneing:{:?}",
            card_number, card_suit, your_card_vec, winning_card_vec
        );
        let base: u32 = 2;
        if your_winning > 0 {
            your_winning -= 1;
            sum += base.pow(your_winning as u32);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
