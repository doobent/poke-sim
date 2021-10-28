
use crate::stats::PokemonStats;
use crate::stats::PokemonBaseStats;
use crate::stats::PokemonDVs;
use crate::stats::calc_stats;

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
    dv_values: PokemonDVs,
    status: Status, 
}

impl Pokemon {
    pub fn calc_stats(&self) -> PokemonStats {
        calc_stats(&self.base_pokemon.base_stats, &self.dv_values, &self.stat_exp, self.level)
    }
}

/// Base info for each pokemon
#[derive(Debug, Clone, Copy, Default)]
pub struct BasePokemon {
    index: u8,
    base_stats: PokemonBaseStats,
    type1: Type,
    type2: Type,
}

impl BasePokemon {
    pub fn new(index: u8, health: u8, attack: u8,
        defense: u8, speed: u8, special: u8, 
        type1: Type, type2: Type) -> BasePokemon {

        let base_stats = PokemonBaseStats::new(health, attack, defense, speed, special);
        BasePokemon {
            index,
            base_stats,
            type1,
            type2,
        }
    }
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
pub enum Type {
    Physical(PhysicalType),
    Special(SpecialType),
}

impl Type {
    pub fn from_str(string: &str) -> Type {
        match string {
            "normal" => Type::Physical(PhysicalType::Normal),
            "fighting" => Type::Physical(PhysicalType::Fighting),
            "flying" => Type::Physical(PhysicalType::Flying),
            "poison" => Type::Physical(PhysicalType::Poison),
            "ground" => Type::Physical(PhysicalType::Ground),
            "rock" => Type::Physical(PhysicalType::Rock),
            "bird" => Type::Physical(PhysicalType::Bird),
            "bug" => Type::Physical(PhysicalType::Bug),
            "ghost" => Type::Physical(PhysicalType::Ghost),
            "fire" => Type::Special(SpecialType::Fire),
            "water" => Type::Special(SpecialType::Water),
            "grass" => Type::Special(SpecialType::Grass),
            "electric" => Type::Special(SpecialType::Electric),
            "psychic" => Type::Special(SpecialType::Psychic),
            "ice" => Type::Special(SpecialType::Ice),
            "dragon" => Type::Special(SpecialType::Dragon),
            _ => panic!("Invalid type passed in - {}", string),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Physical(PhysicalType::Normal)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PhysicalType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bird,
    Bug,
    Ghost,
}

#[derive(Debug, Clone, Copy)]
pub enum SpecialType {
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
}