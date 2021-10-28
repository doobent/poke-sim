#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;



use crate::pokemon::BasePokemon;

mod pokemon;
mod stats;

#[cfg(target_arch = "wasm32")]
mod wasm_load;
#[cfg(target_arch = "wasm32")]
mod wasm_misc;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_1() -> u8 {
    7
}

fn load_pokemon(pokemon: Vec<BasePokemon>) {

}
