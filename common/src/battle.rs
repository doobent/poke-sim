
use super::pokemon::Pokemon;
use super::stats::PokemonStats;

pub struct Battle {
    pokemon1: BattlePokemon,
    pokemon2: BattlePokemon,
}

impl Battle {
    pub fn new(poke1: Pokemon, poke2: Pokemon) -> Battle {
        let pokemon1 = BattlePokemon::new(poke1);
        let pokemon2 = BattlePokemon::new(poke2);

        Battle {
            pokemon1,
            pokemon2,
        }
    }

    ///Runs and returns a log of the events
    pub fn run_log(&self) -> String {
        let log = String::new();


    }

    /// false is pokemon1 winning, true is pokemon2 winning
    pub fn run_result(&self) -> bool {

    }

    fn run_turn(&self) {
        //pick moves for both pokes

        //choose who goes first, priority then speed
    }
}

/// Pokemon in battle, stores their status and which pokemon they are
struct BattlePokemon {
    battle_stats: PokemonStats,
    status: Status,
    effects: BattleEffects,
    pokemon: Pokemon,
}

impl BattlePokemon {
    fn new(pokemon: Pokemon) -> BattlePokemon {
        let battle_stats = pokemon.get_stats();
        BattlePokemon {
            battle_stats,
            status: Default::default(),
            effects: Default::default(),
            pokemon,
        }
    }
}


/// Temporary effects while in battle
#[derive(Debug, Clone, Copy, Default)]
struct BattleEffects {
    bide: bool, //
    thrash: bool, //also petal dance
    multi_attack: bool, //fury swipes
    flinch: bool,
    charging: bool, //like solarbeam
    multi_turn: bool, //like wrap
    invulnerable: bool, //dig/fly
    confusion: bool,
    x_accuracy: bool,
    mist: bool, //also guard spec
    focus_energy: bool,
    substitute: bool,
    recharge: bool, //needs to recharge, like hyper beam
    rage: bool,
    leech_seed: bool,
    toxic: bool,
    light_screen: bool,
    reflect: bool,
    transform: bool,
}

#[derive(Debug, Clone, Copy)]
enum Status {
    None,
    Poisoned,
    Burned,
    Paralyzed,
    Frozen,
}

impl Default for Status {
    fn default() -> Self {
        Status::None
    }
}