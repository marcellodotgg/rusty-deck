use rusty_deck::Deck;

fn main() {
    // Play a round of Blackjack
    let mut deck = Deck::shuffled();

    // Draw two cards
    let mut hand = deck.draw(2).unwrap();

    // Draw another card
    hand.add(deck.draw(1).unwrap());

    // Look at your hand
    println!("{:?}", hand);
}
