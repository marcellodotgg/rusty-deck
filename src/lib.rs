use std::usize;

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /** Returns an empty deck of cards. */
    pub fn empty() -> Deck {
        Deck { cards: Vec::new() }
    }

    /**
     * Creates a new deck of cards containing 52 playing cards
     * which are not shuffled.
     *
     * If you would like to create a shuffled deck, use `shuffled()`.
     */
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(Suit::iter().count() * Rank::iter().count());

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck { cards }
    }

    /**
     * Creates, shuffles, and returns the deck. Shorthand
     * for calling `new()` then `shuffle()`.
     */
    pub fn shuffled() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        deck
    }

    /**
     * Shuffles the deck in place.
     */
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    /**
     * Draws the given number of cards from the top of the deck.
     *
     * Returns an error if you try to draw more cards than are available.
     */
    pub fn draw(&mut self, number_of_cards: u8) -> Result<Hand, String> {
        if number_of_cards as usize > self.cards.len() {
            return Err("Not enough cards in the deck".to_string());
        }

        let hand = Hand {
            cards: self.cards.drain(..number_of_cards as usize).collect(),
        };

        Ok(hand)
    }

    /**
     * Discards the given number of cards from the deck.
     */
    pub fn discard(&mut self, number_of_cards: &mut u8) {
        if *number_of_cards as usize > self.cards.len() {
            *number_of_cards = self.cards.len() as u8;
        }
        self.cards.drain(..*number_of_cards as usize);
    }

    /**
     * Adds a card to the end of the deck.
     */
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /**
     * Adds a group of cards to the end of the deck.
     */
    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
    }

    /**
     * Adds another deck to the end of the deck.
     */
    pub fn add_deck(&mut self, deck: Deck) {
        self.cards.extend(deck.cards);
    }

    /**
     * Gets the length of the deck.
     */
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn add(&mut self, other_hand: Hand) {
        self.cards.extend(other_hand.cards)
    }
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

#[derive(EnumIter, Debug, Copy, Clone)]
pub enum Rank {
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

#[derive(EnumIter, Debug, Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}
