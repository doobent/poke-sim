

pub fn calc_stats(base_stats: &PokemonBaseStats, dvs: &PokemonDVs, stat_exp: &PokemonStats, level: u8) -> PokemonStats {
    let health = calc_stat(
        base_stats.health, 
        dvs.calc_hp_dv(), 
        stat_exp.health, 
        level, 
        true
    );
    let attack = calc_stat(
        base_stats.attack, 
        dvs.attack, 
        stat_exp.attack, 
        level, 
        false
    );
    let defense = calc_stat(
        base_stats.defense, 
        dvs.defense, 
        stat_exp.defense, 
        level, 
        false
    );
    let speed = calc_stat(
        base_stats.speed, 
        dvs.speed, 
        stat_exp.speed, 
        level, 
        false
    );
    let special = calc_stat(
        base_stats.special, 
        dvs.special, 
        stat_exp.special, 
        level, 
        false
    );

    PokemonStats { 
        health, 
        attack, 
        defense, 
        speed, 
        special 
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

/// Only need u8 for base stats
#[derive(Debug, Clone, Copy, Default)]
pub struct PokemonBaseStats {
    pub health: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub special: u8,
}

impl PokemonBaseStats {
    pub fn new(health: u8, attack: u8, defense: u8, speed: u8, special: u8) -> PokemonBaseStats {
        PokemonBaseStats {
            health,
            attack,
            defense,
            speed,
            special,
        }
    }
}

/// Stats for pokemon, used for stat(between 1 and 999) and stat exp(between 0 and 65535)
#[derive(serde::Deserialize)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PokemonStats {
    pub health: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub special: u16,
}

impl PokemonStats {
    pub fn new(health: u16, attack: u16, defense: u16, speed: u16, special: u16) -> PokemonStats {
        PokemonStats {
            health,
            attack,
            defense,
            speed,
            special,
        }
    }
}

/// Determinant values(also known as individual values) of a pokemon
/// Health DV is calculated from the others, so not stored here
/// TODO: change default to random dvs?(0-15)
#[derive(serde::Deserialize)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PokemonDVs {
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub special: u8,
}

impl PokemonDVs {
    /// HP DV is the LSB(least significant bit) of the other 4 DVs
    /// [attack, defense, speed, special]
    pub fn calc_hp_dv(&self) -> u8 {
        let mut hp_dv = 0;

        hp_dv += (self.attack & 1) << 3;
        hp_dv += (self.defense & 1) << 2;
        hp_dv += (self.speed & 1) << 1;
        hp_dv += self.special & 1;

        return hp_dv;
    }

    pub fn new(attack: u8, defense: u8, speed: u8, special: u8) -> PokemonDVs {
        PokemonDVs {
            attack,
            defense,
            speed,
            special,
        }
    }
}