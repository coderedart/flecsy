### Goals
The main goal is to expose a safe API with which one can safely sandbox different guests. 

The order of priorities:
1. Compile times / Dependency count: 
    1. Try to avoid adding dependencies.
    2. If you do add them, only add stable/maintained dependencies.
    3. Prefer lightweight alternatives over "batteries included". Don't let heavy hitters like syn/proc-macro creep in.
    4. If you *really* need a dependency, see if you can feature gate it, so that the rest of us can still have fast compile times. 
2. Convenience:
    1. Try as much as possible to pass data by value
    2. Only use references/borrows in local contexts where you can reason about them with lifetime annotations.
    3. Try to have lots of examples or documentation.
3. Correctness
    1. Don't construct invalid entities or ids. Use newtype patterns to prevent misuse of APIs.
    2. Use as many asserts as you want to make sure invariants hold.
    3. Stick to safe code and don't try to optimize functions that are rarely used (eg: registering components). But definitely try to optimize getting component ids of types, as that would probably be used very often.
4. Performance
    1. Use `binding_ctx` of the world to store bindings related data. Its readily available inside the world.

