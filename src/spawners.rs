pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        Health {
            current: 20,
            max: 20,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (glyph, health) = match rng.range(0, 4) {
        0 => (to_cp437('E'), 20),
        1 => (to_cp437('O'), 15),
        2 => (to_cp437('o'), 10),
        _ => (to_cp437('g'), 3),
    };
    ecs.push((
        Enemy,
        MovingRandomly,
        Health {
            current: health,
            max: health,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}
