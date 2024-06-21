To generate bindings, change into this directory and use
```sh
bindgen --merge-extern-blocks --default-enum-style rust --no-layout-tests -o bindings.rs flecs.h
```

To select all the constants at once, use `^pub const (ecs|flecs).*;` regex.

These following functions have `usize` in their signature. wasm32 bindings will be interacting with a 64bit host application, so manually change usize to u64 in these fns to make sure that bindings match on both sides.
```rust
ecs_record_get_by_column
ecs_set_id
ecs_field_w_size
ecs_field_size
ecs_table_get_column_size
ecs_cpp_get_type_name
ecs_cpp_get_symbol_name
ecs_cpp_get_constant_name
ecs_cpp_component_validate
ecs_cpp_component_register_explicit
```






For wasm runtimes to interact with your native host ECS, there's a few pitfalls that you need to be aware of.

### Native Sized Integers
`usize` is a footgun as most wasm guests are `wasm32` (32bit) and their usize is usually `u32`.
But hosts are mostly 64bit and their usize is `u64`. 
This leads to a mismatch in C ABI/API. 
For example, look at this fn
```rust
pub fn ecs_set_id(
    world: *mut ecs_world_t,
    entity: ecs_entity_t,
    id: ecs_id_t,
    size: usize,
    ptr: *const ::std::os::raw::c_void,
);
```
When giving `size` argument to wasm guest, it will want a `u32` and the host has a `u64`. 
If we do `fallible` conversion, then its even worse because the value may not actually fit into `u32`.
**fix**: In these cases, just use u64 everywhere in your bindings. (assuming host is always 64bit). This allows wasm bindings to just take u64 and work with that.
**Quirk**: This works fine, until you want a scripting language like javascript as wasm guest. JS only supports `f64` (doubles), so.. 2^53 is the max value of the `u64` it can receive. Its recommended to keep your `u64` values below that and just err if the value goes out of range.

### Entity Id as u64
entity ids are `u64` and pretty much all the API uses entity ids as args or return values.
Its very important for ergonomics that you use runtimes which support using u64 by value.
For example, `luajit` or `v8` use `nan-boxing` with a `f64(double)`and only support integers values up to 2^53. So, they will have to "box" (heap allocate) entity ids.
Fortunately, most guests like rust/c/cpp/go/zig/C# on wasm support u64 type directly. Java supports i64, so that can simply be used by casting the bits. 
lua/luau (roblox lua) also support passing entity ids by value because their `Value` tagged union has a `LightUserData` type with which you can pass pointers around (and pointers are 64bit on host).

### Raw Host Pointers
This is a pain point for any sandboxed guest, because, *usually* guests can't access host memory.
For lua/luau, its easy to pass pointers around using `LightUserData`. And in wasm, you can pass them around as `u64` (just like entity ids). 
The real problem is that we cannot *trust* the guests to do the right thing. Either by accident or malicious intent, they might use the API the wrong way to trigger an attack.
In lua/Luau, guests can't change the pointer value. But they can take a `const char *` and pass it to an API expecting `char *` to modify readonly values. Or they can pass it to an API expecting `* some_ecs_struct_t` with a random length to trigger OOB access/writes.
In wasm, they can simply cast the bits of an integer into a pointer and pass it to your API.

We can resolve this with a few different solutions
1. **fix**: Change the bindings API to return a copy of the data. eg: strings or byte arrays. This requires wasm guests to expose allocators so that you can allocate/write into their memory. This is very useful when the guest need the "data" (bytes of plain old data or strings)
2. **fix**: Wrap the pointers into garbage collectable "owned" objects. eg: resources in wasm. This has a cost via GC pressure. This is more useful when you need to pass "ownership" of a pointer rather than just raw data. The best examples are probably iterators or Vec like containers or even a graphics resource like `GlBuffer`. 
3. **fix**: Keep a generation-based arena (eg: `slotmap`) where you can store the host pointers, and give a key into the arena as 32bit pointer to the guests. In all the API functions where you use the pointers, just get the actual pointer from arena. When its time to remove that pointer, just delete it from the arena, so that when the key is used again, you can simply error. This is transparent to both guests and hosts, so you don't need to change any API. 

**Quirks**: 
1. First method is simple, but can only be used for simpler data.
2. Second method works with ownership, so it can't help you when you are borrowing pointers. eg: a pointer might only be valid within a callback. In these cases, you can try using some sort of "scoped" pattern. `mlua` provides a scope, and any data create inside these scopes will be inaccessible to lua after you return from the scope. Wasm, unfortunately.. has no such functionality.
3. Third method has issues when data is temporary. eg: a pointer only valid inside a callback. We would need to make sure that any time we are exiting a scope, we should remove all pointers that are invalidated. So, if wasm guests try to access a pointer using their key, they will simply fail. Its better to use separate arenas for owned and borrwed pointers. For all APIs dealing with temporary borrows, just clear the temp arena at the end of a function.

### Restricting access with permissions
All the previous problems were technical limitations or workarounds, but this is a logical problem. 
An ECS has entities created by many guests and host.
guests will simply have a id (eg: u32) and entities will have permission components that tell us whether this guest can read/modify it. 
Individual permissions to precisely allow a guest to read/write. 
1. `(ReadableBy, GuestId)`
2. `(WriteableBy, GuestId)`
Public Permissions tags which allow any host/guest to read/write to this entity.
1. `PublicRead`
2. `PublicWrite`

Host API will validate these at runtime so that guests won't touch entities they don't access to. A lot of host entities like `Position` or `Color` would be public readable so that guests can access these components and add them to their entities.