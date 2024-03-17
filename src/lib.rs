mod game;
mod input;
mod upgrade;

//Import source modules
use crate::game::ElevatorGame;
use crate::input::ElevatorGameInput;
use crate::upgrade::ElevatorGameUpgrades;

//Import standard/imported libraries
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use elevate_lib::building::Building;
use elevate_lib::controller::{RandomController};

lazy_static! {
  static ref GAME: Mutex<ElevatorGame> = Mutex::new(
    ElevatorGame::from(
      Box::new(
        RandomController::from_building(
          Building::from(
            4_usize,
            2_usize,
            0.5_f64,
            100_usize,
            10_usize,
            5.0_f64,
            2.5_f64,
            0.5_f64
          )
        )
      ),
      ElevatorGameUpgrades::new(),
      StdRng::from_seed(rand::thread_rng().gen())
    )
  );
}

#[wasm_bindgen]
pub fn update_game_state(input: String) {
  //Parse the input JSON string into an input object
  let game_input: ElevatorGameInput = ElevatorGameInput::from_json(input);

  //Acquire lock for game state and update given input
  let mut game = GAME.lock().unwrap();
  game.update_game_state(game_input);
}

#[wasm_bindgen]
pub fn get_game_state() -> String {
  let mut game = GAME.lock().unwrap();
  game.get_game_state()
}