mod automata_test;
mod drunkard_test;
mod rooms_tests;

use super::*;

fn display_rooms(mb: &MapBuilder) {
    mb.map.tiles.iter().enumerate().for_each(|(idx, tile)| {
        let display = match tile {
            _ if map_index(mb.player_start.x, mb.player_start.y) == idx => "@",
            _ if mb
                .monster_spawns
                .iter()
                .find(|p| map_index(p.x, p.y) == idx)
                .is_some() =>
            {
                "M"
            }
            TileType::Wall => "#",
            TileType::Floor => ".",
        };

        if idx > 0 && idx % (SCREEN_WIDTH as usize) == 0 {
            print!("\n");
        }
        print!("{}", display);
    })
}

#[test]
fn map_builder_test() {
    let mut rng = RandomNumberGenerator::new();
    let mb = MapBuilder::new(&mut rng);

    display_rooms(&mb);
}
