use super::*;
use crate::prelude::*;

#[test]
fn generate_automata_rooms() {
    let mut architect = automata::CellularAutomataArchitect {};
    let mut rng = RandomNumberGenerator::new();
    let mb = architect.new(&mut rng);

    display_rooms(&mb)
}
