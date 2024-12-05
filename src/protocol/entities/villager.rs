use crate::protocol::types::VarInt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Villager {
    pub head_shake_ticks: VarInt,
    pub biome: VillagerBiome,
    pub profession: VillagerProfession,
    pub level: VarInt,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum VillagerBiome {
    #[default]
    Desert = 0,
    Jungle = 1,
    Plains = 2,
    Savanna = 3,
    Snow = 4,
    Swamp = 5,
    Taiga = 6,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum VillagerProfession {
    #[default]
    None = 0,
    Armorer = 1,
    Butcher = 2,
    Cartographer = 3,
    Cleric = 4,
    Farmer = 5,
    Fisherman = 6,
    Fletcher = 7,
    Leatherworker = 8,
    Librarian = 9,
    Mason = 10,
    Nitwit = 11,
    Shepherd = 12,
    Toolsmith = 13,
    Weaponsmith = 14,
}
