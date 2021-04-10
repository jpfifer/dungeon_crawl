use super::*;
use crate::map_builder::drunkard::DESIRED_FLOOR;
use crate::prelude::*;

#[test]
pub fn generate_drunkard() {
    let mut rng = RandomNumberGenerator::new();
    let mut drunkard = drunkard::DrunkardsWalkArchitect {};
    let mut mb = drunkard.new(&mut rng);

    display_rooms(&mb);
}
