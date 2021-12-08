
const POKEMON_DATA_PATH: &str = "data/pokemon.json";

use std::fs::{self, read_to_string};

use clap::{Arg, App};
use common::{stats::{PokemonDVs, PokemonStats, calc_stats, PokemonBaseStats}, pokemon::BasePokemon};
use serde::Deserialize;


fn main() {
    let matches = App::new("Pokemon Simulator")
                    .version("0.1.0")
                    .author("doobent")
                    .about("Does pokemon battle simulations for generation 1")
                    .arg(Arg::with_name("mode")
                        .short("m")
                        .long("mode")
                        .takes_value(true)
                        .default_value("battle")
                        .possible_values(&["stat", "battle"]))
                    .arg(Arg::with_name("input")
                        .long("input")
                        .takes_value(true)
                    )
                    .get_matches();
    
    match matches.value_of("mode").unwrap() {
        "stat" => {
            let input_file = matches.value_of("input").unwrap();

            let input_str = read_to_string(input_file).expect("Couldn't read from input file");
            let stat_input: PokemonStatInput = serde_json::from_str(&input_str).expect("Input file wasn't valid pokemon data");

            let base_pokemon = get_base_pokemon();

            let base_pokemon = base_pokemon.iter().find(|poke| poke.name == stat_input.pokemon_name)
                .expect("Couldn't find pokemon from name");

            let base_stats = PokemonBaseStats::new(base_pokemon.health, base_pokemon.attack, base_pokemon.defense, base_pokemon.speed, base_pokemon.special);

            let stats = calc_stats(&base_stats, &stat_input.dvs, &stat_input.stat_exp, stat_input.level);

            println!("{:?}", stats);
        },
        "battle" => {

        },
        _ => unreachable!(),
    }

}

#[derive(Deserialize, Debug, Clone)]
struct PokemonStatInput {
    pokemon_name: String,
    level: u8,
    dvs: PokemonDVs,
    stat_exp: PokemonStats,
}

fn get_base_pokemon() -> Vec<PokemonDataInput> {

    let pokemon_data = fs::read_to_string(POKEMON_DATA_PATH).expect("Couldn't read pokemon data file");

    let data_input: Vec<PokemonDataInput> = serde_json::from_str(&pokemon_data).expect("");

    data_input
}

#[derive(Deserialize, Debug, Clone)]
struct PokemonDataInput {
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

#[derive(Deserialize, Debug, Clone)]
struct LearnMove {
    level: u8,
    name: String,
}