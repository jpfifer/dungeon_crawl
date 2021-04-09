pub use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        Health {
            current: 10,
            max: 10,
        },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        FieldOfView::new(8),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn make_monster(
    rng: &mut RandomNumberGenerator,
    pos: Point,
) -> (
    Enemy,
    ChasingPlayer,
    Health,
    Name,
    Point,
    Render,
    FieldOfView,
) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };
    (
        Enemy,
        ChasingPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        FieldOfView::new(6),
    )
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push(make_monster(rng, pos));
}

pub fn spawn_spawner(ecs: &mut World, pos: Point) {
    ecs.push((
        Enemy,
        MonsterSpawner,
        Health {
            current: 10,
            max: 10,
        },
        Name("Monster Chest".to_string()),
        pos,
        Render {
            color: ColorPair::new(RED, BLUE),
            glyph: to_cp437('&'),
        },
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
