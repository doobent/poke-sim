
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
        let health = Pokemon::calc_stat(
            self.base_pokemon.base_stats.health, 
            self.dv_values.calc_hp_dv(), 
            self.stat_exp.health, 
            self.level, 
            true
        );
        let attack = Pokemon::calc_stat(
            self.base_pokemon.base_stats.attack, 
            self.dv_values.attack, 
            self.stat_exp.attack, 
            self.level, 
            false
        );
        let defense = Pokemon::calc_stat(
            self.base_pokemon.base_stats.defense, 
            self.dv_values.defense, 
            self.stat_exp.defense, 
            self.level, 
            false
        );
        let speed = Pokemon::calc_stat(
            self.base_pokemon.base_stats.speed, 
            self.dv_values.speed, 
            self.stat_exp.speed, 
            self.level, 
            false
        );
        let special = Pokemon::calc_stat(
            self.base_pokemon.base_stats.special, 
            self.dv_values.special, 
            self.stat_exp.special, 
            self.level, 
            false
        );

        PokemonStats {
            health,
            attack,
            defense,
            speed,
            special,
        }
    }

    fn calc_stat(base_stat: u8, dv_value: u8, stat_exp: u16, level: u8, hp_stat: bool) -> u16 {
        let value = ((u16::from(base_stat) + u16::from(dv_value)) * 2 + 
            ((stat_exp as f64).sqrt()/4.0).floor() as u16) * u16::from(level) / 100;
        
        if hp_stat {
            value + u16::from(level) + 10
        } else {
            value + 5
        }
    }
}

/// Determinant values(also known as individual values) of a pokemon
/// Health DV is calculated from the others, so not stored here
/// TODO: change default to random dvs?(0-15)
#[derive(Debug, Clone, Copy, Default)]
pub struct PokemonDVs {
    attack: u8,
    defense: u8,
    speed: u8,
    special: u8,
}

impl PokemonDVs {
    /// HP DV is the LSB of the other 4 DVs
    /// [attack, defense, speed, special]
    pub fn calc_hp_dv(&self) -> u8 {
        let mut hp_dv = 0;

        hp_dv += (self.attack & 1) << 3;
        hp_dv += (self.defense & 1) << 2;
        hp_dv += (self.speed & 1) << 1;
        hp_dv += self.special & 1;

        return hp_dv;
    }
}

/// Stats for pokemon, used for stat(between 1 and 999) and stat exp(between 0 and 65535)
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

        let base_stats = PokemonBaseStats {
            health,
            attack,
            defense,
            speed,
            special,
        };
        BasePokemon {
            index,
            base_stats,
            type1,
            type2,
        }
    }
}

/// Only need u8 for base stats
#[derive(Debug, Clone, Copy, Default)]
struct PokemonBaseStats {
    health: u8,
    attack: u8,
    defense: u8,
    speed: u8,
    special: u8,
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