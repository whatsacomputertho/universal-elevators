mod game;

//Import source modules
use crate::game::ElevatorGame;

//Import standard/imported libraries
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use elevate_lib::building::Building;
use elevate_lib::controller::RandomController;

lazy_static! {
  static ref GAME: Mutex<ElevatorGame<RandomController>> = Mutex::new(
    ElevatorGame::from(
      RandomController::from(
        Building::from(
          4_usize,
          2_usize,
          0.5_f64,
          5.0_f64,
          2.5_f64,
          0.5_f64
        ),
        StdRng::from_seed(rand::thread_rng().gen())
      ),
      StdRng::from_seed(rand::thread_rng().gen())
    )
  );
}

#[wasm_bindgen]
pub fn update_game_state() {
  let mut game = GAME.lock().unwrap();
  game.update_game_state();
}

#[wasm_bindgen]
pub fn get_game_state() -> String {
  let game = GAME.lock().unwrap();
  game.get_game_state()
}