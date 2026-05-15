use game_core::*;
use std::sync::Arc;
use std::sync::Mutex;
use axum::{
    routing::post,
    Router,
    extract::State
};

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

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(new_game()));
    let app = Router::new().route("/new-game", post(new_game_handler)).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn new_game_handler(State(state): State<Arc<Mutex<GameState>>>) -> &'static str {
    let mut guard = state.lock().unwrap();
    *guard = new_game();
    "New game started"
}