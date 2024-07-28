use living_world_rust::control::world::World;
use living_world_rust::orgs::grass::Grass;
use living_world_rust::orgs::simple_organism::SimpleOrganism;

fn main() {
    let org = SimpleOrganism::new(3, 5, 10);
    let grass: Grass = Grass::new(1, 1, 10);
    let mut world: World = World::new(10, 10);
    world.add_organism(Box::new(org));
    world.add_organism(Box::new(grass));
    for _ in 0..33 {
        world.make_turn();
    }
}
