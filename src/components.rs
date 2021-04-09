pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Enemy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MonsterSpawner;
