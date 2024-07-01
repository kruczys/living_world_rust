use crate::control::organism::Organism;
use crate::control::world::World;

pub mod control;

fn main() {
    let mut world: World = World::new(10, 10);
    let organism: Organism = Organism::origin();
    println!("{:?}", organism);
    world.add_organism(organism);
    println!("{:?}", world);
}
