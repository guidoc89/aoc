use std::collections::HashMap;


use crate::utils::{file, solution::{Solution, SolutionPair}};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Card {
    fn from_char(s: char) -> Option<Self> {
        match s {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    // HighCard(Card),
    HighCard,
    SinglePair,
    DoublePair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn from_hand(hand: Vec<Card>) -> (Self, Vec<Card>) {
        let hand_count = count_card_ranks(hand.clone());
        let mut counts: Vec<(Card, u32)> = hand_count.into_iter().collect();
        counts.sort_by(|a,b| {
            let count_cmp = b.1.cmp(&a.1);
            if count_cmp == std::cmp::Ordering::Equal {
                b.0.cmp(&a.0)
            } else {
                count_cmp
            }
        });

        if counts[0].1 == 5 {
            return (Self::FiveOfAKind, hand);

        } else if counts[0].1 == 4 {
            return (Self::FourOfAKind, hand);

        } else if counts[0].1 == 3 && counts[1].1 == 2 {
            return (Self::FullHouse, hand);

        } else if counts[0].1 == 3  {
            return (Self::ThreeOfAKind, hand);

        } else if counts[0].1 == 2 && counts[1].1 == 2 {
            return (Self::DoublePair, hand);

        } else if counts[0].1 == 2 {
            return (Self::SinglePair, hand);

        } else {
            let max_card = hand.clone().into_iter().min().unwrap();
            return (Self::HighCard, hand);
        }
    }
}

fn count_card_ranks(hand: Vec<Card>) -> HashMap<Card, u32> {
    let mut hand_count: HashMap<Card, u32> = HashMap::new();
    for card in &hand {
        hand_count.entry(*card).and_modify(|count| *count+=1).or_insert(1);
    }
    hand_count
}


fn strength(hand: &str) -> (u32, Vec<u32>) {
    let mut numeric_hand: Vec<u32> = hand.chars().map(|c| {
        if c == 'T' {10} 
        else if c == 'J' {1}
        else if c == 'Q' {12}
        else if c == 'K' {13}
        else if c == 'A' {14}
        else {c.to_digit(10).unwrap()}
    }).collect();
    let mut counter = HashMap::new();

    for n in &numeric_hand {
        counter.entry(n).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut sorted_values = counter.into_values().collect::<Vec<_>>();
    sorted_values.sort();

    if sorted_values == [5] {
        return (10u32, numeric_hand)
    } else if sorted_values == [1,4] {
        return (9u32, numeric_hand)
    } else if sorted_values == [2,3] {
        return (8u32, numeric_hand)
    } else if sorted_values == [1,1,3] {
        return (7u32, numeric_hand)
    } else if sorted_values == [1,2,2] {
        return (6u32, numeric_hand)
    } else if sorted_values == [1,1,1,2] {
        return (5u32, numeric_hand)
    } else {
        return (4u32, numeric_hand)
    }
}

fn part1(file_string: String) -> u32 {
     // solution: 250232501
    let file_lines_iter = file_string.split_terminator("\n");
    let mut hands = file_lines_iter.map(|line| {
        let parts: Vec<&str> =  line.split_whitespace().collect();
        let cards = parts[0].chars().map(|c| Card::from_char(c).unwrap()).collect::<Vec<_>>();
        let hand = Hand::from_hand(cards);
        let bid = parts[1].parse::<u32>().unwrap();
        (hand, bid)
    }).collect::<Vec<_>>();
    hands.sort_by(|a,b| {
        let rank_cmp = a.0.cmp(&b.0);
        if rank_cmp == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            rank_cmp
        }
    });
    println!("Hands: {:?}", hands);
    let mut ans: u32 = 0;
    for (idx, (hand, bid)) in hands.iter().enumerate() {
        let ranking = 1 + idx as u32;
        ans += bid * (ranking)

    }

    ans
}

fn part2(file_string: String) -> &'static str {
    "Not implemented"

}
pub fn solve(input_path: &str) -> SolutionPair{
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
