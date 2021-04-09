use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(Point)]
#[read_component(MonsterSpawner)]
pub fn monster_spawner(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut spawner = <(&Point, &MonsterSpawner)>::query();
    spawner.iter(ecs).for_each(|(p, _)| {
        let mut monsters = <&Enemy>::query();

        let count = monsters.iter(ecs).count();
        if count < 5 {
            let mut rng = RandomNumberGenerator::new();
            if rng.range(0, 100) > 75 {
                let spawn_position = match rng.range(0, 4) {
                    0 => Point::new(-1, 0),
                    1 => Point::new(1, 0),
                    2 => Point::new(0, -1),
                    _ => Point::new(0, 1),
                } + *p;
                if map.can_enter_tile(spawn_position) {
                    commands.push(make_monster(&mut rng, spawn_position));
                }
            }
        }
    });
}
