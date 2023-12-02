use wasm_bindgen::prelude::*;
use std::ffi::CString;
use std::os::raw::c_char;

static GAME_NAME: &'static str = "Universal Elevators";

#[wasm_bindgen]
pub fn get_game_name() -> *mut c_char {
  let s = CString::new(GAME_NAME).unwrap();
  s.into_raw()
}

#[wasm_bindgen]
pub fn get_game_name_len() -> usize {
  GAME_NAME.len()
}