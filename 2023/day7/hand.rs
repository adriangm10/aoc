use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveEq = 7,
    FourEq = 6,
    FullHouse = 5,
    ThreeEq = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand<'a> {
    pub hand: &'a str,
    pub bid: u32,
    hand_type: HandType,
    card_order: HashMap<char, u8>,
}

impl Hand<'_> {
    pub fn new(hand: &'_ str, bid: u32) -> Hand<'_> {
        // part 1
        // let card_order = HashMap::from([
        //     ('A', 14),
        //     ('K', 13),
        //     ('Q', 12),
        //     ('J', 11),
        //     ('T', 10),
        //     ('9', 9),
        //     ('8', 8),
        //     ('7', 7),
        //     ('6', 6),
        //     ('5', 5),
        //     ('4', 4),
        //     ('3', 3),
        //     ('2', 2),
        // ]);

        //part 2
        let card_order = HashMap::from([
            ('A', 13),
            ('K', 12),
            ('Q', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
            ('J', 1),
        ]);

        let hand = Hand {
            hand,
            bid,
            hand_type: Hand::hand_type2(hand),
            card_order,
        };
        return hand;
    }

    fn hand_type1(hand: &'_ str) -> HandType {
        let mut counts = HashMap::new();

        for i in hand.chars() {
            match counts.get_mut(&i) {
                Some(c) => *c += 1,
                None => {
                    counts.insert(i, 1);
                }
            };
        }

        match counts.keys().count() {
            1 => HandType::FiveEq,
            2 => {
                if counts.iter().find(|(_, count)| **count == 4).is_some() {
                    return HandType::FourEq;
                }
                return HandType::FullHouse;
            }
            3 => {
                if counts.iter().find(|(_, count)| **count == 3).is_some() {
                    return HandType::ThreeEq;
                }
                return HandType::TwoPair;
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!("the hand cannot have more than 5 cards"),
        }

        // match counts.keys().count() {
        //     1 => HandType::FiveEq(*counts.keys().next().unwrap()),
        //     2 => {
        //         if let Some((card, _)) = counts.iter().find(|(_, count)| **count == 4) {
        //             return HandType::FourEq(*card);
        //         }

        //         return HandType::FullHouse(
        //             *counts.keys().nth(0).unwrap(),
        //             *counts.keys().nth(1).unwrap(),
        //         );
        //     }
        //     3 => {
        //         let mut c1 = '\0';
        //         let mut c2 = '\0';
        //         for (card, count) in counts.iter() {
        //             if *count == 3 {
        //                 return HandType::ThreeEq(*card);
        //             }

        //             if *count == 2 {
        //                 if c1 == '\0' {
        //                     c1 = *card;
        //                 } else {
        //                     c2 = *card;
        //                 }
        //             }
        //         }
        //         return HandType::TwoPair(c1, c2);
        //     }
        //     4 => {
        //         let (card, _) = counts.iter().find(|(_, count)| **count == 2).unwrap();
        //         return HandType::OnePair(*card);
        //     },
        //     5 => {
        //         let card = counts.iter().fold('2', |acc, (card, _)| {
        //             match card {
        //                 'A' => acc,
        //                 'K' => if acc != 'A' { 'K' } else { acc },
        //                 'Q' => if acc != 'A' || acc != 'K' { 'Q' } else { acc },
        //                 'J' => if acc == 'T' || acc.is_digit(10) { 'J' } else { acc },
        //                 'T' => if acc.is_digit(10) { 'T' } else { acc },
        //                 c if card.is_digit(10) => if *c > acc { *c } else { acc },
        //                 _ => unreachable!("Unknown card"),
        //             }
        //         });

        //         return HandType::HighCard(card);
        //     },
        //     _ => unreachable!("the hand cannot have more than 5 cards"),
        // }
    }

    fn hand_type2(hand: &'_ str) -> HandType {
        let mut counts = HashMap::new();

        for i in hand.chars() {
            match counts.get_mut(&i) {
                Some(c) => *c += 1,
                None => {
                    counts.insert(i, 1);
                }
            };
        }

        match counts.keys().count() {
            1 => HandType::FiveEq,
            2 => {
                if counts.get(&'J').is_some() {
                    return HandType::FiveEq;
                }

                if counts.iter().find(|(_, count)| **count == 4).is_some() {
                    return HandType::FourEq;
                }

                return HandType::FullHouse;
            }
            3 => {
                if counts.iter().find(|(_, count)| **count == 3).is_some() {
                    if counts.get(&'J').is_some() {
                        return HandType::FourEq;
                    }
                    return HandType::ThreeEq;
                }
                if let Some(jokers) = counts.get(&'J') {
                    return match jokers {
                        1 => HandType::FullHouse,
                        2 => HandType::FourEq,
                        _ => unreachable!("too many jokers"),
                    };
                }
                return HandType::TwoPair;
            }
            4 => {
                if counts.get(&'J').is_some() {
                    return HandType::ThreeEq;
                }
                return HandType::OnePair;
            }
            5 => {
                if counts.get(&'J').is_some() {
                    return HandType::OnePair;
                }
                return HandType::HighCard;
            }
            _ => unreachable!("the hand cannot have more than 5 cards"),
        }
    }
}

pub struct ParseHandError;

impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = ParseHandError;

    fn try_from(value: &'a str) -> Result<Hand<'a>, Self::Error> {
        let (hand, bid) = value.split_once(" ").ok_or(ParseHandError)?;
        let bid = bid.parse::<u32>().map_err(|_| ParseHandError)?;
        Ok(Hand::new(hand, bid))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            return std::cmp::Ordering::Greater;
        }
        if self.hand_type < other.hand_type {
            return std::cmp::Ordering::Less;
        }

        match self
            .hand
            .chars()
            .zip(other.hand.chars())
            .find(|(scard, ocard)| scard != ocard)
        {
            Some((self_card, other_card)) => {
                println!("{}, {}", self_card, other_card);
                return if self.card_order.get(&self_card).unwrap() > self.card_order.get(&other_card).unwrap() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                };
            }
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
