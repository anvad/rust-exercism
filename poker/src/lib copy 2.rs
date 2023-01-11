/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 0 {
        return Vec::new();
    }
    let hands_with_score: Vec<_> = hands.iter().map(|&h| (get_score(h), h)).collect();

    let max_score = hands_with_score.iter().max().unwrap_or(&(0, "")).0;
    hands_with_score
        .iter()
        .filter(|(score, _)| *score == max_score)
        .map(|&(_, hand)| hand)
        .collect()
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

/// given a poker hand string, returns score
fn get_score(hand: &str) -> u32 {
    // convert hand into 5-tuple of 2-tuples
    let mut hand_tuple = hand
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
    hand_tuple.sort();

    // now find the type of the hand and corresponding score

    let suit = hand_tuple[0].1;
    let is_flush = hand_tuple.iter().all(|(_, s)| *s == suit);

    let hand_nums: Vec<_> = hand_tuple.iter().map(|(n, _)| *n).collect();
    match hand_nums[..] {
        [n1, n2, n3, n4, n5] if n1 == n2 - 1 && n1 == n3 - 2 && n1 == n4 - 3 && n1 == n5 - 4 => {
            match is_flush {
                true => STRAIGHT_FLUSH + sum((n1, n2, n3, n4, n5)),
                _ => STRAIGHT + sum((n1, n2, n3, n4, n5)),
            }
        }
        [2, 3, 4, 5, 14] => match is_flush {
            true => STRAIGHT_FLUSH + sum((1, 2, 3, 4, 5)),
            _ => STRAIGHT + sum((1, 2, 3, 4, 5)),
        },
        [n1, n2, n3, n4, n5] if is_flush => FLUSH + sum((n1, n2, n3, n4, n5)),
        [n1, n2, n3, n4, l1] if n1 == n2 && n1 == n3 && n1 == n4 => {
            FOUR_OAK + sum((l1, n1, n2, n3, n4))
        }
        [l1, n1, n2, n3, n4] if n1 == n2 && n1 == n3 && n1 == n4 => {
            FOUR_OAK + sum((l1, n1, n2, n3, n4))
        }
        [p1, p2, t1, t2, t3] if p1 == p2 && t1 == t2 && t1 == t3 => {
            FULL_HOUSE + sum((p1, p2, t1, t2, t3))
        }
        [t1, t2, t3, p1, p2] if t1 == t2 && t1 == t3 && p1 == p2 => {
            FULL_HOUSE + sum((p1, p2, t1, t2, t3))
        }
        [n1, n2, n3, l1, l2] if n1 == n2 && n1 == n3 => THREE_OAK + sum((l1, l2, n1, n2, n3)),
        [l1, n1, n2, n3, l2] if n1 == n2 && n1 == n3 => THREE_OAK + sum((l1, l2, n1, n2, n3)),
        [l1, l2, n1, n2, n3] if n1 == n2 && n1 == n3 => THREE_OAK + sum((l1, l2, n1, n2, n3)),
        [n1, n2, n3, n4, l1] if n1 == n2 && n3 == n4 => TWO_PAIR + sum((l1, n1, n2, n3, n4)),
        [n1, n2, l1, n3, n4] if n1 == n2 && n3 == n4 => TWO_PAIR + sum((l1, n1, n2, n3, n4)),
        [l1, n1, n2, n3, n4] if n1 == n2 && n3 == n4 => TWO_PAIR + sum((l1, n1, n2, n3, n4)),
        [n1, n2, l1, l2, l3] if n1 == n2 => ONE_PAIR + sum((l1, l2, l3, n1, n2)),
        [l1, n1, n2, l2, l3] if n1 == n2 => ONE_PAIR + sum((l1, l2, l3, n1, n2)),
        [l1, l2, n1, n2, l3] if n1 == n2 => ONE_PAIR + sum((l1, l2, l3, n1, n2)),
        [l1, l2, l3, n1, n2] if n1 == n2 => ONE_PAIR + sum((l1, l2, l3, n1, n2)),
        [n1, n2, n3, n4, n5] => HIGH_CARD + sum((n1, n2, n3, n4, n5)),
        _ => 0,
    }
}

fn sum((n1, n2, n3, n4, n5): (u8, u8, u8, u8, u8)) -> u32 {
    (n1 as u32) * 2u32.pow(0)
        + (n2 as u32) * 2u32.pow(4)
        + (n3 as u32) * 2u32.pow(8)
        + (n4 as u32) * 2u32.pow(12)
        + (n5 as u32) * 2u32.pow(16)
}
