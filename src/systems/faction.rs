use bevy::prelude::Component;

#[derive(PartialEq, Copy, Clone)]
pub enum Factions {
    Player,
    Aliens,
}
impl Default for Factions {
    fn default() -> Self {
        Factions::Player
    }
}

#[derive(Default, Component)]
pub struct Faction {
    pub faction: Factions,
}
impl Faction {
    pub fn new(faction: Factions) -> Self {
        Self { faction }
    }
}
