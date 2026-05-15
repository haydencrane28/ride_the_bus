use game_core::*;

struct GameState {
    deck: Vec<Card>,
    card1: Option<Card>,
    card2: Option<Card>,
    card3: Option<Card>,
    step: CurrStep,
    status: CurrStatus,
}

enum CurrStep {
    RedBlack,
    HighLow,
    InOut,
    Suit,
}

enum CurrStatus{
    InProgress,
    Won,
    Lost,
    OutOfCards,
}

fn new_game() -> GameState {
    let mut deck = create_deck();
    shuffle_deck(&mut deck);
    GameState {
        deck,
        card1: None,
        card2: None,
        card3: None,
        step: CurrStep::RedBlack,
        status: CurrStatus::InProgress,
    }
}

fn main() {}