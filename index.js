import init, { get_1 } from "./pkg/poke_sim.js";

async function run() {
    await init();

    console.log(get_1());
}

run();