## rusty-deck
My spin on standard-deck using the Rust language instead of Go. This is my first official project in Rust.

## Usage
```rs
use rusty_deck::Deck;

fn main() {
    // Create a deck to play a round of Blackjack
    let mut deck = Deck::shuffled();

    // Draw two cards
    let mut hand = deck.draw(2).unwrap();

    // Draw another card
    hand.add(deck.draw(1).unwrap());

    // Look at your hand
    println!("{:?}", hand);
}
```
