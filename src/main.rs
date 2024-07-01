use crate::control::organism::Organism;
use crate::control::world::World;

pub mod control;

fn main() {
    let mut world: World = World::new(10, 10);
    let organism: Organism = Organism::origin();
    println!("{:?}", organism);
    println!("{:?}", organism.position.on_world(world.dim()));
    world.add_organism(organism);
    println!("{:?}", world);
}
