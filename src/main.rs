use crate::control::grass::Grass;
use crate::control::orgs_data_model::Survival;
use crate::control::simple_organism::SimpleOrganism;
use crate::control::world::World;
pub mod control;

fn main() {
    let org = SimpleOrganism::new(0, 0);
    let grass: Grass = Grass::new(1, 1);
    let mut world: World = World::new(10, 10);
    world.add_organism(org);
    world.add_organism(grass);

    println!("Hello world");
}
