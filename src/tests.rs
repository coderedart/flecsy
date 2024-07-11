use std::sync::{atomic::AtomicBool, Arc};

use crate::*;
use auto::WorldTrait;
use rstest::{fixture, rstest};
#[fixture]
fn world() -> World {
    World::mini()
}
#[rstest]
fn test_get_type(mut world: World) {
    let entity = world.new();
    assert_eq!(world.get_type(entity), None);
    let id = world.new();
    assert_eq!(world.get_type(id), None);
    world.add_id(entity, id);
    assert_eq!(world.get_type(entity), Some([id].as_slice()));
}

#[rstest]
fn test_bulk_ids(mut world: World) {
    let ids = world.bulk_new_w_id(0, 10).to_vec();
    assert_eq!(ids.len(), 10);
    for id in ids {
        assert!(world.is_alive(id));
    }
}
#[test]
fn ctx_test() {
    let mut world = crate::World::mini();
    world.set_ctx(Some(42u32));
    assert_eq!(world.get_ctx::<u32>().copied().unwrap(), 42);
}

#[test]
fn ctx_drop() {
    let mut world = crate::World::mini();
    let mut true_if_dropped = Arc::new(AtomicBool::new(false));
    struct DropMe(Arc<AtomicBool>);
    impl Drop for DropMe {
        fn drop(&mut self) {
            self.0.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
    world.set_ctx(Some(DropMe(true_if_dropped.clone())));
    assert_eq!(world.get_ctx::<DropMe>().is_some(), true);
    assert_eq!(
        true_if_dropped.load(std::sync::atomic::Ordering::SeqCst),
        false
    );
    drop(world);
    assert_eq!(
        true_if_dropped.load(std::sync::atomic::Ordering::SeqCst),
        true
    );
}

#[test]
fn at_finish() {
    let mut world = crate::World::mini();
    let mut true_at_finish = Arc::new(AtomicBool::new(false));
    fn finish_action(world: &mut World, true_at_finish: Arc<AtomicBool>) {
        assert!(world.is_finishing());
        true_at_finish.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    world.at_finish(finish_action, true_at_finish.clone());
    assert_eq!(
        true_at_finish.load(std::sync::atomic::Ordering::SeqCst),
        false
    );
    drop(world);
    assert_eq!(
        true_at_finish.load(std::sync::atomic::Ordering::SeqCst),
        true
    );
}
