use std::collections::HashMap;
use std::str::FromStr;

// https://rosettacode.org/wiki/Modular_inverse#Rust
fn mod_inv(a: i128, module: i128) -> i128 {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

#[derive(Debug)]
enum Shuffle {
    NewStack,
    Cut(i32),
    Increment(i32),
}

impl FromStr for Shuffle {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("deal with") {
            let n = s
                .chars()
                .skip_while(|c| !c.is_digit(10))
                .collect::<String>()
                .parse()?;
            Ok(Shuffle::Increment(n))
        } else if s.starts_with("cut") {
            let n = s
                .chars()
                .skip_while(|&c| !c.is_digit(10) && c != '-')
                .collect::<String>()
                .parse()?;
            Ok(Shuffle::Cut(n))
        } else {
            Ok(Shuffle::NewStack)
        }
    }
}

#[derive(Debug, Clone)]
struct Card {
    value: i128,
    pos: i128,
}

impl Card {
    fn shuffle(&mut self, shuffle: &Shuffle, len: i128) {
        match shuffle {
            Shuffle::NewStack => self.pos = len - 1 - self.pos,
            Shuffle::Cut(n) => {
                let cnt = n.abs() as i128;
                self.pos = if *n < 0 {
                    (self.pos + cnt) % len
                } else {
                    (self.pos + len - cnt) % len
                };
            }
            Shuffle::Increment(n) => {
                self.pos = (self.pos * *n as i128) % len;
            }
        }
    }

    fn shuffle_rev(&mut self, shuffle: &Shuffle, len: i128) {
        match shuffle {
            Shuffle::NewStack => self.pos = len - 1 - self.pos,
            Shuffle::Cut(n) => {
                let cnt = n.abs() as i128;
                self.pos = if *n < 0 {
                    (self.pos + len - cnt) % len
                } else {
                    (self.pos + cnt) % len
                };
            }
            Shuffle::Increment(n) => {
                let n = mod_inv(*n as i128, len);
                self.pos = self.pos * n % len;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn from_len(len: i128) -> Self {
        Deck {
            cards: (0..len).map(|v| Card { value: v, pos: v }).collect(),
        }
    }

    #[allow(dead_code)]
    fn from_slice(values: &[i128]) -> Self {
        Deck {
            cards: values
                .iter()
                .cloned()
                .enumerate()
                .map(|(pos, value)| Card {
                    value,
                    pos: pos as i128,
                })
                .collect(),
        }
    }

    fn cards(&mut self) -> impl Iterator<Item = &Card> + '_ {
        self.cards.sort_by_key(|c| c.pos);
        self.cards.iter()
    }

    fn shuffle(&mut self, shuffle: &Shuffle) {
        let len = self.cards.len() as i128;
        for c in &mut self.cards {
            c.shuffle(shuffle, len);
        }
    }

    #[allow(dead_code)]
    fn shuffle_rev(&mut self, shuffle: &Shuffle) {
        let len = self.cards.len() as i128;
        for c in &mut self.cards {
            c.shuffle_rev(shuffle, len);
        }
    }
}

fn gen_shuffles(input: &str) -> crate::Result<Vec<Shuffle>> {
    Ok(input
        .lines()
        .map(Shuffle::from_str)
        .collect::<Result<_, _>>()?)
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let mut deck = Deck::from_len(10_007);
    for shuffle in gen_shuffles(input)? {
        deck.shuffle(&shuffle);
    }

    let val = deck
        .cards()
        .enumerate()
        .find_map(|(i, c)| if c.value == 2019 { Some(i) } else { None })
        .unwrap();

    Ok(val)
}

const DECK_LEN_P2: i128 = 119_315_717_514_047;
const ITER_CNT_P2: i128 = 101_741_582_076_661;

fn run_rev_shuffles(shuffles: &[Shuffle], pos: i128) -> i128 {
    let mut card = Card { value: 0, pos };

    for s in shuffles {
        card.shuffle_rev(s, DECK_LEN_P2);
    }

    card.pos
}

fn run_once(init: i128, add: i128, n: i128) -> i128 {
    (init + (n * add)) % DECK_LEN_P2
}

fn run_twice(init: i128, add: i128, n: i128) -> i128 {
    let n = run_once(init, add, n);
    run_once(init, add, n)
}

pub fn part2(input: &str) -> crate::Result<i128> {
    let mut shuffles = gen_shuffles(input)?;
    shuffles.reverse();

    let mut dmap = HashMap::new();
    {
        let init = run_rev_shuffles(&shuffles, 0);
        let add =
            (run_rev_shuffles(&shuffles, 1) - init).rem_euclid(DECK_LEN_P2);
        dmap.insert(1, (init, add));
    }

    let mut n = 1;
    {
        while n < ITER_CNT_P2 {
            let (init, add) = dmap.get(&n).unwrap();
            n *= 2;

            let nxt_init = run_twice(*init, *add, 0);
            let nxt_add =
                (run_twice(*init, *add, 1) - nxt_init).rem_euclid(DECK_LEN_P2);
            dmap.insert(n, (nxt_init, nxt_add));
        }
    }

    let mut pos = 2020;
    let mut cnt = ITER_CNT_P2;
    while cnt > 0 {
        n /= 2;
        if n <= cnt {
            let (init, add) = dmap.get(&n).unwrap();
            pos = run_once(*init, *add, pos);
            cnt -= n;
        }
    }

    Ok(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_new_stack() {
        let mut deck = Deck::from_len(10);
        deck.shuffle(&Shuffle::NewStack);
        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
        );
        deck.shuffle(&Shuffle::NewStack);
        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn deal_increment() {
        let mut deck = Deck::from_len(10);
        deck.shuffle(&Shuffle::Increment(3));
        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
    }

    #[test]
    fn cut() {
        let mut deck = Deck::from_len(10);
        deck.shuffle(&Shuffle::Cut(3));
        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
        );
    }

    #[test]
    fn cut_neg() {
        let mut deck = Deck::from_len(10);
        deck.shuffle(&Shuffle::Cut(-4));
        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn ex1() {
        let mut inp = String::new();
        inp.push_str("deal with increment 7\n");
        inp.push_str("deal into new stack\n");
        inp.push_str("deal into new stack");

        let mut deck = Deck::from_len(10);
        for shuffle in gen_shuffles(&inp).unwrap() {
            deck.shuffle(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        );
    }

    #[test]
    fn ex2() {
        let mut inp = String::new();
        inp.push_str("cut 6\n");
        inp.push_str("deal with increment 7\n");
        inp.push_str("deal into new stack");

        let mut deck = Deck::from_len(10);
        for shuffle in gen_shuffles(&inp).unwrap() {
            deck.shuffle(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
        );
    }

    #[test]
    fn ex3() {
        let mut inp = String::new();
        inp.push_str("deal with increment 7\n");
        inp.push_str("deal with increment 9\n");
        inp.push_str("cut -2");

        let mut deck = Deck::from_len(10);
        for shuffle in gen_shuffles(&inp).unwrap() {
            deck.shuffle(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
        );
    }

    #[test]
    fn ex4() {
        let mut inp = String::new();
        inp.push_str("deal into new stack\n");
        inp.push_str("cut -2\n");
        inp.push_str("deal with increment 7\n");
        inp.push_str("cut 8\n");
        inp.push_str("cut -4\n");
        inp.push_str("deal with increment 7\n");
        inp.push_str("cut 3\n");
        inp.push_str("deal with increment 9\n");
        inp.push_str("deal with increment 3\n");
        inp.push_str("cut -1");

        let mut deck = Deck::from_len(10);
        for shuffle in gen_shuffles(&inp).unwrap() {
            deck.shuffle(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
        );
    }

    #[test]
    fn p1_rev() {
        let inp = include_str!("../../input/day22").trim();

        let mut shuffles = gen_shuffles(inp).unwrap();
        shuffles.reverse();

        let mut card = Card {
            value: 0,
            pos: 8191,
        };

        for s in &shuffles {
            card.shuffle_rev(s, 10_007);
        }

        assert_eq!(card.pos, 2019);
    }

    #[test]
    fn p2_fwd() {
        let inp = include_str!("../../input/day22").trim();

        let shuffles = gen_shuffles(inp).unwrap();

        let mut card = Card {
            value: 0,
            pos: 22598648703875,
        };

        for s in &shuffles {
            card.shuffle(s, DECK_LEN_P2);
        }

        assert_eq!(card.pos, 2020);
    }

    #[test]
    fn ex1_rev() {
        let mut inp = String::new();
        inp.push_str("deal with increment 7\n");
        inp.push_str("deal into new stack\n");
        inp.push_str("deal into new stack");

        let mut deck = Deck::from_slice(&[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        let mut shuffles = gen_shuffles(&inp).unwrap();
        shuffles.reverse();
        for shuffle in &shuffles {
            deck.shuffle_rev(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn ex2_rev() {
        let mut inp = String::new();
        inp.push_str("cut 6\n");
        inp.push_str("deal with increment 7\n");
        inp.push_str("deal into new stack");

        let mut deck = Deck::from_slice(&[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);

        let mut shuffles = gen_shuffles(&inp).unwrap();
        shuffles.reverse();
        for shuffle in &shuffles {
            deck.shuffle_rev(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn ex3_rev() {
        let mut inp = String::new();
        inp.push_str("deal with increment 7\n");
        inp.push_str("deal with increment 9\n");
        inp.push_str("cut -2");

        let mut deck = Deck::from_slice(&[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);

        let mut shuffles = gen_shuffles(&inp).unwrap();
        shuffles.reverse();
        for shuffle in &shuffles {
            deck.shuffle_rev(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn ex4_rev() {
        let mut inp = String::new();
        inp.push_str("deal into new stack\n");
        inp.push_str("cut -2\n");
        inp.push_str("deal with increment 7\n");
        inp.push_str("cut 8\n");
        inp.push_str("cut -4\n");
        inp.push_str("deal with increment 7\n");
        inp.push_str("cut 3\n");
        inp.push_str("deal with increment 9\n");
        inp.push_str("deal with increment 3\n");
        inp.push_str("cut -1");

        let mut deck = Deck::from_slice(&[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);

        let mut shuffles = gen_shuffles(&inp).unwrap();
        shuffles.reverse();
        for shuffle in &shuffles {
            deck.shuffle_rev(&shuffle);
        }

        assert_eq!(
            deck.cards().map(|c| c.value).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
