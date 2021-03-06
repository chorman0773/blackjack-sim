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

fn is_equal(l: &Card, r: &Card) -> bool {
    match (l.val, r.val) {
        (
            Value::King | Value::Queen | Value::Jack | Value::Number(10),
            Value::King | Value::Queen | Value::Jack | Value::Number(10),
        ) => true,
        (Value::Ace, Value::Ace) => true,
        (Value::Number(n), Value::Number(m)) => n == m,
        _ => false,
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

fn one_round<R: Rng>(deck: &mut Vec<Card>, rng: &mut R, win_count: &mut u32) {
    let mut player_hand = smallvec::SmallVec::<[Card; 5]>::new();
    let mut player_hand2 = None;
    let mut dealer_hand = smallvec::SmallVec::<[Card; 5]>::new();
    let mut skip = false;
    if deck.len() < DECK.len() / 3 {
        println!("Shuffled");
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
        return;
    } else {
        println!(
            "Player Hand: {} (Score {})",
            PrintHand(&player_hand),
            calculate_hand_value(&player_hand)
        );
        println!("Dealer's Hand: {}|**", dealer_hand[0]);
        if is_equal(&player_hand[0], &player_hand[1]) {
            print!("(H)it, (S)tand, s(P)lit, (D)ouble> ");
        } else {
            print!("(H)it, (S)tand, (D)ouble> ");
        }

        io::stdout().flush().unwrap();
        let mut buf = String::with_capacity(2);
        io::stdin().read_line(&mut buf).unwrap();

        match buf.as_bytes()[0] {
            b'H' | b'h' => draw(deck, &mut player_hand, 1),
            b'S' | b's' => skip = true,
            b'P' | b'p' => {
                if !is_equal(&player_hand[0], &player_hand[1]) {
                    eprintln!("Cannot split {}", PrintHand(&player_hand));
                    return;
                }
                let mut hand2 = smallvec::SmallVec::<[Card; 5]>::new();
                hand2[0] = player_hand.pop().unwrap(); // Player has 2 cards in hand
                draw(deck, &mut player_hand, 1);
                draw(deck, &mut hand2, 1);
                player_hand2 = Some(hand2);
            }
            b'D' | b'd' => {
                println!("Doubled Down");
                draw(deck, &mut player_hand, 1);
                skip = true;
            }
            i => {
                eprintln!(
                    "Error: unexpected input {}: Expected H, S, P, or D",
                    i as char
                );
            }
        }
    }
    while !skip {
        println!(
            "Player Hand: {} (Score {})",
            PrintHand(&player_hand),
            calculate_hand_value(&player_hand)
        );
        if let Some(hand2) = &player_hand2 {
            println!(
                "Split Player Hand: {} (Score {})",
                PrintHand(hand2),
                calculate_hand_value(&player_hand)
            );
        }
        println!("Dealer's Hand: {}|**", dealer_hand[0]);

        if calculate_hand_value(&player_hand) >= GOAL {
            break;
        }
        print!("(H)it, (S)tand> ");
        io::stdout().flush().unwrap();
        let mut buf = String::with_capacity(2);
        io::stdin().read_line(&mut buf).unwrap();

        match buf.as_bytes()[0] {
            b'H' | b'h' => draw(deck, &mut player_hand, 1),
            b'S' | b's' => skip = true,
            i => {
                eprintln!("Error: unexpected input {}: Expected H or S", i as char);
                continue;
            }
        }
    }

    if let Some(hand2) = &mut player_hand2 {
        println!("Split Hand");
        skip = false;
        while !skip {
            println!(
                "Player Hand: {} (Score {})",
                PrintHand(&player_hand),
                calculate_hand_value(&player_hand)
            );
            println!(
                "Split Player Hand: {} (Score {})",
                PrintHand(hand2),
                calculate_hand_value(&player_hand)
            );
            println!("Dealer's Hand: {}|**", dealer_hand[0]);

            if calculate_hand_value(hand2) >= GOAL {
                break;
            }
            print!("(H)it, (S)tand> ");
            io::stdout().flush().unwrap();
            let mut buf = String::with_capacity(2);
            io::stdin().read_line(&mut buf).unwrap();

            match buf.as_bytes()[0] {
                b'H' | b'h' => draw(deck, hand2, 1),
                b'S' | b's' => skip = true,
                i => {
                    eprintln!("Error: unexpected input {}: Expected H or S", i as char);
                    continue;
                }
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
    let mut hand1_bust = false;
    if player_result > GOAL {
        println!("Player Busts");
        hand1_bust = true;
    }

    let split_result = player_hand2.as_deref().map(calculate_hand_value);

    let mut hand2_bust = split_result.is_none();
    if split_result.unwrap_or(0) > GOAL {
        println!("Split Hand Busts");
        hand2_bust = true;
    }

    if hand1_bust && hand2_bust {
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
        *win_count += (1 - hand1_bust as u32) + (1 - (hand2_bust || split_result.is_none()) as u32);
        println!("Dealer Busts");
        return;
    }
    if !hand1_bust {
        match player_result.cmp(&dealer_result) {
            Ordering::Less => println!("Hand 1: Dealer Wins"),
            Ordering::Equal => println!("Hand 1: Tie"),
            Ordering::Greater => {
                *win_count += 1;
                println!("Hand 1: Player wins")
            }
        }
    }

    if !hand2_bust {
        let split_result = split_result.unwrap();
        match split_result.cmp(&dealer_result) {
            Ordering::Less => println!("Hand 2: Dealer Wins"),
            Ordering::Equal => println!("Hand 2: Tie"),
            Ordering::Greater => {
                *win_count += 1;
                println!("Hand 2: Player wins")
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut deck = Vec::from(DECK);
    let mut win_count = 0;
    let mut round_count = 0;
    deck.shuffle(&mut rng);
    loop {
        one_round(&mut deck, &mut rng, &mut win_count);
        round_count += 1;
        println!("Player wins {}. Rounds Played {}", win_count, round_count);
        println!("Press enter to continue>");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
