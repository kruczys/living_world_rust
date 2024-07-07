use living_world_rust::control::world::World;
use living_world_rust::orgs::grass::Grass;
use living_world_rust::orgs::simple_organism::SimpleOrganism;

fn main() {
    let org = SimpleOrganism::new(3, 5);
    let grass: Grass = Grass::new(1, 1);
    let mut world: World = World::new(10, 10);
    world.add_organism(Box::new(org));
    world.add_organism(Box::new(grass));
    world.printwrld();
}
