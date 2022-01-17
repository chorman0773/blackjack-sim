use rand::{seq::SliceRandom, Rng};
use std::{
    cmp::Ordering,
    fmt::Display,
    io::{self, Read, Write},
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

impl Display for Suit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Clubs => fmt.write_str("C"),
            Self::Spades => fmt.write_str("S"),
            Self::Diamonds => fmt.write_str("D"),
            Self::Hearts => fmt.write_str("H"),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Number(u32),
}

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ace => fmt.write_str("A"),
            Self::King => fmt.write_str("K"),
            Self::Queen => fmt.write_str("Q"),
            Self::Jack => fmt.write_str("J"),
            Self::Number(n) => n.fmt(fmt),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Card {
    pub val: Value,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.suit, self.val))
    }
}

fn calculate_hand_value(x: &[Card]) -> u32 {
    let mut ace_count = 0;
    let mut total = 0;
    for i in x.iter().map(|Card { val, .. }| val) {
        match i {
            Value::Ace => {
                ace_count += 1;
                total += 1;
            }
            Value::King | Value::Queen | Value::Jack => total += 10,
            Value::Number(n) => total += *n,
        }
    }

    if total < 21 {
        let diff = (21 - total) / 10;
        total += diff.min(ace_count) * 10;
    }

    total
}

#[rustfmt::skip] // Rustfmt looks garbage
const DECK: &[Card] = &[
    Card{val: Value::Ace,suit: Suit::Spades},
    Card{val: Value::Ace,suit: Suit::Clubs},
    Card{val: Value::Ace,suit: Suit::Diamonds},
    Card{val: Value::Ace,suit: Suit::Hearts},
    Card{val: Value::King,suit: Suit::Spades},
    Card{val: Value::King,suit: Suit::Clubs},
    Card{val: Value::King,suit: Suit::Diamonds},
    Card{val: Value::King,suit: Suit::Hearts},
    Card{val: Value::Queen,suit: Suit::Spades},
    Card{val: Value::Queen,suit: Suit::Clubs},
    Card{val: Value::Queen,suit: Suit::Diamonds},
    Card{val: Value::Queen,suit: Suit::Hearts},
    Card{val: Value::Jack,suit: Suit::Spades},
    Card{val: Value::Jack,suit: Suit::Clubs},
    Card{val: Value::Jack,suit: Suit::Diamonds},
    Card{val: Value::Jack,suit: Suit::Hearts},
    Card{val: Value::Number(10),suit: Suit::Spades},
    Card{val: Value::Number(10),suit: Suit::Clubs},
    Card{val: Value::Number(10),suit: Suit::Diamonds},
    Card{val: Value::Number(10),suit: Suit::Hearts},
    Card{val: Value::Number(9),suit: Suit::Spades},
    Card{val: Value::Number(9),suit: Suit::Clubs},
    Card{val: Value::Number(9),suit: Suit::Diamonds},
    Card{val: Value::Number(9),suit: Suit::Hearts},
    Card{val: Value::Number(8),suit: Suit::Spades},
    Card{val: Value::Number(8),suit: Suit::Clubs},
    Card{val: Value::Number(8),suit: Suit::Diamonds},
    Card{val: Value::Number(8),suit: Suit::Hearts},
    Card{val: Value::Number(7),suit: Suit::Spades},
    Card{val: Value::Number(7),suit: Suit::Clubs},
    Card{val: Value::Number(7),suit: Suit::Diamonds},
    Card{val: Value::Number(7),suit: Suit::Hearts},
    Card{val: Value::Number(6),suit: Suit::Spades},
    Card{val: Value::Number(6),suit: Suit::Clubs},
    Card{val: Value::Number(6),suit: Suit::Diamonds},
    Card{val: Value::Number(6),suit: Suit::Hearts},
    Card{val: Value::Number(5),suit: Suit::Spades},
    Card{val: Value::Number(5),suit: Suit::Clubs},
    Card{val: Value::Number(5),suit: Suit::Diamonds},
    Card{val: Value::Number(5),suit: Suit::Hearts},
    Card{val: Value::Number(4),suit: Suit::Spades},
    Card{val: Value::Number(4),suit: Suit::Clubs},
    Card{val: Value::Number(4),suit: Suit::Diamonds},
    Card{val: Value::Number(4),suit: Suit::Hearts},
    Card{val: Value::Number(3),suit: Suit::Spades},
    Card{val: Value::Number(3),suit: Suit::Clubs},
    Card{val: Value::Number(3),suit: Suit::Diamonds},
    Card{val: Value::Number(3),suit: Suit::Hearts},
    Card{val: Value::Number(2),suit: Suit::Spades},
    Card{val: Value::Number(2),suit: Suit::Clubs},
    Card{val: Value::Number(2),suit: Suit::Diamonds},
    Card{val: Value::Number(2),suit: Suit::Hearts},
];

pub struct PrintHand<'a>(&'a [Card]);

impl<'a> Display for PrintHand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut needs_sep = false;
        for card in self.0 {
            if needs_sep {
                f.write_str("|")?;
            }
            card.fmt(f)?;
            needs_sep = true
        }
        Ok(())
    }
}

fn draw<V: Extend<Card>>(deck: &mut Vec<Card>, hand: &mut V, n: usize) {
    let len = deck.len();
    assert!(n < len);
    hand.extend(deck.drain((len - n)..))
}

const GOAL: u32 = 21;
const DEAL_GOAL: u32 = 17;

fn one_round<R: Rng>(deck: &mut Vec<Card>, rng: &mut R) {
    let mut player_hand = smallvec::SmallVec::<[Card; 5]>::new();
    let mut dealer_hand = smallvec::SmallVec::<[Card; 5]>::new();
    if deck.len() < DECK.len() / 3 {
        deck.clear();
        deck.extend_from_slice(DECK);
        deck.shuffle(rng);
    }
    draw(deck, &mut player_hand, 2);
    draw(deck, &mut dealer_hand, 2);
    if calculate_hand_value(&player_hand) == GOAL {
        println!(
            "Player Hand: {} (Score {})",
            PrintHand(&player_hand),
            calculate_hand_value(&player_hand)
        );
        println!(
            "Dealer's Hand: {} (Score {})",
            PrintHand(&dealer_hand),
            calculate_hand_value(&dealer_hand)
        );
        println!("Blackjack: Player Win's");
    }
    loop {
        println!(
            "Player Hand: {} (Score {})",
            PrintHand(&player_hand),
            calculate_hand_value(&player_hand)
        );
        println!("Dealer's Hand: {}|**", dealer_hand[0]);

        if calculate_hand_value(&player_hand) >= GOAL {
            break;
        }
        print!("(H)it, (S)tand, s(P)lit, (D)ouble> ");
        io::stdout().flush().unwrap();
        let mut buf = String::with_capacity(2);
        io::stdin().read_line(&mut buf).unwrap();

        match buf.as_bytes()[0] {
            b'H' | b'h' => draw(deck, &mut player_hand, 1),
            b'S' | b's' => break,
            b'P' | b'p' => {
                eprintln!("Error: Split is not yet implemented");
                continue;
            }
            b'D' | b'd' => {
                draw(deck, &mut player_hand, 1);
                break;
            }
            i => {
                eprintln!(
                    "Error: unexpected input {}: Expected H, S, P, or D",
                    i as char
                );
                continue;
            }
        }
    }

    println!(
        "Player Hand: {} (Score {})",
        PrintHand(&player_hand),
        calculate_hand_value(&player_hand)
    );
    println!(
        "Dealer's Hand: {} (Score {})",
        PrintHand(&dealer_hand),
        calculate_hand_value(&dealer_hand)
    );
    let player_result = calculate_hand_value(&player_hand);
    if player_result > GOAL {
        println!("Player Busts");
        return;
    }
    while calculate_hand_value(&dealer_hand) < DEAL_GOAL {
        draw(deck, &mut dealer_hand, 1);
        println!(
            "Dealer's Hand: {} (Score {})",
            PrintHand(&dealer_hand),
            calculate_hand_value(&dealer_hand)
        );
    }
    let dealer_result = calculate_hand_value(&dealer_hand);
    if dealer_result > GOAL {
        println!("Dealer Busts");
        return;
    }

    match player_result.cmp(&dealer_result) {
        Ordering::Less => println!("Dealer Wins"),
        Ordering::Equal => println!("Tie"),
        Ordering::Greater => println!("Player wins"),
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut deck = Vec::from(DECK);
    deck.shuffle(&mut rng);
    loop {
        one_round(&mut deck, &mut rng);
        println!("Press enter to continue>");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
