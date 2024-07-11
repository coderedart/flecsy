# WARNING: WORK IN PROGRESS.


This contains opinionated bindings for flecs. The main purpose is to enable scripting using wasmtime, while also sandboxing those addons (limit what addons can access or mutate in ECS).


I made a `null-terminated` utf-8 string type in `src/nullstr.rs` to make the bindings more ergonomic.

So, At *this* particular moment, I am trying to autogenerate bindings using `flecs-bindings-generator` crate. 
* This will allow us to evolve bindings easily by just modifying a little code in the generator crate
* We don't need to repeat boiler plate. Although AI can help to some extent, it will still be a lot of manual monkey typing.
* We can switch the kinds of bindings generated based on our config. 
    * Wasm Guest: When generating bindings that wasm guests will use, we can skip certain functions (eg: ecs_fini or ecs_quit), as guests shouldn't be able to do that. 
    * Wasm Host: This will generate boiler plate code that adds "host fns" to wasmtime linker (and wasmer in near future) that will connect to the guest bindings. But this time, we can add validation code like checking if the entity that is modified by the call is actually owned by the guest or return an error. There's LOTS of variations here. Another use case might be disallowing mutating fns when in readonly mode. Or not allowing fn calls when the plugin is in a different thread (async?). This also requires us to think about passing in and out data by value, rather than pointers. 
    * Host native: This is the usuall FFI bindings we do for any C library. Because we have direct access to memory of the c library, this is the simplest part. But it is also the most boring part.
    * Lua host: How do we expose ECS bindings to lua? We will have to use light user data as entities/ids, as luau doesn't support 64bit integers. We also need to be careful and not create too many temporary objects (increases GC pressure). How do we convert a simple component like `Position3` to luau? use inbuilt Vec3? or an array of floats? more such questions need to be answer for ergonomic bindings. 
    * Lua Guest: This is probably easier, as there's existing flecs-lua project by flecs dev. We can mostly use that as reference. 


# Help needed
### Rust Safety 
My experience with C is limited and I'm not really an unsafe expert in rust. I will try to stick to some basic principes like owning vs borrow, xor mutability, only passing `repr(C)` structs in the ABI etc.., but someone more experienced needs to take a look at the generated bindings and wave a red flag in the issues if there's any soundness issues.
### Ergonomics
Need suggestions on what kind of API would make it easy to play with flecs. Especially the often used Api functions like iterators (query/filter/observer). With wasm, we can't just copy `ecs_iter_t` to wasm memory. First, host pointers are 64bit and wasm pointers are 32bit. Second, what would wasm even do with host pointers? We need to expose a more value-semantics oiented API. Like I said, I'm all ears and would appreciate any kind of advice in this area.




