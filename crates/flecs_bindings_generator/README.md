just run `cargo run -p flecs_bindings_generator` while in the workspace directory. 

It will read `flecs.h` and write bindgen based bindings into `crates/flecys/src/lib.rs` file.
Then, it will try to generate safe wrappers for limited API into `src/auto.rs` file. 