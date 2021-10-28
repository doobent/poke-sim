use wasm_bindgen::prelude::*;

use js_sys::*;
use crate::wasm_misc::log::*;
use crate::pokemon::Type;

use std::convert::TryInto;

use serde::{Serialize, Deserialize};

use crate::BasePokemon;

#[wasm_bindgen]
pub fn load_pokemon(pokemon_array: Array) {
    console_log!("{:?}", pokemon_array.get(1));

    //let vec = Vec::with_capacity(pokemon_array.length().try_into().unwrap());

    let obj= Object::from(pokemon_array.get(1));

    let data = obj.into_serde::<PokemonData>().unwrap();

    let type1 = Type::from_str(&data.type1);
    let type2 = Type::from_str(&data.type2);

    let base_poke = BasePokemon::new(1, data.health, data.attack,
        data.defense, data.speed, data.special, 
        type1, type2);

    console_log!("{:?}", base_poke);
}

fn convert_pokemon(value: &JsValue) -> BasePokemon {
    //let obj: &Object = value;

        unimplemented!()
}

#[derive(Debug, Serialize, Deserialize)]
struct PokemonData {
    name: String,
    health: u8,
    attack: u8,
    defense: u8,
    speed: u8,
    special: u8,
    type1: String,
    type2: String,
    starting_moves: Vec<String>,
    learnset: Vec<LearnMove>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LearnMove {
    level: u8,
    name: String,
}