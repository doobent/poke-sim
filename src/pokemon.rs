
pub type Party = [Option<Pokemon>; 6];

/// Pokemon in battle, stores their status and which pokemon they are
struct BattlePokemon {
    statuses: BattleStatus,
    pokemon: Pokemon,
}


/// Temporary statuses for in battle
#[derive(Debug, Clone, Copy, Default)]
struct BattleStatus {
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

/// This is general 
#[derive(Debug, Clone, Copy, Default)]
pub struct Pokemon {
    stats: Option<PokemonStats>,
    level: u8,
    stat_exp: PokemonStats,
    base_pokemon: BasePokemon,
    status: Status,
}

impl Pokemon {
    pub fn calc_stats(&mut self) {
        
    }
}

/// Stats for pokemon
#[derive(Debug, Clone, Copy, Default)]
pub struct PokemonStats {
    health: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special: u16,
}

/// Base info for each pokemon
#[derive(Debug, Clone, Copy, Default)]
struct BasePokemon {
    index: u8,
    base_stats: PokemonStats,
    type1: Type,
    type2: Type,
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

#[derive(Debug, Clone, Copy)]
enum Type {
    Physical(PhysicalType),
    Special(SpecialType),
}

impl Default for Type {
    fn default() -> Self {
        Type::Physical(PhysicalType::Normal)
    }
}

#[derive(Debug, Clone, Copy)]
enum PhysicalType {
    Normal,
}

#[derive(Debug, Clone, Copy)]
enum SpecialType {
    Psychic,
}