import init, { get_1, load_pokemon } from "./pkg/poke_sim.js";

async function run() {
    await init();

    console.log(get_1());
}

async function load_pokemon_data() {
    let val = await fetch("./data/pokemon.json");

    let json = await val.json();
    console.log(json);
    load_pokemon(json);
}

load_pokemon_data();


run();