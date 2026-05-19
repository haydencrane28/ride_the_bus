use game_core::*;
use std::sync::Arc;
use std::sync::Mutex;
use axum::{
    routing::post,
    Router,
    extract::State,
    extract::Json
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

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
    let app = Router::new()
        .route("/new-game", post(new_game_handler))
        .route("/guess", post(answer))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn new_game_handler(State(state): State<Arc<Mutex<GameState>>>) -> Json<GuessResponse> {
    let mut guard = state.lock().unwrap();
    *guard = new_game();
    Json( GuessResponse {
        correct: false,
        card: "".to_string(),
        step: "red_black".to_string(),
        status: "in_progress".to_string(),
    })
}

#[derive(Deserialize)]
struct GuessRequest {
    guess: String,
}

#[derive(Serialize)]
struct GuessResponse {
    correct: bool,
    card: String,
    step: String,
    status: String
}

async fn answer(State(state): State<Arc<Mutex<GameState>>>, request: Json<GuessRequest>) -> Json<GuessResponse> {
    let mut guard = state.lock().unwrap();
    match draw_card(&mut guard.deck) {
        None => { 
            guard.status = CurrStatus::OutOfCards;
            return Json(GuessResponse {
                correct: false,
                card: "deck_empty".to_string(),
                step: "red_black".to_string(),
                status: "out_of_cards".to_string(),
            });
        }
        Some(card) => {
            match guard.step {
                CurrStep::RedBlack => {
                    guard.card1 = Some(card);
                    if check_red_or_black(&guard.card1.as_ref().unwrap(), &request.guess){
                        guard.step = CurrStep::HighLow;
                        return Json(GuessResponse{
                            correct: true,
                            card: card_to_string(&guard.card1.as_ref().unwrap()),
                            step: "high_low".to_string(),
                            status: "in_progress".to_string(),
                        });
                    } else {
                        return Json(GuessResponse{
                            correct: false,
                            card: card_to_string(&guard.card1.as_ref().unwrap()),
                            step: "red_black".to_string(),
                            status: "in_progress".to_string(),
                        });
                    }

                }
                CurrStep::HighLow => {
                    guard.card2 = Some(card);
                    if check_high_low(&guard.card1.as_ref().unwrap(), &guard.card2.as_ref().unwrap() , &request.guess) {
                        guard.step = CurrStep::InOut;
                        return Json(GuessResponse{
                            correct: true,
                            card: card_to_string(&guard.card2.as_ref().unwrap()),
                            step: "in_out".to_string(),
                            status: "in_progress".to_string(),
                        });
                    } else {
                        guard.step = CurrStep::RedBlack;
                        return Json(GuessResponse{
                            correct: false,
                            card: card_to_string(&guard.card2.as_ref().unwrap()),
                            step: "red_black".to_string(),
                            status: "in_progress".to_string(),
                        });
                    }

                }
                CurrStep::InOut => {
                    guard.card3 = Some(card);
                    if check_in_out(&guard.card1.as_ref().unwrap(), &guard.card2.as_ref().unwrap(), &guard.card3.as_ref().unwrap(), &request.guess) {
                        guard.step = CurrStep::Suit;
                        return Json(GuessResponse{
                            correct: true,
                            card: card_to_string(&guard.card3.as_ref().unwrap()),
                            step: "suit".to_string(),
                            status: "in_progress".to_string(),
                        });
                    } else {
                        guard.step = CurrStep::RedBlack;
                        return Json(GuessResponse{
                            correct: false,
                            card: card_to_string(&guard.card3.as_ref().unwrap()),
                            step: "red_black".to_string(),
                            status: "in_progress".to_string(),
                        });
                    }
                }
                CurrStep::Suit => {
                    if check_suit(&card, &request.guess) {
                        guard.status = CurrStatus::Won;
                        return Json(GuessResponse{
                            correct: true,
                            card: card_to_string(&card),
                            step: "suit".to_string(),
                            status: "won".to_string(),
                        });
                    } else {
                        guard.step = CurrStep::RedBlack;
                        return Json(GuessResponse{
                            correct: false,
                            card: card_to_string(&card),
                            step: "red_black".to_string(),
                            status: "in_progress".to_string(),
                        });
                    }
                }
            }
        }
    };
}