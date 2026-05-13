use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Suit {
        Hearts,
        Diamonds,
        Clubs,
        Spades,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Rank {
        Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
    }

    #[derive(Debug, Clone, Copy)]
    struct Card {
        suit: Suit,
        rank: Rank,
    }

    fn create_deck() -> Vec<Card> {
        let suits = [
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Clubs,
            Suit::Spades,
        ];

        let ranks = [
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
            Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack,
            Rank::Queen, Rank::King, Rank::Ace,
        ];

        let mut deck = Vec::new();

        for suit in suits {
            for rank in ranks {
                deck.push(Card { suit, rank });
            }
        }
        
        deck
    }

    fn shuffle_deck(deck: &mut Vec<Card>) {
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
    }

    fn draw_card(deck: &mut Vec<Card>) -> Option<Card> {
        deck.pop()
    }

    fn check_red_or_black(card: &Card, guess: &str) -> bool {
        if (card.suit == Suit::Hearts || card.suit == Suit::Diamonds) && guess.trim().eq_ignore_ascii_case("red") {
            true
        } else if (card.suit == Suit::Clubs || card.suit == Suit::Spades) && guess.trim().eq_ignore_ascii_case("black") {
            true
        } else {
            false
        }
    }

    fn check_high_low(card1: &Card, card2: &Card, guess: &str) -> bool {
        (card2.rank > card1.rank && guess.trim().eq_ignore_ascii_case("higher"))
            || (card2.rank < card1.rank && guess.trim().eq_ignore_ascii_case("lower"))
    }

    fn check_in_out(card1: &Card, card2: &Card, card3: &Card, guess: &str) -> bool {
        let is_inside = (card3.rank > card1.rank && card3.rank < card2.rank) || (card3.rank > card2.rank && card3.rank < card1.rank);
        let is_outside = !is_inside;

        (is_inside && guess.trim().eq_ignore_ascii_case("inside")) || (is_outside && guess.trim().eq_ignore_ascii_case("outside"))
    }

    fn check_suit(card: &Card, guess: &str) -> bool {
        let suit = match card.suit {
            Suit::Hearts => "hearts",
            Suit::Diamonds => "diamonds",
            Suit::Clubs => "clubs",
            Suit::Spades => "spades",
        };

        return guess.trim().eq_ignore_ascii_case(suit);

    }

    fn print_card(card: &Card) {
        let rank = match card.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };

        let suit = match card.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };

        println!("You drew the {} of {}", rank, suit);
    }

    fn play_round(deck: &mut Vec<Card>) -> bool{

        if deck.len() == 0 {
            return false;
        }

        let mut guess = String::new();
        loop {
            println!("Red or black?");
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            if is_valid_input(&guess) {
                break;
            }
            guess = String::new();
        }

        let Some(card1) = draw_card(deck) else {
            println!("No more cards in the deck! Game over.");
            return false;   
        };

        if !check_red_or_black(&card1, &guess) {
            println!("Wrong color!");
            return false;
        }

        print_card(&card1);

        let mut guess = String::new();
        loop {
            println!("Higher or lower?");
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            if is_valid_input(&guess) {
                break;
            }
            guess = String::new();
        }
        
        

        let Some(card2) = draw_card(deck) else {
            println!("No more cards in the deck!");
            return false;   
        };

        if !check_high_low(&card1, &card2, &guess) {
            println!("Wrong guess for higher or lower!");
            return false;
        }

        print_card(&card2);

        let mut guess = String::new();
        loop {
            println!("Inside or outside?");
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            if is_valid_input(&guess) {
                break;
            }
            guess = String::new();
        }

        let Some(card3) = draw_card(deck) else {
            println!("No more cards in the deck!");
            return false;   
        };

        if !check_in_out(&card1, &card2, &card3, &guess) {
            println!("Wrong guess for inside or outside!");
            return false;
        }

        print_card(&card3);

        let mut guess = String::new();
        loop {
            println!("Guess the suit?");
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            if is_valid_input(&guess) {
                break;
            }
            guess = String::new();
        }
        
        let Some(card4) = draw_card(deck) else {
            println!("No more cards in the deck!");
            return false;   
        };

        if !check_suit(&card4, &guess) {
            println!("Wrong suit!");
            return false;
        }

        print_card(&card4);

        println!("Congratulations! You guessed all correctly!");
        true
    }

    fn is_valid_input(guess: &str) -> bool {
        let valid_guesses = ["red", "black", "inside", "outside", "higher", "lower", "spades", "hearts", "clubs", "diamonds"];
        if valid_guesses.contains(&guess.trim().to_ascii_lowercase().as_str()) {
            true
        } else {
            println!("Invalid guess.");
            false
        }
    }
            

fn main() {

    let mut deck = create_deck();

    shuffle_deck(&mut deck);

    for _ in 0..51 {
        if play_round(&mut deck) {
            break;
        }
        if deck.len() == 0 {
            println!("No more cards in the deck! Game over.");
        }
    }

    println!("Remaining cards: {}", deck.len());

}
