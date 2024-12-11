use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    info!("Logger initialized!");
}

#[wasm_bindgen]
pub fn double_value(input: i32) -> i32 {
    info!("Received input: {}", input);
    let result = input * 2;
    info!("Computed result: {}", result);
    result
}
