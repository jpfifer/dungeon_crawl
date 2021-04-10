use super::*;
use crate::prelude::*;

#[test]
fn generate_rooms() {
    let mut rooms_architect = rooms::RoomsArchitect {};
    let mut rng = RandomNumberGenerator::new();
    let mb = rooms_architect.new(&mut rng);

    display_rooms(&mb);
}
