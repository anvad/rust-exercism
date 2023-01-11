/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // high card (13 * 12 * 11 * 10 * 9 ways = 154440) max score: 14+13+12+11+10 = 60
    // < one pair max score: 14+14 + 13 + 12 + 11 = 64
    // < two pair max score: 14+14 + 13+13 + 12 = 66
    // < 3 of a kind max score: 14+14+14 + 13 + 12 = 67
    // < straight max score: 14+13+12+11+10 = 60
    // < flush max score: 14+13+12+11+10 = 60
    // < full house max score: 14+14+14 + 13+13 = 68
    // < 4 of a kind: 14+14+14+14 + 13 = 69
    // < straight flush (10 ways) max score: 14+13+12+11+10 = 60

    if hands.len() == 0 {
        return Vec::new();
    }
    let hands_with_score: Vec<_> = hands.iter().map(|&h| (get_score(h), h)).collect();
    let max_score = hands_with_score.iter().max().unwrap_or(&(0, "")).0;
    // hands_with_score.sort();
    // let max_score = hands_with_score.last().unwrap_or(&(0, "")).0;
    println!("{:#?}", hands_with_score);
    hands_with_score
        .iter()
        .filter(|(score, _)| *score == max_score)
        .map(|(_, hand)| *hand)
        .collect()
}

/// given a poker hand string, returns score
fn get_score(hand: &str) -> u32 {
    // convert hand into 5-tuple of 2-tuples
    let mut ht = hand
        .split(' ')
        .map(|card| match card.as_bytes() {
            [n1, n2, s] if *n1 == b'1' && *n2 == b'0' => (10, *s),
            [n, s] => match (*n, *s) {
                (b'A', s) => (14, s),
                (b'K', s) => (13, s),
                (b'Q', s) => (12, s),
                (b'J', s) => (11, s),
                (n, s) => (n - b'0', s),
            },
            _ => (b'0', b'S'),
        })
        .collect::<Vec<_>>();
    ht.sort();
    let (hand_type, score) = get_hand_type(&ht);
    println!("{hand}\n\t= {:?}, {:?}", ht, hand_type);
    score
}

fn sum((n1, n2, n3, n4, n5): (u8, u8, u8, u8, u8)) -> u32 {
    (n1 as u32) * 2u32.pow(0)
        + (n2 as u32) * 2u32.pow(4)
        + (n3 as u32) * 2u32.pow(8)
        + (n4 as u32) * 2u32.pow(12)
        + (n5 as u32) * 2u32.pow(16)
}

fn get_hand_type(hand: &Vec<(u8, u8)>) -> (PokerHandType, u32) {
    use PokerHandType::*;

    let suit = hand[0].1;
    let is_flush = hand.iter().all(|(_, s)| *s == suit);

    let hand_nums: Vec<_> = hand.iter().map(|(n, _)| *n).collect();
    let (hand_type, score) = match hand_nums[..] {
        [n1, n2, n3, n4, n5] if n1 == n2 - 1 && n1 == n3 - 2 && n1 == n4 - 3 && n1 == n5 - 4 => {
            match is_flush {
                true => (StraightFlush, STRAIGHT_FLUSH + sum((n1, n2, n3, n4, n5))),
                _ => (Straight, STRAIGHT + sum((n1, n2, n3, n4, n5))),
            }
        }
        [2, 3, 4, 5, 14] => match is_flush {
            true => (StraightFlush, STRAIGHT_FLUSH + sum((1, 2, 3, 4, 5))),
            _ => (Straight, STRAIGHT + sum((1, 2, 3, 4, 5))),
        },
        [n1, n2, n3, n4, n5] if is_flush => (Flush, FLUSH + sum((n1, n2, n3, n4, n5))),
        [n1, n2, n3, n4, l1] if n1 == n2 && n1 == n3 && n1 == n4 => {
            (FourOAK, FOUR_OAK + sum((l1, n1, n2, n3, n4)))
        }
        [l1, n1, n2, n3, n4] if n1 == n2 && n1 == n3 && n1 == n4 => {
            (FourOAK, FOUR_OAK + sum((l1, n1, n2, n3, n4)))
        }
        [p1, p2, t1, t2, t3] if p1 == p2 && t1 == t2 && t1 == t3 => {
            (FullHouse, FULL_HOUSE + sum((p1, p2, t1, t2, t3)))
        }
        [t1, t2, t3, p1, p2] if t1 == t2 && t1 == t3 && p1 == p2 => {
            (FullHouse, FULL_HOUSE + sum((p1, p2, t1, t2, t3)))
        }
        [n1, n2, n3, l1, l2] if n1 == n2 && n1 == n3 => {
            (ThreeOAK, THREE_OAK + sum((l1, l2, n1, n2, n3)))
        }
        [l1, n1, n2, n3, l2] if n1 == n2 && n1 == n3 => {
            (ThreeOAK, THREE_OAK + sum((l1, l2, n1, n2, n3)))
        }
        [l1, l2, n1, n2, n3] if n1 == n2 && n1 == n3 => {
            (ThreeOAK, THREE_OAK + sum((l1, l2, n1, n2, n3)))
        }
        [n1, n2, n3, n4, l1] if n1 == n2 && n3 == n4 => {
            (TwoPair, TWO_PAIR + sum((l1, n1, n2, n3, n4)))
        }
        [n1, n2, l1, n3, n4] if n1 == n2 && n3 == n4 => {
            (TwoPair, TWO_PAIR + sum((l1, n1, n2, n3, n4)))
        }
        [l1, n1, n2, n3, n4] if n1 == n2 && n3 == n4 => {
            (TwoPair, TWO_PAIR + sum((l1, n1, n2, n3, n4)))
        }
        [n1, n2, l1, l2, l3] if n1 == n2 => (OnePair, ONE_PAIR + sum((l1, l2, l3, n1, n2))),
        [l1, n1, n2, l2, l3] if n1 == n2 => (OnePair, ONE_PAIR + sum((l1, l2, l3, n1, n2))),
        [l1, l2, n1, n2, l3] if n1 == n2 => (OnePair, ONE_PAIR + sum((l1, l2, l3, n1, n2))),
        [l1, l2, l3, n1, n2] if n1 == n2 => (OnePair, ONE_PAIR + sum((l1, l2, l3, n1, n2))),
        [n1, n2, n3, n4, n5] => (HighCard, HIGH_CARD + sum((n1, n2, n3, n4, n5))),
        _ => (HighCard, 100),
    };

    (hand_type, score)
}

#[derive(Debug)]
enum PokerHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    Straight,
    Flush,
    FullHouse,
    FourOAK,
    StraightFlush,
}
static HIGH_CARD: u32 = 2u32.pow(20);
static ONE_PAIR: u32 = 2u32.pow(21);
static TWO_PAIR: u32 = 2u32.pow(22);
static THREE_OAK: u32 = 2u32.pow(23);
static STRAIGHT: u32 = 2u32.pow(24);
static FLUSH: u32 = 2u32.pow(25);
static FULL_HOUSE: u32 = 2u32.pow(26);
static FOUR_OAK: u32 = 2u32.pow(27);
static STRAIGHT_FLUSH: u32 = 2u32.pow(28);
