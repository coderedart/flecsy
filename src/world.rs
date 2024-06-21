use std::{
    alloc::Layout,
    any::{Any, TypeId},
    ffi::{c_int, CStr, CString, NulError},
    num::{NonZeroU64, TryFromIntError},
    ops::{Deref, DerefMut},
    os::raw::c_void,
    sync::OnceLock,
    vec,
};

use flecsys::*;
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Id(pub NonZeroU64);

impl Id {
    pub fn as_u64(self) -> u64 {
        self.0.into()
    }
}
impl Deref for Id {
    type Target = NonZeroU64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Id> for NonZeroU64 {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl From<Id> for u64 {
    fn from(value: Id) -> Self {
        value.0.into()
    }
}
impl TryFrom<u64> for Id {
    type Error = TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Entity(pub NonZeroU64);

impl Entity {
    pub fn as_u64(self) -> u64 {
        self.0.into()
    }
}
impl Deref for Entity {
    type Target = NonZeroU64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Entity> for NonZeroU64 {
    fn from(value: Entity) -> Self {
        value.0
    }
}
impl From<Entity> for u64 {
    fn from(value: Entity) -> Self {
        value.0.into()
    }
}
impl TryFrom<u64> for Entity {
    type Error = TryFromIntError;
    fn try_from(value: u64) -> Result<Entity, Self::Error> {
        Ok(Entity(value.try_into()?))
    }
}
pub struct Type {
    pub array: *mut Id,
    pub count: i32,
}

/// Owned version of the world
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct World(*mut ecs_world_t);

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Stage(*mut ecs_world_t);
impl Deref for Stage {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(&self.0) }
    }
}
impl DerefMut for Stage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(&mut self.0) }
    }
}
impl Default for World {
    #[doc = " Create a new world.\n This operation automatically imports modules from addons Flecs has been built\n with, except when the module specifies otherwise.\n\n @return A new world"]
    fn default() -> Self {
        Self(unsafe { ecs_init() })
    }
}
impl World {
    #[doc = " Create a new world with just the core module.\n Same as ecs_init(), but doesn't import modules from addons. This operation is\n faster than ecs_init() and results in less memory utilization.\n\n @return A new tiny world"]
    pub fn mini() -> Self {
        Self(unsafe { ecs_mini() })
    }
    #[doc = " Create a new world with arguments.\n Same as ecs_init(), but allows passing in command line arguments. Command line\n arguments are used to:\n - automatically derive the name of the application from argv[0]\n\n @return A new world"]
    pub fn from_args() -> Result<Self, NulError> {
        let args: Result<Vec<CString>, NulError> =
            std::env::args().into_iter().map(CString::new).collect();
        let args = args?;
        let argc = args.len() as i32;
        let argv = args
            .iter()
            .map(CString::as_c_str)
            .map(CStr::as_ptr)
            .collect::<Vec<_>>();
        Ok(Self(unsafe { ecs_init_w_args(argc, argv.as_ptr() as _) }))
    }
    fn as_ptr(&self) -> *const ecs_world_t {
        self.0 as _
    }
    fn as_mut_ptr(&mut self) -> *mut ecs_world_t {
        self.0
    }
    #[doc = " Returns whether the world is being deleted.\n This operation can be used in callbacks like type hooks or observers to\n detect if they are invoked while the world is being deleted.\n\n @param world The world.\n @return True if being deleted, false if not."]
    pub fn is_finishing(&self) -> bool {
        unsafe { ecs_is_fini(self.as_ptr()) }
    }
    unsafe extern "C" fn trampoline_world_userdata<T>(
        mut world: *mut ecs_world_t,
        ctx: *mut std::ffi::c_void,
    ) {
        let (f, data) = *Box::from_raw(ctx as *mut (fn(&mut World, T), T));
        // we must not take ownership of world here.
        let world_mut: &mut World = get_world_mut(&mut world);

        (f)(world_mut, data);
    }
    #[doc = " Register action to be executed when world is destroyed.\n Fini actions are typically used when a module needs to clean up before a\n world shuts down.\n\n @param world The world.\n @param action The function to execute.\n @param data Userdata to pass to the function"]
    pub fn at_finish<T: Send + Sync + 'static + Sized>(
        &mut self,
        action: fn(&mut World, T),
        data: T,
    ) {
        let data = Box::leak(Box::new((action, data))) as *mut (fn(&mut World, T), T);

        unsafe {
            ecs_atfini(
                self.as_mut_ptr(),
                Some(Self::trampoline_world_userdata::<T>),
                data as *mut std::ffi::c_void,
            );
        }
    }

    #[doc = " Begin frame.\n When an application does not use ecs_progress() to control the main loop, it\n can still use Flecs features such as FPS limiting and time measurements. This\n operation needs to be invoked whenever a new frame is about to get processed.\n\n Calls to ecs_frame_begin() must always be followed by ecs_frame_end().\n\n The function accepts a delta_time parameter, which will get passed to\n systems. This value is also used to compute the amount of time the function\n needs to sleep to ensure it does not exceed the target_fps, when it is set.\n When 0 is provided for delta_time, the time will be measured.\n\n This function should only be ran from the main thread.\n\n @param world The world.\n @param delta_time Time elapsed since the last frame.\n @return The provided delta_time, or measured time if 0 was provided."]
    pub fn frame_begin(&mut self, delta_time: f32) -> f32 {
        unsafe { ecs_frame_begin(self.as_mut_ptr(), delta_time) }
    }
    #[doc = " End frame.\n This operation must be called at the end of the frame, and always after\n ecs_frame_begin().\n\n @param world The world."]
    pub fn frame_end(&mut self) {
        unsafe { ecs_frame_end(self.as_mut_ptr()) }
    }

    #[doc = " Register action to be executed once after frame.\n Post frame actions are typically used for calling operations that cannot be\n invoked during iteration, such as changing the number of threads.\n\n @param world The world.\n @param action The function to execute.\n @param ctx Userdata to pass to the function"]
    pub fn run_post_frame<T: Send + Sync + 'static + Sized>(
        &mut self,
        action: fn(&mut World, T),
        data: T,
    ) {
        let data = Box::leak(Box::new((action, data))) as *mut (fn(&mut World, T), T);

        unsafe {
            ecs_run_post_frame(
                self.as_mut_ptr(),
                Some(Self::trampoline_world_userdata::<T>),
                data as *mut std::ffi::c_void,
            );
        }
    }

    #[doc = " Signal exit\n This operation signals that the application should quit. It will cause\n ecs_progress() to return false.\n\n @param world The world to quit."]
    pub fn quit(&mut self) {
        unsafe { ecs_quit(self.as_mut_ptr()) }
    }
    #[doc = " Return whether a quit has been requested.\n\n @param world The world.\n @return Whether a quit has been requested.\n @see ecs_quit()"]
    pub fn should_quit(world: &World) -> bool {
        unsafe { ecs_should_quit(world.as_ptr()) }
    }
    #[doc = " Measure frame time.\n Frame time measurements measure the total time passed in a single frame, and\n how much of that time was spent on systems and on merging.\n\n Frame time measurements add a small constant-time overhead to an application.\n When an application sets a target FPS, frame time measurements are enabled by\n default.\n\n @param world The world.\n @param enable Whether to enable or disable frame time measuring."]
    pub fn measure_frame_time(world: &mut World, enable: bool) {
        unsafe { ecs_measure_frame_time(world.as_mut_ptr(), enable) }
    }
    #[doc = " Measure system time.\n System time measurements measure the time spent in each system.\n\n System time measurements add overhead to every system invocation and\n therefore have a small but measurable impact on application performance.\n System time measurements must be enabled before obtaining system statistics.\n\n @param world The world.\n @param enable Whether to enable or disable system time measuring."]
    pub fn measure_system_time(world: &mut World, enable: bool) {
        unsafe { ecs_measure_system_time(world.as_mut_ptr(), enable) }
    }
    #[doc = " Set target frames per second (FPS) for application.\n Setting the target FPS ensures that ecs_progress() is not invoked faster than\n the specified FPS. When enabled, ecs_progress() tracks the time passed since\n the last invocation, and sleeps the remaining time of the frame (if any).\n\n This feature ensures systems are ran at a consistent interval, as well as\n conserving CPU time by not running systems more often than required.\n\n Note that ecs_progress() only sleeps if there is time left in the frame. Both\n time spent in flecs as time spent outside of flecs are taken into\n account.\n\n @param world The world.\n @param fps The target FPS."]
    pub fn set_target_fps(world: &mut World, fps: f32) {
        unsafe { ecs_set_target_fps(world.as_mut_ptr(), fps) }
    }
    #[doc = " Begin readonly mode.\n This operation puts the world in readonly mode, which disallows mutations on\n the world. Readonly mode exists so that internal mechanisms can implement\n optimizations that certain aspects of the world to not change, while also\n providing a mechanism for applications to prevent accidental mutations in,\n for example, multithreaded applications.\n\n Readonly mode is a stronger version of deferred mode. In deferred mode\n ECS operations such as add/remove/set/delete etc. are added to a command\n queue to be executed later. In readonly mode, operations that could break\n scheduler logic (such as creating systems, queries) are also disallowed.\n\n Readonly mode itself has a single threaded and a multi threaded mode. In\n single threaded mode certain mutations on the world are still allowed, for\n example:\n - Entity liveliness operations (such as new, make_alive), so that systems are\n   able to create new entities.\n - Implicit component registration, so that this works from systems\n - Mutations to supporting data structures for the evaluation of uncached\n   queries (filters), so that these can be created on the fly.\n\n These mutations are safe in a single threaded applications, but for\n multithreaded applications the world needs to be entirely immutable. For this\n purpose multi threaded readonly mode exists, which disallows all mutations on\n the world. This means that in multi threaded applications, entity liveliness\n operations, implicit component registration, and on-the-fly query creation\n are not guaranteed to work.\n\n While in readonly mode, applications can still enqueue ECS operations on a\n stage. Stages are managed automatically when using the pipeline addon and\n ecs_progress(), but they can also be configured manually as shown here:\n\n @code\n // Number of stages typically corresponds with number of threads\n ecs_set_stage_count(world, 2);\n ecs_stage_t *stage = ecs_get_stage(world, 1);\n\n ecs_readonly_begin(world);\n ecs_add(world, e, Tag); // readonly assert\n ecs_add(stage, e, Tag); // OK\n @endcode\n\n When an attempt is made to perform an operation on a world in readonly mode,\n the code will throw an assert saying that the world is in readonly mode.\n\n A call to ecs_readonly_begin() must be followed up with ecs_readonly_end().\n When ecs_readonly_end() is called, all enqueued commands from configured\n stages are merged back into the world. Calls to ecs_readonly_begin() and\n ecs_readonly_end() should always happen from a context where the code has\n exclusive access to the world. The functions themselves are not thread safe.\n\n In a typical application, a (non-exhaustive) call stack that uses\n ecs_readonly_begin() and ecs_readonly_end() will look like this:\n\n @code\n ecs_progress()\n   ecs_readonly_begin()\n     ecs_defer_begin()\n\n       // user code\n\n   ecs_readonly_end()\n     ecs_defer_end()\n@endcode\n\n @param world The world\n @param multi_threaded Whether to enable readonly/multi threaded mode.\n @return Whether world is in readonly mode."]
    pub fn ecs_readonly_begin(world: &mut World, multi_threaded: bool) -> bool {
        unsafe { ecs_readonly_begin(world.as_mut_ptr(), multi_threaded) }
    }
    #[doc = " End readonly mode.\n This operation ends readonly mode, and must be called after\n ecs_readonly_begin(). Operations that were deferred while the world was in\n readonly mode will be flushed.\n\n @param world The world"]
    pub fn ecs_readonly_end(world: &mut World) {
        unsafe { ecs_readonly_end(world.as_mut_ptr()) }
    }
    #[doc = " Merge world or stage.\n When automatic merging is disabled, an application can call this\n operation on either an individual stage, or on the world which will merge\n all stages. This operation may only be called when staging is not enabled\n (either after ecs_progress() or after ecs_readonly_end()).\n\n This operation may be called on an already merged stage or world.\n\n @param world The world."]
    pub fn ecs_merge(world: &mut World) {
        unsafe { ecs_merge(world.as_mut_ptr()) }
    }
    #[doc = " Defer operations until end of frame.\n When this operation is invoked while iterating, operations inbetween the\n ecs_defer_begin() and ecs_defer_end() operations are executed at the end\n of the frame.\n\n This operation is thread safe.\n\n @param world The world.\n @return true if world changed from non-deferred mode to deferred mode."]
    pub fn ecs_defer_begin(world: &mut World) -> bool {
        unsafe { ecs_defer_begin(world.as_mut_ptr()) }
    }
    #[doc = " Test if deferring is enabled for current stage.\n\n @param world The world.\n @return True if deferred, false if not."]
    pub fn ecs_is_deferred(world: &World) -> bool {
        unsafe { ecs_is_deferred(world.as_ptr()) }
    }
    #[doc = " End block of operations to defer.\n See ecs_defer_begin().\n\n This operation is thread safe.\n\n @param world The world.\n @return true if world changed from deferred mode to non-deferred mode."]
    pub fn ecs_defer_end(world: &mut World) -> bool {
        unsafe { ecs_defer_end(world.as_mut_ptr()) }
    }
    #[doc = " Suspend deferring but do not flush queue.\n This operation can be used to do an undeferred operation while not flushing\n the operations in the queue.\n\n An application should invoke ecs_defer_resume() before ecs_defer_end() is called.\n The operation may only be called when deferring is enabled.\n\n @param world The world."]
    pub fn ecs_defer_suspend(world: &mut World) {
        unsafe { ecs_defer_suspend(world.as_mut_ptr()) }
    }
    #[doc = " Resume deferring.\n See ecs_defer_suspend().\n\n @param world The world."]
    pub fn ecs_defer_resume(world: &mut World) {
        unsafe { ecs_defer_resume(world.as_mut_ptr()) }
    }

    #[doc = " Configure world to have N stages.\n This initializes N stages, which allows applications to defer operations to\n multiple isolated defer queues. This is typically used for applications with\n multiple threads, where each thread gets its own queue, and commands are\n merged when threads are synchronized.\n\n Note that the ecs_set_threads() function already creates the appropriate\n number of stages. The ecs_set_stage_count() operation is useful for applications\n that want to manage their own stages and/or threads.\n\n @param world The world.\n @param stages The number of stages."]
    pub fn set_stage_count(&mut self, stages: i32) {
        unsafe { ecs_set_stage_count(self.as_mut_ptr(), stages) }
    }
    #[doc = " Get number of configured stages.\n Return number of stages set by ecs_set_stage_count().\n\n @param world The world.\n @return The number of stages used for threading."]
    pub fn get_stage_count(&self) -> i32 {
        unsafe { ecs_get_stage_count(self.as_ptr()) }
    }
    #[doc = " Get stage-specific world pointer.\n Flecs threads can safely invoke the API as long as they have a private\n context to write to, also referred to as the stage. This function returns a\n pointer to a stage, disguised as a world pointer.\n\n Note that this function does not(!) create a new world. It simply wraps the\n existing world in a thread-specific context, which the API knows how to\n unwrap. The reason the stage is returned as an ecs_world_t is so that it\n can be passed transparently to the existing API functions, vs. having to\n create a dedicated API for threading.\n\n @param world The world.\n @param stage_id The index of the stage to retrieve.\n @return A thread-specific pointer to the world."]
    pub fn get_stage(&self, stage_id: i32) -> Stage {
        unsafe { Stage(ecs_get_stage(self.as_ptr(), stage_id)) }
    }
    #[doc = " Test whether the current world is readonly.\n This function allows the code to test whether the currently used world\n is readonly or whether it allows for writing.\n\n @param world A pointer to a stage or the world.\n @return True if the world or stage is readonly."]
    pub fn stage_is_readonly(&self) -> bool {
        unsafe { ecs_stage_is_readonly(self.as_ptr()) }
    }
    #[doc = " Create unmanaged stage.\n Create a stage who's lifecycle is not managed by the world. Must be freed\n with ecs_stage_free.\n\n @param world The world.\n @return The stage."]
    pub unsafe fn stage_new(&mut self) -> Stage {
        unsafe { Stage(ecs_stage_new(self.as_mut_ptr())) }
    }
    #[doc = " Free unmanaged stage.\n\n @param stage The stage to free."]
    pub unsafe fn stage_free(mut stage: Stage) {
        unsafe { ecs_stage_free(stage.as_mut_ptr()) }
    }
    #[doc = " Get stage id.\n The stage id can be used by an application to learn about which stage it is\n using, which typically corresponds with the worker thread id.\n\n @param world The world.\n @return The stage id."]
    pub fn stage_get_id(&self) -> i32 {
        unsafe { ecs_stage_get_id(self.as_ptr()) }
    }
    unsafe extern "C" fn drop_trampoline_userdata(ctx: *mut std::ffi::c_void) {
        let _ = Box::from_raw(ctx as *mut Box<dyn Any>);
    }
}
// TODO: check if this is worth it for performance or just stick to Box<Box<dyn Any>> for ctx
// code was provided by `yandros` from rust community discord. msg link: https://discord.com/channels/273534239310479360/818964227783262209/1252679646856613952
pub trait Bounds: 'static + Send + Sync {}
impl<T> Bounds for T where Self: 'static + Send + Sync {}

#[repr(C)]
pub struct InlineTypeId<T: Bounds = ::core::marker::PhantomData<dyn Bounds>> {
    type_id: TypeId,
    data: T,
}

impl<T: Bounds> InlineTypeId<T> {
    pub fn thin_boxed(data: T) -> *mut InlineTypeId {
        Box::into_raw(Box::new(InlineTypeId {
            type_id: TypeId::of::<T>(),
            data,
        }))
        .cast()
    }
}
impl World {
    #[doc = " Set a world context.\n This operation allows an application to register custom data with a world\n that can be accessed anywhere where the application has the world.\n\n @param world The world.\n @param ctx A pointer to a user defined structure.\n @param ctx_free A function that is invoked with ctx when the world is freed."]
    pub fn set_ctx<T: 'static + Send + Sync + Any>(world: &mut World, ctx: T) {
        let ctx: Box<dyn Any> = Box::new(ctx);

        let ctx = Box::leak(Box::new(ctx)) as *mut _;
        unsafe {
            ecs_set_ctx(
                world.as_mut_ptr(),
                ctx as *mut ::std::os::raw::c_void,
                Some(Self::drop_trampoline_userdata),
            )
        }
    }
    #[doc = " Set a world binding context.\n Same as ecs_set_ctx() but for binding context. A binding context is intended\n specifically for language bindings to store binding specific data.\n\n @param world The world.\n @param ctx A pointer to a user defined structure.\n @param ctx_free A function that is invoked with ctx when the world is freed."]
    pub fn set_binding_ctx<T: 'static + Send + Sync + Any>(world: &mut World, ctx: T) {
        let ctx: Box<dyn Any> = Box::new(ctx);
        let ctx = Box::leak(Box::new(ctx)) as *mut _;
        unsafe {
            ecs_set_binding_ctx(
                world.as_mut_ptr(),
                ctx as *mut ::std::os::raw::c_void,
                Some(Self::drop_trampoline_userdata),
            )
        }
    }
    #[doc = " Get the world context.\n This operation retrieves a previously set world context.\n\n @param world The world.\n @return The context set with ecs_set_ctx(). If no context was set, the\n         function returns NULL."]
    pub fn get_ctx<T: 'static + Send + Sync + Any>(world: &World) -> Option<&T> {
        unsafe {
            (ecs_get_ctx(world.as_ptr()) as *mut Box<dyn Any>)
                .as_ref()?
                .downcast_ref()
        }
    }
    #[doc = " Get the world binding context.\n This operation retrieves a previously set world binding context.\n\n @param world The world.\n @return The context set with ecs_set_binding_ctx(). If no context was set, the\n         function returns NULL."]
    pub fn get_binding_ctx<T: 'static + Send + Sync + Any>(world: &World) -> Option<&T> {
        unsafe {
            (ecs_get_binding_ctx(world.as_ptr()) as *mut Box<dyn Any>)
                .as_ref()?
                .downcast_ref()
        }
    }
    #[doc = " Get build info.\n  Returns information about the current Flecs build.\n\n @return A struct with information about the current Flecs build."]
    pub fn get_build_info() -> &'static BuildInfo {
        BuildInfo::get_static()
    }
    #[doc = " Get world info.\n\n @param world The world.\n @return Pointer to the world info. Valid for as long as the world exists."]
    pub fn get_world_info(world: &World) -> WorldInfo {
        let info_ptr = unsafe { ecs_get_world_info(world.as_ptr()) };
        assert!(!info_ptr.is_null());

        let info = unsafe { &*info_ptr };

        WorldInfo {
            last_component_id: Entity(info.last_component_id.try_into().unwrap()),
            min_id: Entity(info.min_id.try_into().unwrap()),
            max_id: Entity(info.max_id.try_into().unwrap()),
            delta_time_raw: info.delta_time_raw,
            delta_time: info.delta_time,
            time_scale: info.time_scale,
            target_fps: info.target_fps,
            frame_time_total: info.frame_time_total,
            system_time_total: info.system_time_total,
            emit_time_total: info.emit_time_total,
            merge_time_total: info.merge_time_total,
            world_time_total: info.world_time_total,
            world_time_total_raw: info.world_time_total_raw,
            rematch_time_total: info.rematch_time_total,
            frame_count_total: info.frame_count_total,
            merge_count_total: info.merge_count_total,
            rematch_count_total: info.rematch_count_total,
            id_create_total: info.id_create_total,
            id_delete_total: info.id_delete_total,
            table_create_total: info.table_create_total,
            table_delete_total: info.table_delete_total,
            pipeline_build_count_total: info.pipeline_build_count_total,
            systems_ran_frame: info.systems_ran_frame,
            observers_ran_frame: info.observers_ran_frame,
            tag_id_count: info.tag_id_count,
            component_id_count: info.component_id_count,
            pair_id_count: info.pair_id_count,
            table_count: info.table_count,
            empty_table_count: info.empty_table_count,
            add_count: info.cmd.add_count,
            remove_count: info.cmd.remove_count,
            delete_count: info.cmd.delete_count,
            clear_count: info.cmd.clear_count,
            set_count: info.cmd.set_count,
            ensure_count: info.cmd.ensure_count,
            modified_count: info.cmd.modified_count,
            discard_count: info.cmd.discard_count,
            event_count: info.cmd.event_count,
            other_count: info.cmd.other_count,
            batched_entity_count: info.cmd.batched_entity_count,
            batched_command_count: info.cmd.batched_command_count,
            // check for nullptr
            name_prefix: (!info.name_prefix.is_null())
                .then(|| {
                    unsafe { CStr::from_ptr(info.name_prefix) }
                        .to_str()
                        .unwrap()
                })
                .unwrap_or(""),
        }
    }
    #[doc = " Dimension the world for a specified number of entities.\n This operation will preallocate memory in the world for the specified number\n of entities. Specifying a number lower than the current number of entities in\n the world will have no effect.\n\n @param world The world.\n @param entity_count The number of entities to preallocate."]
    pub fn dim(world: &mut World, entity_count: i32) {
        unsafe { ecs_dim(world.as_mut_ptr(), entity_count) }
    }
    #[doc = " Set a range for issuing new entity ids.\n This function constrains the entity identifiers returned by ecs_new_w() to the\n specified range. This operation can be used to ensure that multiple processes\n can run in the same simulation without requiring a central service that\n coordinates issuing identifiers.\n\n If id_end is set to 0, the range is infinite. If id_end is set to a non-zero\n value, it has to be larger than id_start. If id_end is set and ecs_new is\n invoked after an id is issued that is equal to id_end, the application will\n abort.\n\n @param world The world.\n @param id_start The start of the range.\n @param id_end The end of the range."]
    pub fn ecs_set_entity_range(world: &mut World, id_start: Entity, id_end: Option<Entity>) {
        if let Some(id_end) = id_end {
            assert!(id_end.0 > id_start.0);
        }
        unsafe {
            ecs_set_entity_range(
                world.as_mut_ptr(),
                id_start.0.into(),
                id_end.map(Into::into).unwrap_or(0),
            )
        }
    }
    #[doc = " Enable/disable range limits.\n When an application is both a receiver of range-limited entities and a\n producer of range-limited entities, range checking needs to be temporarily\n disabled when inserting received entities. Range checking is disabled on a\n stage, so setting this value is thread safe.\n\n @param world The world.\n @param enable True if range checking should be enabled, false to disable.\n @return The previous value."]
    pub fn ecs_enable_range_check(world: &mut World, enable: bool) -> bool {
        unsafe { ecs_enable_range_check(world.as_mut_ptr(), enable) }
    }
    #[doc = " Get the largest issued entity id (not counting generation).\n\n @param world The world.\n @return The largest issued entity id."]
    pub fn ecs_get_max_id(world: &World) -> Entity {
        unsafe { Entity(ecs_get_max_id(world.as_ptr()).try_into().unwrap()) }
    }
    #[doc = " Force aperiodic actions.\n The world may delay certain operations until they are necessary for the\n application to function correctly. This may cause observable side effects\n such as delayed triggering of events, which can be inconvenient when for\n example running a test suite.\n\n The flags parameter specifies which aperiodic actions to run. Specify 0 to\n run all actions. Supported flags start with 'EcsAperiodic'. Flags identify\n internal mechanisms and may change unannounced.\n\n @param world The world.\n @param flags The flags specifying which actions to run."]
    pub fn ecs_run_aperiodic(world: &mut World, flags: AperiodicFlags) {
        unsafe { ecs_run_aperiodic(world.as_mut_ptr(), flags.bits()) }
    }

    #[doc = " Cleanup empty tables.\n This operation cleans up empty tables that meet certain conditions. Having\n large amounts of empty tables does not negatively impact performance of the\n ECS, but can take up considerable amounts of memory, especially in\n applications with many components, and many components per entity.\n\n The generation specifies the minimum number of times this operation has\n to be called before an empty table is cleaned up. If a table becomes non\n empty, the generation is reset.\n\n The operation allows for both a \"clear\" generation and a \"delete\"\n generation. When the clear generation is reached, the table's\n resources are freed (like component arrays) but the table itself is not\n deleted. When the delete generation is reached, the empty table is deleted.\n\n By specifying a non-zero id the cleanup logic can be limited to tables with\n a specific (component) id. The operation will only increase the generation\n count of matching tables.\n\n The min_id_count specifies a lower bound for the number of components a table\n should have. Often the more components a table has, the more specific it is\n and therefore less likely to be reused.\n\n The time budget specifies how long the operation should take at most.\n\n @param world The world.\n @param id Optional component filter for the tables to evaluate.\n @param clear_generation Free table data when generation > clear_generation.\n @param delete_generation Delete table when generation > delete_generation.\n @param min_id_count Minimum number of component ids the table should have.\n @param time_budget_seconds Amount of time operation is allowed to spend.\n @return Number of deleted tables."]
    pub fn ecs_delete_empty_tables(
        &mut self,
        id: Option<Entity>,
        clear_generation: u16,
        delete_generation: u16,
        min_id_count: i32,
        time_budget_seconds: f64,
    ) -> i32 {
        let id = id.map(Entity::as_u64).unwrap_or(0);
        unsafe {
            ecs_delete_empty_tables(
                self.as_mut_ptr(),
                id,
                clear_generation,
                delete_generation,
                min_id_count,
                time_budget_seconds,
            )
        }
    }
    /*

    #[doc = " Get world from poly.\n\n @param poly A pointer to a poly object.\n @return The world."]
    pub fn ecs_get_world(poly: *const flecs_poly_t) -> *const ecs_world_t;
    #[doc = " Get entity from poly.\n\n @param poly A pointer to a poly object.\n @return Entity associated with the poly object."]
    pub fn ecs_get_entity(poly: *const flecs_poly_t) -> ecs_entity_t;
     */

    /// Make a pair id.
    ///
    /// This function is equivalent to using the `ecs_pair!` macro, and is added for
    /// convenience to make it easier for non C/C++ bindings to work with pairs.
    ///
    /// # Arguments
    ///
    /// * `first` - The first element of the pair.
    /// * `second` - The target of the pair.
    ///
    /// # Returns
    ///
    /// A pair id.
    pub fn make_pair(first: Entity, second: Entity) -> Id {
        unsafe { ecs_make_pair(first.as_u64(), second.as_u64()) }
            .try_into()
            .unwrap()
    }

    /// Create new entity id.
    ///
    /// This operation returns an unused entity id. This operation is guaranteed to
    /// return an empty entity as it does not use values set by `ecs_set_scope()` or
    /// `ecs_set_with()`.
    ///
    /// # Arguments
    ///
    /// * `world` - The world.
    ///
    /// # Returns
    ///
    /// The new entity id.
    pub fn new(world: &mut World) -> Entity {
        unsafe { ecs_new(world.as_mut_ptr()) }.try_into().unwrap()
    }

    /// Create new low id.
    ///
    /// This operation returns a new low id. Entity ids start after the
    /// [FLECS_HI_COMPONENT_ID] constant. This reserves a range of low ids for things
    /// like components, and allows parts of the code to optimize operations.
    ///
    /// Note that `FLECS_HI_COMPONENT_ID` does not represent the maximum number of
    /// components that can be created, only the maximum number of components that
    /// can take advantage of these optimizations.
    ///
    /// This operation is guaranteed to return an empty entity as it does not use
    /// values set by `ecs_set_scope()` or `ecs_set_with()`.
    ///
    /// This operation does not recycle ids.
    ///
    /// # Arguments
    ///
    /// * `world` - The world.
    ///
    /// # Returns
    ///
    /// The new component id.
    pub fn new_low_id(world: &mut World) -> Id {
        unsafe { ecs_new_low_id(world.as_mut_ptr()) }
            .try_into()
            .unwrap()
    }

    /// Create new entity with (component) id.
    ///
    /// This operation creates a new entity with an optional (component) id. When 0
    /// is passed to the id parameter, no component is added to the new entity.
    ///
    /// # Arguments
    ///
    /// * `world` - The world.
    /// * `id` - The component id to initialize the new entity with.
    ///
    /// # Returns
    ///
    /// The new entity.
    pub fn new_with_id(world: &mut World, id: Option<Id>) -> Entity {
        unsafe { ecs_new_w_id(world.as_mut_ptr(), id.map(Into::into).unwrap_or_default()) }
            .try_into()
            .unwrap()
    }
    #[doc = " Create new entity in table.\n This operation creates a new entity in the specified table.\n\n @param world The world.\n @param table The table to which to add the new entity.\n @return The new entity."]
    pub fn new_w_table(world: *mut ecs_world_t, table: &mut Table) -> Entity {
        unsafe { ecs_new_w_table(world, table.as_mut_ptr()) }
            .try_into()
            .unwrap()
    }
    /*
    #[doc = " Find or create an entity.\n This operation creates a new entity, or modifies an existing one. When a name\n is set in the ecs_entity_desc_t::name field and ecs_entity_desc_t::entity is\n not set, the operation will first attempt to find an existing entity by that\n name. If no entity with that name can be found, it will be created.\n\n If both a name and entity handle are provided, the operation will check if\n the entity name matches with the provided name. If the names do not match,\n the function will fail and return 0.\n\n If an id to a non-existing entity is provided, that entity id become alive.\n\n See the documentation of ecs_entity_desc_t for more details.\n\n @param world The world.\n @param desc Entity init parameters.\n @return A handle to the new or existing entity, or 0 if failed."]
    pub fn ecs_entity_init(world: &mut World, desc: &EntityDesc) -> Entity {
        unsafe { ecs_entity_init(world.as_mut_ptr(), desc.as_ptr()) }
            .try_into()
            .unwrap()
    }
    #[doc = " Bulk create/populate new entities.\n This operation bulk inserts a list of new or predefined entities into a\n single table.\n\n The operation does not take ownership of component arrays provided by the\n application. Components that are non-trivially copyable will be moved into\n the storage.\n\n The operation will emit OnAdd events for each added id, and OnSet events for\n each component that has been set.\n\n If no entity ids are provided by the application, the returned array of ids\n points to an internal data structure which changes when new entities are\n created/deleted.\n\n If as a result of the operation triggers are invoked that deletes\n entities and no entity ids were provided by the application, the returned\n array of identifiers may be incorrect. To avoid this problem, an application\n can first call ecs_bulk_init() to create empty entities, copy the array to one\n that is owned by the application, and then use this array to populate the\n entities.\n\n @param world The world.\n @param desc Bulk creation parameters.\n @return Array with the list of entity ids created/populated."]
    pub fn ecs_bulk_init(
        world: *mut ecs_world_t,
        desc: *const ecs_bulk_desc_t,
    ) -> *const ecs_entity_t;
    */

    #[doc = " Create N new entities.\n This operation is the same as ecs_new_w_id(), but creates N entities\n instead of one.\n\n @param world The world.\n @param id The component id to create the entities with.\n @param count The number of entities to create.\n @return The first entity id of the newly created entities."]
    pub fn bulk_new_w_id(world: &mut World, id: Option<Id>, count: u32) -> &[Entity] {
        unsafe {
            let entities = ecs_bulk_new_w_id(
                world.as_mut_ptr(),
                id.map(Into::into).unwrap_or_default(),
                count as _,
            );
            let array = std::slice::from_raw_parts(entities, count as _);
            // verify that they are all non-zeroes. this is a safety precaution
            assert!(array
                .into_iter()
                .copied()
                .map(TryInto::try_into)
                .all(|r: Result<Entity, _>| r.is_ok()));
            // now safely transmute to &[Entity]. both layouts are the same.
            std::mem::transmute(array)
        }
    }
    #[doc = " Clone an entity\n This operation clones the components of one entity into another entity. If\n no destination entity is provided, a new entity will be created. Component\n values are not copied unless copy_value is true.\n\n If the source entity has a name, it will not be copied to the destination\n entity. This is to prevent having two entities with the same name under the\n same parent, which is not allowed.\n\n @param world The world.\n @param dst The entity to copy the components to.\n @param src The entity to copy the components from.\n @param copy_value If true, the value of components will be copied to dst.\n @return The destination entity."]
    pub fn clone(world: &mut World, dst: Entity, src: Entity, copy_value: bool) -> Entity {
        unsafe { ecs_clone(world.as_mut_ptr(), dst.into(), src.into(), copy_value) }
            .try_into()
            .unwrap()
    }
    #[doc = " Delete an entity.\n This operation will delete an entity and all of its components. The entity id\n will be made available for recycling. If the entity passed to ecs_delete() is\n not alive, the operation will have no side effects.\n\n @param world The world.\n @param entity The entity."]
    pub fn delete(world: &mut World, entity: Entity) {
        unsafe { ecs_delete(world.as_mut_ptr(), entity.into()) }
    }
    #[doc = " Delete all entities with the specified id.\n This will delete all entities (tables) that have the specified id. The id\n may be a wildcard and/or a pair.\n\n @param world The world.\n @param id The id."]
    pub fn delete_with(world: &mut World, id: Id) {
        unsafe { ecs_delete_with(world.as_mut_ptr(), id.into()) }
    }
    #[doc = " Add a (component) id to an entity.\n This operation adds a single (component) id to an entity. If the entity\n already has the id, this operation will have no side effects.\n\n @param world The world.\n @param entity The entity.\n @param id The id to add."]
    pub fn add_id(world: &mut World, entity: Entity, id: Id) {
        unsafe { ecs_add_id(world.as_mut_ptr(), entity.into(), id.into()) }
    }
    #[doc = " Remove a (component) id from an entity.\n This operation removes a single (component) id to an entity. If the entity\n does not have the id, this operation will have no side effects.\n\n @param world The world.\n @param entity The entity.\n @param id The id to remove."]
    pub fn remove_id(world: &mut World, entity: Entity, id: Id) {
        unsafe { ecs_remove_id(world.as_mut_ptr(), entity.into(), id.into()) }
    }
    #[doc = " Add auto override for (component) id.\n An auto override is a component that is automatically added to an entity when\n it is instantiated from a prefab. Auto overrides are added to the entity that\n is inherited from (usually a prefab). For example:\n\n @code\n ecs_entity_t prefab = ecs_insert(world,\n   ecs_value(Position, {10, 20}),\n   ecs_value(Mass, {100}));\n\n ecs_auto_override(world, prefab, Position);\n\n ecs_entity_t inst = ecs_new_w_pair(world, EcsIsA, prefab);\n assert(ecs_owns(world, inst, Position)); // true\n assert(ecs_owns(world, inst, Mass)); // false\n @endcode\n\n An auto override is equivalent to a manual override:\n\n @code\n ecs_entity_t prefab = ecs_insert(world,\n   ecs_value(Position, {10, 20}),\n   ecs_value(Mass, {100}));\n\n ecs_entity_t inst = ecs_new_w_pair(world, EcsIsA, prefab);\n assert(ecs_owns(world, inst, Position)); // false\n ecs_add(world, inst, Position); // manual override\n assert(ecs_owns(world, inst, Position)); // true\n assert(ecs_owns(world, inst, Mass)); // false\n @endcode\n\n This operation is equivalent to manually adding the id with the AUTO_OVERRIDE\n bit applied:\n\n @code\n ecs_add_id(world, entity, ECS_AUTO_OVERRIDE | id);\n @endcode\n\n When a component is overridden and inherited from a prefab, the value from\n the prefab component is copied to the instance. When the component is not\n inherited from a prefab, it is added to the instance as if using ecs_add_id.\n\n Overriding is the default behavior on prefab instantiation. Auto overriding\n is only useful for components with the (OnInstantiate, Inherit) trait.\n When a component has the (OnInstantiate, DontInherit) trait and is overridden\n the component is added, but the value from the prefab will not be copied.\n\n @param world The world.\n @param entity The entity.\n @param id The (component) id to auto override."]
    pub fn auto_override_id(world: &mut World, entity: Entity, id: Id) {
        unsafe { ecs_auto_override_id(world.as_mut_ptr(), entity.into(), id.into()) }
    }
    #[doc = " Clear all components.\n This operation will remove all components from an entity.\n\n @param world The world.\n @param entity The entity."]
    pub fn clear(world: &mut World, entity: Entity) {
        unsafe { ecs_clear(world.as_mut_ptr(), entity.into()) }
    }
    #[doc = " Remove all instances of the specified (component) id.\n This will remove the specified id from all entities (tables). The id may be\n a wildcard and/or a pair.\n\n @param world The world.\n @param id The id."]
    pub fn remove_all(world: &mut World, id: Id) {
        unsafe { ecs_remove_all(world.as_mut_ptr(), id.into()) }
    }
    #[doc = " Set current with id.\n New entities are automatically created with the specified id.\n\n @param world The world.\n @param id The id.\n @return The previous id."]
    pub fn set_with(world: &mut World, id: Id) -> Id {
        unsafe { ecs_set_with(world.as_mut_ptr(), id.into()) }
            .try_into()
            .unwrap()
    }
    #[doc = " Get current with id.\n Get the id set with ecs_set_with().\n\n @param world The world.\n @return The last id provided to ecs_set_with()."]
    pub fn get_with(world: &World) -> Id {
        unsafe { ecs_get_with(world.as_ptr()).try_into().unwrap() }
    }
    #[doc = " Enable or disable entity.\n This operation enables or disables an entity by adding or removing the\n EcsDisabled tag. A disabled entity will not be matched with any systems,\n unless the system explicitly specifies the EcsDisabled tag.\n\n @param world The world.\n @param entity The entity to enable or disable.\n @param enabled true to enable the entity, false to disable."]
    pub fn enable(world: &mut World, entity: Entity, enabled: bool) {
        unsafe { ecs_enable(world.as_mut_ptr(), entity.into(), enabled) }
    }
    #[doc = " Enable or disable component.\n Enabling or disabling a component does not add or remove a component from an\n entity, but prevents it from being matched with queries. This operation can\n be useful when a component must be temporarily disabled without destroying\n its value. It is also a more performant operation for when an application\n needs to add/remove components at high frequency, as enabling/disabling is\n cheaper than a regular add or remove.\n\n @param world The world.\n @param entity The entity.\n @param id The component.\n @param enable True to enable the component, false to disable."]
    pub fn enable_id(world: &mut World, entity: Entity, id: Id, enable: bool) {
        unsafe { ecs_enable_id(world.as_mut_ptr(), entity.into(), id.into(), enable) }
    }
    #[doc = " Test if component is enabled.\n Test whether a component is currently enabled or disabled. This operation\n will return true when the entity has the component and if it has not been\n disabled by ecs_enable_component().\n\n @param world The world.\n @param entity The entity.\n @param id The component.\n @return True if the component is enabled, otherwise false."]
    pub fn is_enabled_id(world: &World, entity: Entity, id: Id) -> bool {
        unsafe { ecs_is_enabled_id(world.as_ptr(), entity.into(), id.into()) }
    }
    /*


    #[doc = " Get an immutable pointer to a component.\n This operation obtains a const pointer to the requested component. The\n operation accepts the component entity id.\n\n This operation can return inherited components reachable through an IsA\n relationship.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component to get.\n @return The component pointer, NULL if the entity does not have the component."]
    pub fn ecs_get_id(
        world: *const ecs_world_t,
        entity: ecs_entity_t,
        id: ecs_id_t,
    ) -> *const ::std::os::raw::c_void;
    #[doc = " Get a mutable pointer to a component.\n This operation obtains a mutable pointer to the requested component. The\n operation accepts the component entity id.\n\n Unlike ecs_get_id, this operation does not return inherited components.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component to get.\n @return The component pointer, NULL if the entity does not have the component."]
    pub fn ecs_get_mut_id(
        world: *const ecs_world_t,
        entity: ecs_entity_t,
        id: ecs_id_t,
    ) -> *mut ::std::os::raw::c_void;
    #[doc = " Get a mutable pointer to a component.\n This operation returns a mutable pointer to a component. If the component did\n not yet exist, it will be added.\n\n If ensure is called when the world is in deferred/readonly mode, the\n function will:\n - return a pointer to a temp storage if the component does not yet exist, or\n - return a pointer to the existing component if it exists\n\n @param world The world.\n @param entity The entity.\n @param id The entity id of the component to obtain.\n @return The component pointer."]
    pub fn ecs_ensure_id(
        world: *mut ecs_world_t,
        entity: ecs_entity_t,
        id: ecs_id_t,
    ) -> *mut ::std::os::raw::c_void;
    #[doc = " Combines ensure + modified in single operation.\n This operation is a more efficient alternative to calling ecs_ensure_id() and\n ecs_modified_id() separately. This operation is only valid when the world is in\n deferred mode, which ensures that the Modified event is not emitted before\n the modification takes place.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component to obtain.\n @return The component pointer."]
    pub fn ecs_ensure_modified_id(
        world: *mut ecs_world_t,
        entity: ecs_entity_t,
        id: ecs_id_t,
    ) -> *mut ::std::os::raw::c_void;
    #[doc = " Create a component ref.\n A ref is a handle to an entity + component which caches a small amount of\n data to reduce overhead of repeatedly accessing the component. Use\n ecs_ref_get() to get the component data.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component.\n @return The reference."]
    pub fn ecs_ref_init_id(
        world: *const ecs_world_t,
        entity: ecs_entity_t,
        id: ecs_id_t,
    ) -> ecs_ref_t;
    #[doc = " Get component from ref.\n Get component pointer from ref. The ref must be created with ecs_ref_init().\n\n @param world The world.\n @param ref The ref.\n @param id The component id.\n @return The component pointer, NULL if the entity does not have the component."]
    pub fn ecs_ref_get_id(
        world: *const ecs_world_t,
        ref_: *mut ecs_ref_t,
        id: ecs_id_t,
    ) -> *mut ::std::os::raw::c_void;
    #[doc = " Update ref.\n Ensures contents of ref are up to date. Same as ecs_ref_get_id(), but does not\n return pointer to component id.\n\n @param world The world.\n @param ref The ref."]
    pub fn ecs_ref_update(world: *const ecs_world_t, ref_: *mut ecs_ref_t);
    #[doc = " Find record for entity.\n An entity record contains the table and row for the entity.\n\n @param world The world.\n @param entity The entity.\n @return The record, NULL if the entity does not exist."]
    pub fn ecs_record_find(world: *const ecs_world_t, entity: ecs_entity_t) -> *mut ecs_record_t;
    #[doc = " Begin exclusive write access to entity.\n This operation provides safe exclusive access to the components of an entity\n without the overhead of deferring operations.\n\n When this operation is called simultaneously for the same entity more than\n once it will throw an assert. Note that for this to happen, asserts must be\n enabled. It is up to the application to ensure that access is exclusive, for\n example by using a read-write mutex.\n\n Exclusive access is enforced at the table level, so only one entity can be\n exclusively accessed per table. The exclusive access check is thread safe.\n\n This operation must be followed up with ecs_write_end().\n\n @param world The world.\n @param entity The entity.\n @return A record to the entity."]
    pub fn ecs_write_begin(world: *mut ecs_world_t, entity: ecs_entity_t) -> *mut ecs_record_t;
    #[doc = " End exclusive write access to entity.\n This operation ends exclusive access, and must be called after\n ecs_write_begin().\n\n @param record Record to the entity."]
    pub fn ecs_write_end(record: *mut ecs_record_t);
    #[doc = " Begin read access to entity.\n This operation provides safe read access to the components of an entity.\n Multiple simultaneous reads are allowed per entity.\n\n This operation ensures that code attempting to mutate the entity's table will\n throw an assert. Note that for this to happen, asserts must be enabled. It is\n up to the application to ensure that this does not happen, for example by\n using a read-write mutex.\n\n This operation does *not* provide the same guarantees as a read-write mutex,\n as it is possible to call ecs_read_begin() after calling ecs_write_begin(). It is\n up to application has to ensure that this does not happen.\n\n This operation must be followed up with ecs_read_end().\n\n @param world The world.\n @param entity The entity.\n @return A record to the entity."]
    pub fn ecs_read_begin(world: *mut ecs_world_t, entity: ecs_entity_t) -> *const ecs_record_t;
    #[doc = " End read access to entity.\n This operation ends read access, and must be called after ecs_read_begin().\n\n @param record Record to the entity."]
    pub fn ecs_read_end(record: *const ecs_record_t);
    #[doc = " Get entity corresponding with record.\n This operation only works for entities that are not empty.\n\n @param record The record for which to obtain the entity id.\n @return The entity id for the record."]
    pub fn ecs_record_get_entity(record: *const ecs_record_t) -> ecs_entity_t;
    #[doc = " Get component from entity record.\n This operation returns a pointer to a component for the entity\n associated with the provided record. For safe access to the component, obtain\n the record with ecs_read_begin() or ecs_write_begin().\n\n Obtaining a component from a record is faster than obtaining it from the\n entity handle, as it reduces the number of lookups required.\n\n @param world The world.\n @param record Record to the entity.\n @param id The (component) id.\n @return Pointer to component, or NULL if entity does not have the component."]
    pub fn ecs_record_get_id(
        world: *const ecs_world_t,
        record: *const ecs_record_t,
        id: ecs_id_t,
    ) -> *const ::std::os::raw::c_void;
    #[doc = " Same as ecs_record_get_id(), but returns a mutable pointer.\n For safe access to the component, obtain the record with ecs_write_begin().\n\n @param world The world.\n @param record Record to the entity.\n @param id The (component) id.\n @return Pointer to component, or NULL if entity does not have the component."]
    pub fn ecs_record_ensure_id(
        world: *mut ecs_world_t,
        record: *mut ecs_record_t,
        id: ecs_id_t,
    ) -> *mut ::std::os::raw::c_void;
    #[doc = " Test if entity for record has a (component) id.\n\n @param world The world.\n @param record Record to the entity.\n @param id The (component) id.\n @return Whether the entity has the component."]
    pub fn ecs_record_has_id(
        world: *mut ecs_world_t,
        record: *const ecs_record_t,
        id: ecs_id_t,
    ) -> bool;
    #[doc = " Get component pointer from column/record.\n This returns a pointer to the component using a table column index. The\n table's column index can be found with ecs_table_get_column_index().\n\n Usage:\n @code\n ecs_record_t *r = ecs_record_find(world, entity);\n int32_t column = ecs_table_get_column_index(world, table, ecs_id(Position));\n Position *ptr = ecs_record_get_by_column(r, column, sizeof(Position));\n @endcode\n\n @param record The record.\n @param column The column index in the entity's table.\n @param size The component size.\n @return The component pointer."]
    pub fn ecs_record_get_by_column(
        record: *const ecs_record_t,
        column: i32,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;
    #[doc = " Emplace a component.\n Emplace is similar to ecs_ensure_id() except that the component constructor\n is not invoked for the returned pointer, allowing the component to be\n constructed directly in the storage.\n\n When the is_new parameter is not provided, the operation will assert when the\n component already exists. When the is_new parameter is provided, it will\n indicate whether the returned storage has been constructed.\n\n When is_new indicates that the storage has not yet been constructed, it must\n be constructed by the code invoking this operation. Not constructing the\n component will result in undefined behavior.\n\n @param world The world.\n @param entity The entity.\n @param id The component to obtain.\n @param is_new Whether this is an existing or new component.\n @return The (uninitialized) component pointer."]
    pub fn ecs_emplace_id(
        world: *mut ecs_world_t,
        entity: ecs_entity_t,
        id: ecs_id_t,
        is_new: *mut bool,
    ) -> *mut ::std::os::raw::c_void;
    #[doc = " Signal that a component has been modified.\n This operation is usually used after modifying a component value obtained by\n ecs_ensure_id(). The operation will mark the component as dirty, and invoke\n OnSet observers and hooks.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component that was modified."]
    pub fn ecs_modified_id(world: *mut ecs_world_t, entity: ecs_entity_t, id: ecs_id_t);
    #[doc = " Set the value of a component.\n This operation allows an application to set the value of a component. The\n operation is equivalent to calling ecs_ensure_id() followed by\n ecs_modified_id(). The operation will not modify the value of the passed in\n component. If the component has a copy hook registered, it will be used to\n copy in the component.\n\n If the provided entity is 0, a new entity will be created.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component to set.\n @param size The size of the pointed-to value.\n @param ptr The pointer to the value."]
    pub fn ecs_set_id(
        world: *mut ecs_world_t,
        entity: ecs_entity_t,
        id: ecs_id_t,
        size: usize,
        ptr: *const ::std::os::raw::c_void,
    );
    */

    /// Test whether an entity is valid.
    /// Entities that are valid can be used with API functions. Using invalid
    /// entities with API operations will cause the function to panic.
    ///
    /// An entity is valid if it is not 0 and if it is alive.
    ///
    /// `ecs_is_valid()` will return true for ids that don't exist (alive or not alive). This
    /// allows for using ids that have never been created by ecs_new_w() or similar. In
    /// this the function differs from `ecs_is_alive()`, which will return false for
    /// entities that do not yet exist.
    ///
    /// The operation will return false for an id that exists and is not alive, as
    /// using this id with an API operation would cause it to assert.
    ///
    /// # Parameters
    /// - `world`: The world.
    /// - `e`: The entity.
    ///
    /// # Returns
    /// `true` if the entity is valid, `false` if the entity is not valid.
    pub fn is_valid(&self, e: Entity) -> bool {
        unsafe { ecs_is_valid(self.as_ptr(), e.as_u64()) }
    }

    /// Test whether an entity is alive.
    /// Entities are alive after they are created, and become not alive when they are
    /// deleted. Operations that return alive ids are (amongst others) ecs_new(),
    /// ecs_new_low_id() and ecs_entity_init(). Ids can be made alive with the ecs_make_alive()
    /// function.
    ///
    /// For example, when obtaining the parent id from a ChildOf relationship, the parent
    /// (second element of the pair) will have been stored in a 32 bit value, which
    /// cannot store the entity generation. This function can retrieve the identifier
    /// with the current generation for that id.
    ///
    /// If the provided identifier is not alive, the function will return 0.
    ///
    /// # Parameters
    /// - `world`: The world.
    /// - `e`: The for which to obtain the current alive entity id.
    ///
    /// # Returns
    /// The alive entity id if there is one, or 0 if the id is not alive.
    pub fn is_alive(&self, e: Entity) -> bool {
        unsafe { ecs_is_alive(self.as_ptr(), e.as_u64()) }
    }

    /// Ensure id is alive.
    /// This operation ensures that the provided id is alive. This is useful in
    /// scenarios where an application has an existing id that has not been created
    /// with ecs_new_w() (such as a global constant or an id from a remote application).
    ///
    /// When this operation is successful it guarantees that the provided id exists,
    /// is valid and is alive.
    ///
    /// Before this operation the id must either not be alive or have a generation
    /// that is equal to the passed in entity.
    ///
    /// If the provided id has a non-zero generation count and the id does not exist
    /// in the world, the id will be created with the specified generation.
    ///
    /// If the provided id is alive and has a generation count that does not match
    /// the provided id, the operation will fail.
    ///
    /// # Parameters
    /// - `world`: The world.
    /// - `entity`: The entity id to make alive.
    pub fn make_alive(&mut self, entity: Entity) {
        unsafe { ecs_make_alive(self.as_mut_ptr(), entity.as_u64()) }
    }

    /// Same as ecs_make_alive(), but for (component) ids.
    /// An id can be an entity or pair, and can contain id flags. This operation
    /// ensures that the entity (or entities, for a pair) are alive.
    ///
    /// When this operation is successful it guarantees that the provided id can be
    /// used in operations that accept an id.
    ///
    /// Since entities in a pair do not encode their generation ids, this operation
    /// will not fail when an entity with non-zero generation count already exists in
    /// the world.
    ///
    /// This is different from ecs_make_alive(), which will fail if attempted with an id
    /// that has generation 0 and an entity with a non-zero generation is currently
    /// alive.
    ///
    /// # Parameters
    /// - `world`: The world.
    /// - `id`: The id to make alive.
    pub fn make_alive_id(&mut self, id: Id) {
        unsafe { ecs_make_alive_id(self.as_mut_ptr(), id.as_u64()) }
    }

    /// Test whether an entity exists.
    /// Similar as ecs_is_alive(), but ignores entity generation count.
    ///
    /// # Parameters
    /// - `world`: The world.
    /// - `entity`: The entity.
    ///
    /// # Returns
    /// `true` if the entity exists, `false` if the entity does not exist.
    pub fn exists(&self, entity: Entity) -> bool {
        unsafe { ecs_exists(self.as_ptr(), entity.as_u64()) }
    }

    /// Override the generation of an entity.
    /// The generation count of an entity is increased each time an entity is deleted
    /// and is used to test whether an entity id is alive.
    ///
    /// This operation overrides the current generation of an entity with the
    /// specified generation, which can be useful if an entity is externally managed,
    /// like for external pools, savefiles or netcode.
    ///
    /// This operation is similar to ecs_make_alive, except that it will also
    /// override the generation of an alive entity.
    ///
    /// # Parameters
    /// - `world`: The world.
    /// - `entity`: Entity for which to set the generation with the new generation.
    pub fn set_generation(&mut self, entity: Entity) {
        unsafe { ecs_set_generation(self.as_mut_ptr(), entity.as_u64()) }
    }
    #[doc = " Get the type of an entity.\n\n @param world The world.\n @param entity The entity.\n @return The type of the entity, NULL if the entity has no components."]
    pub fn get_type(&self, entity: Entity) -> Option<&[Id]> {
        unsafe {
            let etype = ecs_get_type(self.as_ptr(), entity.as_u64());
            if !etype.is_null() {
                let result = std::slice::from_raw_parts((*etype).array, (*etype).count as usize);
                assert!(result
                    .into_iter()
                    .copied()
                    .all(|id| Id::try_from(id).is_ok()));
                Some(std::mem::transmute(result))
            } else {
                None
            }
        }
    }
    #[doc = " Get the table of an entity.\n\n @param world The world.\n @param entity The entity.\n @return The table of the entity, NULL if the entity has no components/tags."]
    pub fn get_table(&self, entity: Entity) -> Option<Table> {
        unsafe {
            let ptr = ecs_get_table(self.as_ptr(), entity.as_u64());
            if ptr.is_null() {
                None
            } else {
                Some(Table(ptr))
            }
        }
    }
    #[doc = " Convert type to string.\n The result of this operation must be freed with ecs_os_free().\n\n @param world The world.\n @param type The type.\n @return The stringified type."]
    pub fn type_str(&self, etype: &[Id]) -> Option<String> {
        unsafe {
            let type_: ecs_type_t = std::mem::transmute(etype);
            let ptr = ecs_type_str(self.as_ptr(), &type_);
            if ptr.is_null() {
                None
            } else {
                let result = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
                (ecs_os_get_api().free_.unwrap())(ptr as *mut _);
                Some(result)
            }
        }
    }
    #[doc = " Convert table to string.\n Same as ecs_type_str(world, ecs_table_get_type(table)). The result of this\n operation must be freed with ecs_os_free().\n\n @param world The world.\n @param table The table.\n @return The stringified table type."]
    pub fn table_str(&self, table: &Table) -> Option<String> {
        unsafe {
            let ptr = ecs_table_str(self.as_ptr(), table.as_ptr());
            if ptr.is_null() {
                None
            } else {
                let result = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
                (ecs_os_get_api().free_.unwrap())(ptr as *mut _);
                Some(result)
            }
        }
    }
    #[doc = " Convert entity to string.\n Same as combining:\n - ecs_get_fullpath(world, entity)\n - ecs_type_str(world, ecs_get_type(world, entity))\n\n The result of this operation must be freed with ecs_os_free().\n\n @param world The world.\n @param entity The entity.\n @return The entity path with stringified type."]
    /// Convert entity to string.
    /// Same as combining:
    /// - `ecs_get_fullpath(world, entity)`
    /// - `ecs_type_str(world, ecs_get_type(world, entity))`
    ///
    /// The result of this operation must be freed with `ecs_os_free()`.
    ///
    /// # Arguments
    ///
    /// * `world` - The world.
    /// * `entity` - The entity.
    ///
    /// # Returns
    ///
    /// The entity path with stringified type.
    pub fn entity_str(&self, entity: Entity) -> Option<String> {
        unsafe {
            let ptr = ecs_entity_str(self.as_ptr(), entity.as_u64());
            if ptr.is_null() {
                None
            } else {
                let result = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
                (ecs_os_get_api().free_.unwrap())(ptr as *mut _);
                Some(result)
            }
        }
    }

    /// Test if an entity has an id.
    ///
    /// This operation returns true if the entity has or inherits the specified id.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity.
    /// * `id` - The id to test for.
    ///
    /// # Returns
    ///
    /// True if the entity has the id, false if not.
    pub fn has_id(&self, entity: Entity, id: Id) -> bool {
        unsafe { ecs_has_id(self.as_ptr(), entity.into(), id.into()) }
    }

    /// Test if an entity owns an id.
    ///
    /// This operation returns true if the entity has the specified id. The operation
    /// behaves the same as `has_id()`, except that it will return false for
    /// components that are inherited through an IsA relationship.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity.
    /// * `id` - The id to test for.
    ///
    /// # Returns
    ///
    /// True if the entity has the id, false if not.
    pub fn owns_id(&self, entity: Entity, id: Id) -> bool {
        unsafe { ecs_owns_id(self.as_ptr(), entity.into(), id.into()) }
    }
    /// Get the target of a relationship.
    ///
    /// This will return a target (second element of a pair) of the entity for the
    /// specified relationship. The index allows for iterating through the targets,
    /// if a single entity has multiple targets for the same relationship.
    ///
    /// If the index is larger than the total number of instances the entity has for
    /// the relationship, the operation will return 0.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity.
    /// * `rel` - The relationship between the entity and the target.
    /// * `index` - The index of the relationship instance.
    ///
    /// # Returns
    ///
    /// The target for the relationship at the specified index.
    pub fn get_target(&self, entity: Entity, rel: Entity, index: i32) -> Entity {
        unsafe { ecs_get_target(self.as_ptr(), entity.into(), rel.into(), index) }
            .try_into()
            .unwrap()
    }
    #[doc = " Get parent (target of ChildOf relationship) for entity.\n This operation is the same as calling:\n\n @code\n ecs_get_target(world, entity, EcsChildOf, 0);\n @endcode\n\n @param world The world.\n @param entity The entity.\n @return The parent of the entity, 0 if the entity has no parent."]
    pub fn get_parent(&self, entity: Entity) -> Entity {
        unsafe {
            ecs_get_parent(self.as_ptr(), entity.as_u64())
                .try_into()
                .unwrap()
        }
    }

    /// Get the target of a relationship for a given id.
    /// This operation returns the first entity that has the provided id by following
    /// the specified relationship. If the entity itself has the id then entity will
    /// be returned. If the id cannot be found on the entity or by following the
    /// relationship, the operation will return 0.
    ///
    /// This operation can be used to lookup, for example, which prefab is providing
    /// a component by specifying the IsA relationship:
    ///
    /// ```rust
    /// // Is Position provided by the entity or one of its base entities?
    /// ecs.get_target_for_id(entity, EcsIsA, ecs.id::<Position>())
    /// ```
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity.
    /// * `rel` - The relationship to follow.
    /// * `id` - The id to lookup.
    /// # Returns
    ///
    /// The entity for which the target has been found.
    pub fn get_target_for_id(&self, entity: Entity, rel: Entity, id: Id) -> Entity {
        unsafe { ecs_get_target_for_id(self.as_ptr(), entity.as_u64(), rel.as_u64(), id.as_u64()) }
            .try_into()
            .unwrap()
    }
    #[doc = " Return depth for entity in tree for the specified relationship.\n Depth is determined by counting the number of targets encountered while\n traversing up the relationship tree for rel. Only acyclic relationships are\n supported.\n\n @param world The world.\n @param entity The entity.\n @param rel The relationship.\n @return The depth of the entity in the tree."]
    pub fn get_depth(world: &World, entity: Entity, rel: Entity) -> i32 {
        unsafe { ecs_get_depth(world.as_ptr(), entity.into(), rel.into()) }
    }
    #[doc = " Count entities that have the specified id.\n Returns the number of entities that have the specified id.\n\n @param world The world.\n @param entity The id to search for.\n @return The number of entities that have the id."]
    pub fn count_id(world: &World, entity: Id) -> i32 {
        unsafe { ecs_count_id(world.as_ptr(), entity.into()) }
    }
    #[doc = " Get the name of an entity.\n This will return the name stored in (EcsIdentifier, EcsName).\n\n @param world The world.\n @param entity The entity.\n @return The type of the entity, NULL if the entity has no name."]

    pub fn get_name(&self, entity: Entity) -> Option<String> {
        let name = unsafe { ecs_get_name(self.as_ptr(), entity.as_u64()) };
        if name.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() })
        }
    }
    #[doc = " Get the symbol of an entity.\n This will return the symbol stored in (EcsIdentifier, EcsSymbol).\n\n @param world The world.\n @param entity The entity.\n @return The type of the entity, NULL if the entity has no name."]
    pub fn get_symbol(&self, entity: Entity) -> Option<String> {
        let symbol = unsafe { ecs_get_symbol(self.as_ptr(), entity.as_u64()) };
        if symbol.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(symbol).to_string_lossy().into_owned() })
        }
    }
    #[doc = " Set the name of an entity.\n This will set or overwrite the name of an entity. If no entity is provided,\n a new entity will be created.\n\n The name is stored in (EcsIdentifier, EcsName).\n\n @param world The world.\n @param entity The entity.\n @param name The name.\n @return The provided entity, or a new entity if 0 was provided."]

    pub fn set_name(&mut self, entity: Entity, name: &str) -> Entity {
        let name_cstr = CString::new(name).expect("Failed to create CString");
        unsafe { ecs_set_name(self.as_mut_ptr(), entity.as_u64(), name_cstr.as_ptr()) }
            .try_into()
            .unwrap()
    }
    #[doc = " Set the symbol of an entity.\n This will set or overwrite the symbol of an entity. If no entity is provided,\n a new entity will be created.\n\n The symbol is stored in (EcsIdentifier, EcsSymbol).\n\n @param world The world.\n @param entity The entity.\n @param symbol The symbol.\n @return The provided entity, or a new entity if 0 was provided."]

    pub fn set_symbol(&mut self, entity: Entity, symbol: &str) -> Entity {
        let symbol_cstr = CString::new(symbol).expect("Failed to create CString");
        unsafe { ecs_set_symbol(self.as_mut_ptr(), entity.as_u64(), symbol_cstr.as_ptr()) }
            .try_into()
            .unwrap()
    }
    #[doc = " Set alias for entity.\n An entity can be looked up using its alias from the root scope without\n providing the fully qualified name if its parent. An entity can only have\n a single alias.\n\n The symbol is stored in (EcsIdentifier, EcsAlias).\n\n @param world The world.\n @param entity The entity.\n @param alias The alias."]

    pub fn set_alias(&mut self, entity: Entity, alias: &str) {
        let alias_cstr = CString::new(alias).expect("Failed to create CString");
        unsafe { ecs_set_alias(self.as_mut_ptr(), entity.as_u64(), alias_cstr.as_ptr()) }
    }
    #[doc = " Lookup an entity by it's path.\n This operation is equivalent to calling:\n\n @code\n ecs_lookup_path_w_sep(world, 0, path, \".\", NULL, true);\n @endcode\n\n @param world The world.\n @param path The entity path.\n @return The entity with the specified path, or 0 if no entity was found."]

    pub fn lookup(&self, path: &str) -> Entity {
        let path_cstr = CString::new(path).expect("Failed to create CString");
        unsafe { ecs_lookup(self.as_ptr(), path_cstr.as_ptr()) }
            .try_into()
            .unwrap()
    }
    #[doc = " Lookup a child entity by name.\n Returns an entity that matches the specified name. Only looks for entities in\n the provided parent. If no parent is provided, look in the current scope (\n root if no scope is provided).\n\n @param world The world.\n @param parent The parent for which to lookup the child.\n @param name The entity name.\n @return The entity with the specified name, or 0 if no entity was found."]

    pub fn lookup_child(&self, parent: Entity, name: &str) -> Entity {
        let name_cstr = CString::new(name).expect("Failed to create CString");
        unsafe { ecs_lookup_child(self.as_ptr(), parent.as_u64(), name_cstr.as_ptr()) }
            .try_into()
            .unwrap()
    }

    #[doc = " Lookup an entity from a path.\n Lookup an entity from a provided path, relative to the provided parent. The\n operation will use the provided separator to tokenize the path expression. If\n the provided path contains the prefix, the search will start from the root.\n\n If the entity is not found in the provided parent, the operation will\n continue to search in the parent of the parent, until the root is reached. If\n the entity is still not found, the lookup will search in the flecs.core\n scope. If the entity is not found there either, the function returns 0.\n\n @param world The world.\n @param parent The entity from which to resolve the path.\n @param path The path to resolve.\n @param sep The path separator.\n @param prefix The path prefix.\n @param recursive Recursively traverse up the tree until entity is found.\n @return The entity if found, else 0."]
    pub fn lookup_entity_from_path(
        &self,
        parent: Entity,
        path: &str,
        sep: &str,
        prefix: &str,
        recursive: bool,
    ) -> Entity {
        let path_cstr = CString::new(path).expect("Failed to convert path to CString");
        let sep_cstr = CString::new(sep).expect("Failed to convert sep to CString");
        let prefix_cstr = CString::new(prefix).expect("Failed to convert prefix to CString");
        unsafe {
            ecs_lookup_path_w_sep(
                self.as_ptr(),
                parent.as_u64(),
                path_cstr.as_ptr(),
                sep_cstr.as_ptr(),
                prefix_cstr.as_ptr(),
                recursive,
            )
        }
        .try_into()
        .unwrap()
    }

    #[doc = " Lookup an entity by its symbol name.\n This looks up an entity by symbol stored in (EcsIdentifier, EcsSymbol). The\n operation does not take into account hierarchies.\n\n This operation can be useful to resolve, for example, a type by its C\n identifier, which does not include the Flecs namespacing.\n\n @param world The world.\n @param symbol The symbol.\n @param lookup_as_path If not found as a symbol, lookup as path.\n @param recursive If looking up as path, recursively traverse up the tree.\n @return The entity if found, else 0."]
    pub fn lookup_entity_by_symbol(
        &self,
        symbol: &str,
        lookup_as_path: bool,
        recursive: bool,
    ) -> Entity {
        let symbol_cstr = CString::new(symbol).expect("Failed to convert symbol to CString");
        unsafe {
            ecs_lookup_symbol(
                self.as_ptr(),
                symbol_cstr.as_ptr(),
                lookup_as_path,
                recursive,
            )
        }
        .try_into()
        .unwrap()
    }

    #[doc = " Get a path identifier for an entity.\n This operation creates a path that contains the names of the entities from\n the specified parent to the provided entity, separated by the provided\n separator. If no parent is provided the path will be relative to the root. If\n a prefix is provided, the path will be prefixed by the prefix.\n\n If the parent is equal to the provided child, the operation will return an\n empty string. If a nonzero component is provided, the path will be created by\n looking for parents with that component.\n\n The returned path should be freed by the application.\n\n @param world The world.\n @param parent The entity from which to create the path.\n @param child The entity to which to create the path.\n @param sep The separator to use between path elements.\n @param prefix The initial character to use for root elements.\n @return The relative entity path."]
    pub fn get_entity_path(
        &self,
        parent: Entity,
        child: Entity,
        sep: &str,
        prefix: &str,
    ) -> Option<String> {
        let sep_cstr = CString::new(sep).expect("Failed to convert sep to CString");
        let prefix_cstr = CString::new(prefix).expect("Failed to convert prefix to CString");
        let path_ptr = unsafe {
            ecs_get_path_w_sep(
                self.as_ptr(),
                parent.as_u64(),
                child.as_u64(),
                sep_cstr.as_ptr(),
                prefix_cstr.as_ptr(),
            )
        };
        if !path_ptr.is_null() {
            let path = unsafe { CStr::from_ptr(path_ptr) };
            let path_str = path.to_string_lossy().into_owned();
            unsafe { (ecs_os_get_api().free_.unwrap())(path_ptr as _) };
            Some(path_str)
        } else {
            None
        }
    }

    // #[doc = " Write path identifier to buffer.\n Same as ecs_get_path_w_sep(), but writes result to an ecs_strbuf_t.\n\n @param world The world.\n @param parent The entity from which to create the path.\n @param child The entity to which to create the path.\n @param sep The separator to use between path elements.\n @param prefix The initial character to use for root elements.\n @param buf The buffer to write to."]
    // pub fn get_entity_path_to_buffer(
    //     &self,
    //     parent: Entity,
    //     child: Entity,
    //     sep: &str,
    //     prefix: &str,
    //     buf: &mut ecs_strbuf_t,
    // ) {
    //     let sep_cstr = CString::new(sep).expect("Failed to convert sep to CString");
    //     let prefix_cstr = CString::new(prefix).expect("Failed to convert prefix to CString");
    //     unsafe {
    //         ecs_get_path_w_sep_buf(
    //             world,
    //             parent,
    //             child,
    //             sep_cstr.as_ptr(),
    //             prefix_cstr.as_ptr(),
    //             buf,
    //         )
    //     }
    // }

    #[doc = " Find or create entity from path.\n This operation will find or create an entity from a path, and will create any\n intermediate entities if required. If the entity already exists, no entities\n will be created.\n\n If the path starts with the prefix, then the entity will be created from the\n root scope.\n\n @param world The world.\n @param parent The entity relative to which the entity should be created.\n @param path The path to create the entity for.\n @param sep The separator used in the path.\n @param prefix The prefix used in the path.\n @return The entity."]
    pub fn find_or_create_entity_from_path(
        &mut self,
        parent: Entity,
        path: &str,
        sep: &str,
        prefix: &str,
    ) -> Entity {
        let path_cstr = CString::new(path).expect("Failed to convert path to CString");
        let sep_cstr = CString::new(sep).expect("Failed to convert sep to CString");
        let prefix_cstr = CString::new(prefix).expect("Failed to convert prefix to CString");
        unsafe {
            ecs_new_from_path_w_sep(
                self.as_mut_ptr(),
                parent.as_u64(),
                path_cstr.as_ptr(),
                sep_cstr.as_ptr(),
                prefix_cstr.as_ptr(),
            )
        }
        .try_into()
        .unwrap()
    }

    #[doc = " Add specified path to entity.\n This operation is similar to ecs_new_from_path(), but will instead add the path\n to an existing entity.\n\n If an entity already exists for the path, it will be returned instead.\n\n @param world The world.\n @param entity The entity to which to add the path.\n @param parent The entity relative to which the entity should be created.\n @param path The path to create the entity for.\n @param sep The separator used in the path.\n @param prefix The prefix used in the path.\n @return The entity."]
    pub fn add_path_to_entity(
        &mut self,
        entity: Entity,
        parent: Entity,
        path: &str,
        sep: &str,
        prefix: &str,
    ) -> Entity {
        let path_cstr = CString::new(path).expect("Failed to convert path to CString");
        let sep_cstr = CString::new(sep).expect("Failed to convert sep to CString");
        let prefix_cstr = CString::new(prefix).expect("Failed to convert prefix to CString");
        unsafe {
            ecs_add_path_w_sep(
                self.as_mut_ptr(),
                entity.as_u64(),
                parent.as_u64(),
                path_cstr.as_ptr(),
                sep_cstr.as_ptr(),
                prefix_cstr.as_ptr(),
            )
        }
        .try_into()
        .unwrap()
    }

    #[doc = " Set the current scope.\n This operation sets the scope of the current stage to the provided entity.\n As a result new entities will be created in this scope, and lookups will be\n relative to the provided scope.\n\n It is considered good practice to restore the scope to the old value.\n\n @param world The world.\n @param scope The entity to use as scope.\n @return The previous scope."]
    pub fn set_scope(&mut self, scope: Entity) -> Entity {
        unsafe { ecs_set_scope(self.as_mut_ptr(), scope.as_u64()) }
            .try_into()
            .unwrap()
    }

    #[doc = " Get the current scope.\n Get the scope set by ecs_set_scope(). If no scope is set, this operation will\n return 0.\n\n @param world The world.\n @return The current scope."]
    pub fn get_scope(&self) -> Entity {
        unsafe { ecs_get_scope(self.as_ptr()) }.try_into().unwrap()
    }

    #[doc = " Set a name prefix for newly created entities.\n This is a utility that lets C modules use prefixed names for C types and\n C functions, while using names for the entity names that do not have the\n prefix. The name prefix is currently only used by ECS_COMPONENT.\n\n @param world The world.\n @param prefix The name prefix to use.\n @return The previous prefix."]
    pub fn set_name_prefix(&mut self, prefix: &str) -> Option<String> {
        let prefix_cstr = CString::new(prefix).expect("Failed to convert prefix to CString");
        let prev_prefix_ptr =
            unsafe { ecs_set_name_prefix(self.as_mut_ptr(), prefix_cstr.as_ptr()) };
        if !prev_prefix_ptr.is_null() {
            let prev_prefix = unsafe { CStr::from_ptr(prev_prefix_ptr) };
            let prev_prefix_str = prev_prefix.to_string_lossy().into_owned();
            Some(prev_prefix_str)
        } else {
            None
        }
    }

    // #[doc = " Set search path for lookup operations.\n This operation accepts an array of entity ids that will be used as search\n scopes by lookup operations. The operation returns the current search path.\n It is good practice to restore the old search path.\n\n The search path will be evaluated starting from the last element.\n\n The default search path includes flecs.core. When a custom search path is\n provided it overwrites the existing search path. Operations that rely on\n looking up names from flecs.core without providing the namespace may fail if\n the custom search path does not include flecs.core (EcsFlecsCore).\n\n The search path array is not copied into managed memory. The application must\n ensure that the provided array is valid for as long as it is used as the\n search path.\n\n The provided array must be terminated with a 0 element. This enables an\n application to push/pop elements to an existing array without invoking the\n ecs_set_lookup_path() operation again.\n\n @param world The world.\n @param lookup_path 0-terminated array with entity ids for the lookup path.\n @return Current lookup path array."]
    // pub fn set_lookup_path(
    //     &mut self,
    //     lookup_path: &[Entity],
    // ) -> Vec<Entity> {
    //     let lookup_path_ptr = lookup_path.as_ptr();
    //     unsafe { ecs_set_lookup_path(world, lookup_path_ptr) }
    //         .iter()
    //         .copied()
    //         .collect()
    // }

    // #[doc = " Get current lookup path.\n Returns value set by ecs_set_lookup_path().\n\n @param world The world.\n @return The current lookup path."]
    // pub fn get_lookup_path(world: *const ecs_world_t) -> Vec<ecs_entity_t> {
    //     unsafe { ecs_get_lookup_path(world) }
    //         .iter()
    //         .copied()
    //         .collect()
    // }

    // #[doc = " Find or create a component.\n This operation creates a new component, or finds an existing one. The find or\n create behavior is the same as ecs_entity_init().\n\n When an existing component is found, the size and alignment are verified with\n the provided values. If the values do not match, the operation will fail.\n\n See the documentation of ecs_component_desc_t for more details.\n\n @param world The world.\n @param desc Component init parameters.\n @return A handle to the new or existing component, or 0 if failed."]
    // pub fn find_or_create_component(
    //     world: *mut ecs_world_t,
    //     desc: &ecs_component_desc_t,
    // ) -> ecs_entity_t {
    //     unsafe { ecs_component_init(world, desc) }
    // }

    // #[doc = " Get the type for an id.\n This function returns the type information for an id. The specified id can be\n any valid id. For the rules on how type information is determined based on\n id, see ecs_get_typeid().\n\n @param world The world.\n @param id The id.\n @return The type information of the id."]
    // pub fn get_type_info(
    //     world: *const ecs_world_t,
    //     id: ecs_id_t,
    // ) -> Option<&'static ecs_type_info_t> {
    //     let type_info = unsafe { ecs_get_type_info(world, id) };
    //     if !type_info.is_null() {
    //         Some(unsafe { &*type_info })
    //     } else {
    //         None
    //     }
    // }
}
/// Takes a **reference** to a variable of type `*const ecs_world_t` and then transmutes it to `&World`
/// This allows us to use a variable of `*const ecs_world_t` pointer as a `&World` without actually creating a `World`
/// The main reason is to avoid taking ownership of the world in certain situations like callbacks
///
/// # Safety
/// This function is unsafe because it transmutes references
unsafe fn get_world(ref_to_world_ptr: &*const ecs_world_t) -> &World {
    std::mem::transmute(ref_to_world_ptr)
}
/// Takes a **mutable reference** to a variable of type `*mut ecs_world_t` and then transmutes it to `&mut World`
/// This allows us to use a variable of `*mut ecs_world_t` pointer as a `&mut World` without actually creating a `World`
/// The main reason is to avoid taking ownership of the world in certain situations like callbacks
///
/// # Safety
/// This function is unsafe because it transmutes references
unsafe fn get_world_mut(ref_to_world_ptr: &mut *mut ecs_world_t) -> &mut World {
    std::mem::transmute(ref_to_world_ptr)
}
impl Drop for World {
    #[doc = " Delete a world.\n This operation deletes the world, and everything it contains.\n\n @param world The world to delete.\n @return Zero if successful, non-zero if failed."]
    fn drop(&mut self) {
        unsafe {
            assert!(ecs_fini(self.0) == 0);
        }
    }
}
#[doc = " Type with information about the current Flecs build"]
#[derive(Debug, Copy, Clone)]
pub struct BuildInfo {
    #[doc = "< Compiler used to compile flecs"]
    pub compiler: &'static str,
    #[doc = "< Addons included in build"]
    pub addons: &'static [&'static str],
    #[doc = "< Stringified version"]
    pub version: &'static str,
    #[doc = "< Major flecs version"]
    pub version_major: i16,
    #[doc = "< Minor flecs version"]
    pub version_minor: i16,
    #[doc = "< Patch flecs version"]
    pub version_patch: i16,
    #[doc = "< Is this a debug build"]
    pub debug: bool,
    #[doc = "< Is this a sanitize build"]
    pub sanitize: bool,
    #[doc = "< Is this a perf tracing build"]
    pub perf_trace: bool,
}
impl BuildInfo {
    /// The build info provided by flecs is static.
    /// But, we do need to convert it all into safe rust types, so we use OnceLock to store it
    /// The first time this function gets called, we will read the build info from ecs_get_build_info() and store a [BuildInfo] in oncelock.
    /// After that, we will just keep returning the same static reference for all subsequent calls.
    pub fn get_static() -> &'static Self {
        static BUILD_INFO: OnceLock<BuildInfo> = OnceLock::new();
        static ADDONS_LIST: OnceLock<Box<[&'static str]>> = OnceLock::new();

        BUILD_INFO.get_or_init(|| unsafe {
            let info = *ecs_get_build_info();
            let addons_list = ADDONS_LIST.get_or_init(|| {
                // list is null terminated.
                // The pointer is an array of char*
                let list_ptr = info.addons;
                // check null *just in case*
                assert!(!list_ptr.is_null());
                // as all strings are static, lets gather them into this vec
                let mut addons = vec![];
                // we don't know how many addons might have been loaded
                // So, we use 20 as an upper bound. It is still UB if we go beyond array, but doesn't hurt to limit the damage
                for n in 0..20 {
                    // lets get the nth char* in the array
                    let nth_char_ptr = *list_ptr.add(n);
                    // if null, then we are at the last element, so we break
                    if nth_char_ptr == std::ptr::null_mut::<i8>() {
                        break;
                    }
                    // if not null, then this is a valid char*
                    // lets get a cstr using the pointer. It is null terminated (probably), so we are good.
                    let addon = CStr::from_ptr(nth_char_ptr);
                    addons.push(addon.to_str().unwrap());
                }
                addons.into_boxed_slice()
            });

            BuildInfo {
                compiler: CStr::from_ptr(info.compiler).to_str().unwrap(),
                addons: addons_list.as_ref(),
                version: CStr::from_ptr(info.version).to_str().unwrap(),
                version_major: info.version_major,
                version_minor: info.version_minor,
                version_patch: info.version_patch,
                debug: info.debug,
                sanitize: info.sanitize,
                perf_trace: info.perf_trace,
            }
        })
    }
}
#[doc = " Type that contains information about the world."]
#[derive(Debug, Copy, Clone)]
pub struct WorldInfo<'a> {
    #[doc = "< Last issued component entity id"]
    pub last_component_id: Entity,
    #[doc = "< First allowed entity id"]
    pub min_id: Entity,
    #[doc = "< Last allowed entity id"]
    pub max_id: Entity,
    #[doc = "< Raw delta time (no time scaling)"]
    pub delta_time_raw: f32,
    #[doc = "< Time passed to or computed by ecs_progress"]
    pub delta_time: f32,
    #[doc = "< Time scale applied to delta_time"]
    pub time_scale: f32,
    #[doc = "< Target fps"]
    pub target_fps: f32,
    #[doc = "< Total time spent processing a frame"]
    pub frame_time_total: f32,
    #[doc = "< Total time spent in systems"]
    pub system_time_total: f32,
    #[doc = "< Total time spent notifying observers"]
    pub emit_time_total: f32,
    #[doc = "< Total time spent in merges"]
    pub merge_time_total: f32,
    #[doc = "< Time elapsed in simulation"]
    pub world_time_total: f32,
    #[doc = "< Time elapsed in simulation (no scaling)"]
    pub world_time_total_raw: f32,
    #[doc = "< Time spent on query rematching"]
    pub rematch_time_total: f32,
    #[doc = "< Total number of frames"]
    pub frame_count_total: i64,
    #[doc = "< Total number of merges"]
    pub merge_count_total: i64,
    #[doc = "< Total number of rematches"]
    pub rematch_count_total: i64,
    #[doc = "< Total number of times a new id was created"]
    pub id_create_total: i64,
    #[doc = "< Total number of times an id was deleted"]
    pub id_delete_total: i64,
    #[doc = "< Total number of times a table was created"]
    pub table_create_total: i64,
    #[doc = "< Total number of times a table was deleted"]
    pub table_delete_total: i64,
    #[doc = "< Total number of pipeline builds"]
    pub pipeline_build_count_total: i64,
    #[doc = "< Total number of systems ran in last frame"]
    pub systems_ran_frame: i64,
    #[doc = "< Total number of times observer was invoked"]
    pub observers_ran_frame: i64,
    #[doc = "< Number of tag (no data) ids in the world"]
    pub tag_id_count: i32,
    #[doc = "< Number of component (data) ids in the world"]
    pub component_id_count: i32,
    #[doc = "< Number of pair ids in the world"]
    pub pair_id_count: i32,
    #[doc = "< Number of tables"]
    pub table_count: i32,
    #[doc = "< Number of tables without entities"]
    pub empty_table_count: i32,
    #[doc = "< Add commands processed"]
    pub add_count: i64,
    #[doc = "< Remove commands processed"]
    pub remove_count: i64,
    #[doc = "< Selete commands processed"]
    pub delete_count: i64,
    #[doc = "< Clear commands processed"]
    pub clear_count: i64,
    #[doc = "< Set commands processed"]
    pub set_count: i64,
    #[doc = "< Ensure/emplace commands processed"]
    pub ensure_count: i64,
    #[doc = "< Modified commands processed"]
    pub modified_count: i64,
    #[doc = "< Commands discarded, happens when entity is no longer alive when running the command"]
    pub discard_count: i64,
    #[doc = "< Enqueued custom events"]
    pub event_count: i64,
    #[doc = "< Other commands processed"]
    pub other_count: i64,
    #[doc = "< Entities for which commands were batched"]
    pub batched_entity_count: i64,
    #[doc = "< Commands batched"]
    pub batched_command_count: i64,
    #[doc = "< Value set by ecs_set_name_prefix(). Used\n to remove library prefixes of symbol\n names (such as `Ecs`, `ecs_`) when\n registering them as names."]
    pub name_prefix: &'a str,
}

pub struct Table(*mut ecs_table_t);
impl Table {
    fn as_ptr(&self) -> *const ecs_table_t {
        self.0.cast_const()
    }
    fn as_mut_ptr(&mut self) -> *mut ecs_table_t {
        self.0
    }
}

#[doc = " Used with ecs_entity_init().\n\n @ingroup entities"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EntityDesc<'a> {
    #[doc = "< Set to modify existing entity (optional)"]
    pub id: Option<Entity>,
    #[doc = "< Parent entity."]
    pub parent: Entity,
    #[doc = "< Name of the entity. If no entity is provided, an\n entity with this name will be looked up first. When\n an entity is provided, the name will be verified\n with the existing entity."]
    pub name: Option<&'a CStr>,
    #[doc = "< Optional custom separator for hierarchical names.\n Leave to NULL for default ('.') separator. Set to\n an empty string to prevent tokenization of name."]
    pub sep: Option<&'a CStr>,
    #[doc = "< Optional, used for identifiers relative to root"]
    pub root_sep: Option<&'a CStr>,
    #[doc = "< Optional entity symbol. A symbol is an unscoped\n identifier that can be used to lookup an entity. The\n primary use case for this is to associate the entity\n with a language identifier, such as a type or\n function name, where these identifiers differ from\n the name they are registered with in flecs. For\n example, C type \"EcsPosition\" might be registered\n as \"flecs.components.transform.Position\", with the\n symbol set to \"EcsPosition\"."]
    pub symbol: Option<&'a CStr>,
    #[doc = "< When set to true, a low id (typically reserved for\n components) will be used to create the entity, if\n no id is specified."]
    pub use_low_id: bool,
    #[doc = "array of ids to add to the entity. last id MUST BE none"]
    pub add: &'a [Option<Id>],
    #[doc = " array of values to set on the entity. last value MUST BE none"]
    pub set: *const ecs_value_t,
    #[doc = " String expression with components to add"]
    pub add_expr: *const ::std::os::raw::c_char,
}
#[doc = " Used with ecs_bulk_init().\n\n @ingroup entities"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ecs_bulk_desc_t {
    #[doc = "< Used for validity testing. Must be 0."]
    pub _canary: i32,
    #[doc = "< Entities to bulk insert. Entity ids provided by\n the application must be empty (cannot\n have components). If no entity ids are provided, the\n operation will create 'count' new entities."]
    pub entities: *mut ecs_entity_t,
    #[doc = "< Number of entities to create/populate"]
    pub count: i32,
    #[doc = "< Ids to create the entities with"]
    pub ids: [ecs_id_t; 32usize],
    #[doc = "< Array with component data to insert. Each element in\n the array must correspond with an element in the ids\n array. If an element in the ids array is a tag, the\n data array must contain a NULL. An element may be\n set to NULL for a component, in which case the\n component will not be set by the operation."]
    pub data: *mut *mut ::std::os::raw::c_void,
    #[doc = "< Table to insert the entities into. Should not be set\n at the same time as ids. When 'table' is set at the\n same time as 'data', the elements in the data array\n must correspond with the ids in the table's type."]
    pub table: *mut ecs_table_t,
}
#[doc = " Used with ecs_component_init().\n\n @ingroup components"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ecs_component_desc_t {
    #[doc = "< Used for validity testing. Must be 0."]
    pub _canary: i32,
    #[doc = " Existing entity to associate with observer (optional)"]
    pub entity: ecs_entity_t,
    #[doc = " Parameters for type (size, hooks, ...)"]
    pub type_: ecs_type_info_t,
}
pub struct Query(*mut ecs_query_t);
pub struct Observer(*mut ecs_observer_t);
pub struct Term(*mut ecs_term_t);
pub struct Iter(*mut ecs_iter_t);
pub struct Observable(*mut ecs_observable_t);
pub struct Ref(*mut ecs_ref_t);
// generate newtype wrappers for c opaque structs from flecsys
pub struct TypeHooks(*mut ecs_type_hooks_t);
pub struct TypeInfo(*mut ecs_type_info_t);
pub struct Record(*mut ecs_record_t);
pub struct IdRecord(*mut ecs_id_record_t);
pub struct TableRecord(*mut ecs_table_record_t);
pub struct Poly(*mut ecs_poly_t);
pub struct Mixins(*mut ecs_mixins_t);
pub struct Header {
    magic: i32,
    ty: i32,
    refcount: i32,
    mixins: *mut ecs_mixins_t,
}
pub type RunAction = extern "C" fn(*mut ecs_iter_t) -> c_void;
pub type IterAction = extern "C" fn(*mut ecs_iter_t) -> c_void;
pub type IterNextAction = extern "C" fn(*mut ecs_iter_t) -> c_int;
pub type IterFiniAction = extern "C" fn(*mut ecs_iter_t) -> c_void;
pub type OrderByAction = extern "C" fn(ecs_entity_t, *const c_void, *const c_void) -> c_int;
pub type SortTableAction = extern "C" fn(*mut ecs_world_t, *mut ecs_table_t) -> c_int;
pub type EcsGroupByAction = extern "C" fn(
    world: *mut ecs_world_t,
    table: *mut ecs_table_t,
    group_id: ecs_id_t,
    ctx: *mut std::ffi::c_void,
) -> u64;
pub type EcsGroupCreateAction = extern "C" fn(
    world: *mut ecs_world_t,
    group_id: u64,
    group_by_ctx: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void;
pub type EcsGroupDeleteAction = extern "C" fn(
    world: *mut ecs_world_t,
    group_id: u64,
    group_ctx: *mut std::ffi::c_void,
    group_by_ctx: *mut std::ffi::c_void,
);
pub type EcsModuleAction = extern "C" fn(world: *mut ecs_world_t);
pub type EcsFiniAction = extern "C" fn(world: *mut ecs_world_t, ctx: *mut std::ffi::c_void);
pub type EcsCtxFree = extern "C" fn(ctx: *mut std::ffi::c_void);
pub type EcsCompareAction =
    extern "C" fn(ptr1: *const std::ffi::c_void, ptr2: *const std::ffi::c_void) -> i32;
pub type EcsHashValueAction = extern "C" fn(ptr: *const std::ffi::c_void) -> u64;
pub type EcsXtor =
    extern "C" fn(ptr: *mut std::ffi::c_void, count: i32, type_info: *const ecs_type_info_t);
pub type EcsCopy = extern "C" fn(
    dst_ptr: *mut std::ffi::c_void,
    src_ptr: *const std::ffi::c_void,
    count: i32,
    type_info: *const ecs_type_info_t,
);
pub type EcsMove = extern "C" fn(
    dst_ptr: *mut std::ffi::c_void,
    src_ptr: *mut std::ffi::c_void,
    count: i32,
    type_info: *const ecs_type_info_t,
);
pub type FlecsPolyDtor = extern "C" fn(poly: *mut ecs_poly_t);
/// Specify read/write access for term
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum InOutKind {
    /// InOut for regular terms, In for shared terms
    Default,
    /// Term is neither read nor written
    None,
    /// Same as `None` + prevents term from triggering observers
    Filter,
    /// Term is both read and written
    InOut,
    /// Term is only read
    In,
    /// Term is only written
    Out,
}

/// Specify operator for term
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum OperKind {
    /// The term must match
    And,
    /// One of the terms in an or chain must match
    Or,
    /// The term must not match
    Not,
    /// The term may match
    Optional,
    /// Term must match all components from term id
    AndFrom,
    /// Term must match at least one component from term id
    OrFrom,
    /// Term must match none of the components from term id
    NotFrom,
}

/// Specify cache policy for query
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum QueryCacheKind {
    /// Behavior determined by query creation context
    Default,
    /// Cache query terms that are cacheable
    Auto,
    /// Require that all query terms can be cached
    All,
    /// No caching
    None,
}

/// Import bitflags macro to easily define bitflags
use bitflags::bitflags;

use crate::AperiodicFlags;
bitflags! {
    /// Flags for term ids
    pub struct TermIdFlags: u64 {
        /// Match on self.
        /// Can be combined with other term flags on the ecs_term_t::flags field.
        /// \ingroup queries
        const SelfFlag = 1u64 << 63;

        /// Match by traversing upwards.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const UpFlag = 1u64 << 62;

        /// Traverse relationship transitively.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const TravFlag = 1u64 << 61;

        /// Sort results breadth first.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const CascadeFlag = 1u64 << 60;

        /// Iterate groups in descending order.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const DescFlag = 1u64 << 59;

        /// Term id is a variable.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const IsVariableFlag = 1u64 << 58;

        /// Term id is an entity.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const IsEntityFlag = 1u64 << 57;

        /// Term id is a name (don't attempt to lookup as entity).
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const IsNameFlag = 1u64 << 56;
    }
}

bitflags! {
    /// Flags for term references
    pub struct TermRefFlags: u64 {
        /// Match on self.
        /// Can be combined with other term flags on the ecs_term_t::flags field.
        /// \ingroup queries
        const SelfFlag = 1u64 << 63;

        /// Match by traversing upwards.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const UpFlag = 1u64 << 62;

        /// Traverse relationship transitively.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const TravFlag = 1u64 << 61;

        /// Sort results breadth first.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const CascadeFlag = 1u64 << 60;

        /// Iterate groups in descending order.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const DescFlag = 1u64 << 59;

        /// Term id is a variable.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const IsVariableFlag = 1u64 << 58;

        /// Term id is an entity.
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const IsEntityFlag = 1u64 << 57;

        /// Term id is a name (don't attempt to lookup as entity).
        /// Can be combined with other term flags on the ecs_term_ref_t::id field.
        /// \ingroup queries
        const IsNameFlag = 1u64 << 56;
    }
}
