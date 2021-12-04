
const POKEMON_DATA_PATH: &str = "data/pokemon.json";

use std::fs::{self, read_to_string};

use clap::{Arg, App};
use common::{stats::{PokemonDVs, PokemonStats}, pokemon::BasePokemon};
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

            println!("{:?}", stat_input);
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

fn get_base_pokemon() -> Vec<BasePokemon> {

    let pokes = Vec::new();

    let pokemon_data = fs::read_to_string(POKEMON_DATA_PATH).expect("Couldn't read pokemon data file");

    

    pokes
}