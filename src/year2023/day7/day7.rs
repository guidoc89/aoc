use std::collections::HashMap;
use crate::utils::{file, solution::{Solution, SolutionPair}};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Jack, // for part 2, move back to its original pos for part1
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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
    HighCard,
    SinglePair,
    DoublePair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


fn jacks_count(counts: &Vec<(Card, u32)>, idx: usize) -> usize {
    let jacks_slice = counts[idx..].iter().filter_map(|(card, _)| {
        if *card == Card::Jack {
            Some(*card)
        } else {
            None
        }
    }).collect::<Vec<Card>>().len();

    jacks_slice
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
        let jack_slice =   &counts[1..].iter().filter_map(|(card, _)| {
            if *card == Card::Jack {
                Some(*card)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        // NOTE: For part2, need to check whether the J is part of the current combo or not, to
        // update the hand accordingly

        if counts[0].1 == 5 {
            return (Self::FiveOfAKind, hand);

        } else if counts[0].1 == 4 {
            // [4,1] case
            // If we find even 1 J (doesnt matter if because we have 4J or 1 J), it means that we have a 5 of a kind
            if jacks_count(&counts, 0) > 0 { return (Self::FiveOfAKind, hand) }
            else {(Self::FourOfAKind, hand)}

        } else if counts[0].1 == 3 && counts[1].1 == 2 {
            // [3,2] case. If either of them are J's, we get a 5 kind
            if jacks_count(&counts, 0) > 0 { return (Self::FiveOfAKind, hand) }
            else {(Self::FullHouse, hand)}

        } else if counts[0].1 == 3  {
            // [3,1,1] case
            if jacks_count(&counts, 0) > 0 {return (Self::FourOfAKind, hand)}
            else {return (Self::ThreeOfAKind, hand)}

        } else if counts[0].1 == 2 && counts[1].1 == 2 {
            // [2,2,1]
            // possibilities:
            //     : from double to poker: either of the doubles is J [KK,JJ,10] -> [KK, KK, 10]
            //     : from double to full house: the last card is a J  -> [KK, QQ, J]
            if counts[0].0 == Card::Jack || counts[1].0 == Card::Jack { return (Self::FourOfAKind, hand)}
            else if counts[2].0 == Card::Jack { return (Self::FullHouse, hand)}
            else { return (Self::DoublePair, hand)}

        } else if counts[0].1 == 2 {
            if jacks_count(&counts, 0) > 0 { return (Self::ThreeOfAKind, hand)}
            else { return (Self::SinglePair, hand)}

        } else {
            if jacks_count(&counts, 0) > 0 {return (Self::SinglePair, hand)}
            else {return (Self::HighCard, hand)};
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

fn part1(file_string: String) -> u32 {
     // solution: 250232501 (prod1)
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
    let mut ans: u32 = 0;
    for (idx, (hand, bid)) in hands.iter().enumerate() {
        let ranking = 1 + idx as u32;
        ans += bid * (ranking)

    }

    ans
}

fn part2(file_string: String) -> u32 {
     // solution: 5905 (test2)
    // solution: 249138943 (prod2)
    let file_lines_iter = file_string.split_terminator("\n");
    let mut hands = file_lines_iter.map(|line| {
        let parts: Vec<&str> =  line.split_whitespace().collect();
        let cards = parts[0].chars().map(|c| Card::from_char(c).unwrap()).collect::<Vec<_>>();
        let hand = Hand::from_hand(cards);
        let bid = parts[1].parse::<u32>().unwrap();
        (hand, bid)
    }).collect::<Vec<_>>();
    // TODO: Need to add the "Joker" logic -> since it can be anything, then I should just add them
    // to the strongest hands
    hands.sort_by(|a,b| {
        let rank_cmp = a.0.cmp(&b.0);
        if rank_cmp == std::cmp::Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            rank_cmp
        }
    });
    // println!("Hands: {:?}", hands);
    let mut ans: u32 = 0;
    for (idx, (hand, bid)) in hands.iter().enumerate() {
        let ranking = 1 + idx as u32;
        ans += bid * (ranking)

    }

    ans
}
pub fn solve(input_path: &str) -> SolutionPair{
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
