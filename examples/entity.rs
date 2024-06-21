use flecsys::*;
fn main() {
    unsafe {
        let world = ecs_init();
        let mut entity_desc: ecs_entity_desc_t = std::mem::MaybeUninit::zeroed().assume_init();

        entity_desc.name = c"My Entity".as_ptr();
        // create an entity
        let entity = ecs_entity_init(world, &entity_desc as *const _);
        // get the name using entity and print it
        let name = std::ffi::CStr::from_ptr(ecs_get_name(world, entity))
            .to_str()
            .unwrap();
        println!("{name}");
        ecs_fini(world);
    }
}
