use crate::control::grass::Grass;
use crate::control::simple_organism::SimpleOrganism;
use crate::control::world::World;
pub mod control;

fn main() {
    let org = SimpleOrganism::new(3, 5);
    let grass: Grass = Grass::new(1, 1);
    let mut world: World = World::new(10, 10);
    world.add_organism(Box::new(org));
    world.add_organism(Box::new(grass));
    world.printwrld();
}
