use flecsy::{World, WorldTraitManual};

fn main() {
    let mut world = World::default();
    world.set_ctx(Some(42u32));
    assert_eq!(world.get_ctx::<u32>().copied().unwrap(), 42);
}
