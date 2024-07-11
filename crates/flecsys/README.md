To select all the constants at once, use `^pub const (ecs|flecs).*;` regex.

These following functions have `usize` in their signature. 
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




## Scripting problems

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

So, avoid passing raw host pointers to guests (or taking them as arguments from guest).

### Copying host data to guest
when we have to copy a few bytes to guest wasm, we need to first allocate memory inside guest. 
Guests should usually expose some extern "C" functions to allocate/deallocate.
We will also be copying a lot of "temporary" data. So, guests can probably export some sort of bump allocator, which we can reset every frame (or at the end of callback/scope). This would be faster.
We would still need normal allocator API tho, when we return pointers that use `ecs_os_free` to explicitly free memory.

### Accessing Host Data
For simple value types like entities or booleans etc.., you can just pass to guest by value.
For strings:
    * For lua like managed languages, just create a new lua string and return that.
    * For wasm, allocate memory on guest and copy the string data. Finally, pass the pointer to guest memory (should be 32bit) to wasm.

For `const void *`, like pointers to an entity's component data (eg: `ecs_get_id`), 
    * The easy method would be to convert the component to json and pass it to guests. mlua already provides json -> lua conversion built-in. wasm can just take a string and deserialize it on guest side. may not be fast enough tho. 
    * We can copy the bytes to guest and provide the pointer (just like strings), then guests can somehow to actually parse the bytes. useful for POD types like `Vec3` or `Point` or `Color` etc.. Although, this won't work when the data is not ABI-stable or might have indirections (eg: HashMap<String, String>).
    * The final over-engineered solution would be to simply store a host component pointer on host side in some arena (`slotmap`?), provide the index of the "host pointer" as "pointer" to the guest. Then, guest can use the meta (flecs reflection) functions, to view/edit data. We will have to make sure that this slot will be cleared or we run into the danger of use-after-free. eg: `ecs_get_id` to store the host pointer in a slot -> `ecs_delete` that entity -> use the slot now. Lifecycle of these mutations needs more consideration.

For mutable `void *`, we need to copy the data from guest to host after some time. 

### Queries/Iterators
This is a great example of mutable access to component data. We will have to copy the bytes in/out, or use meta-cursor objects. The real issue would be the scope of the data.
For simple copying of bytes in/out:
    * when calling `ecs_query_next` to get the table, copy the full bytes of each component array to guest. If the terms are mutable (`out`), store those guest pointers in a slotmap. guest can call some sort of `ecs_data_flush` after writing to the component data, and we can copy those bytes back from guest into host. Then, pop the pointers from slotmap.
For reflected components:
    * I have no idea

