use wasm_bindgen::prelude::*;

use js_sys::*;
use crate::wasm_misc::log::*;
use crate::pokemon::Type;
use crate::stats::PokemonBaseStats;
use crate::stats::PokemonDVs;
use crate::stats::PokemonStats;
use crate::stats::calc_stats;

use serde::{Serialize, Deserialize};

use crate::BasePokemon;

const HEALTH_STR: &str = "health";
const ATTACK_STR: &str = "attack";
const DEFENSE_STR: &str = "defense";
const SPEED_STR: &str = "speed";
const SPECIAL_STR: &str = "special";

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

#[wasm_bindgen]
pub fn get_stats(
    base_health: u8, base_attack: u8, base_defense: u8, base_speed: u8, base_special: u8,
    dv_attack: u8, dv_defense: u8, dv_speed: u8, dv_special: u8,
    stat_exp_health: u16, stat_exp_attack: u16, stat_exp_defense: u16, stat_exp_speed: u16, stat_exp_special: u16,
    level: u8,
) -> Object
{
    let base_stats = PokemonBaseStats::new(base_health, base_attack, base_defense, base_speed, base_special);
    let dv_stats = PokemonDVs::new(dv_attack, dv_defense, dv_speed, dv_special);
    let stat_exp = PokemonStats::new(stat_exp_health, stat_exp_attack, stat_exp_defense, stat_exp_speed, stat_exp_special);

    let stats = calc_stats(&base_stats, &dv_stats, &stat_exp, level);

    let object = Object::new();

    Reflect::set(&object, &JsValue::from_str(HEALTH_STR), &JsValue::from(stats.health));
    Reflect::set(&object, &JsValue::from_str(ATTACK_STR), &JsValue::from(stats.attack));
    Reflect::set(&object, &JsValue::from_str(DEFENSE_STR), &JsValue::from(stats.defense));
    Reflect::set(&object, &JsValue::from_str(SPEED_STR), &JsValue::from(stats.speed));
    Reflect::set(&object, &JsValue::from_str(SPECIAL_STR), &JsValue::from(stats.special));

    return object;
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