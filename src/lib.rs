#![allow(unused)]
pub mod auto;
mod nullstr;
#[cfg(test)]
mod tests;

use bitflags::bitflags;
use flecsys::*;
pub use nullstr::*;
use std::{
    any::Any,
    ffi::{c_int, CString, NulError},
    os::raw::c_void,
    sync::OnceLock,
};
use std::{any::TypeId, ffi::CStr, marker::PhantomData};
pub struct Owned;
pub struct Borrowed<'a>(std::marker::PhantomData<&'a ()>);
pub struct BorrowedMut<'a>(std::marker::PhantomData<&'a mut ()>);
/// A helper trait to allow us to drop the owned type, but not the borrowed types.
/// The trait implementation for Owned will drop the owned type by calling the `drop_fn` argument,
/// but the implementation for Borrowed and BorrowedMut will do nothing.
pub trait MaybeDrop {
    fn maybe_drop(&mut self, drop_fn: impl FnOnce());
}
impl MaybeDrop for Owned {
    fn maybe_drop(&mut self, drop_fn: impl FnOnce()) {
        drop_fn();
    }
}
impl MaybeDrop for Borrowed<'_> {
    fn maybe_drop(&mut self, _: impl FnOnce()) {}
}
impl MaybeDrop for BorrowedMut<'_> {
    fn maybe_drop(&mut self, _: impl FnOnce()) {}
}
/// A trait that will be implemented by the opaque types to allow us to drop them.
/// The implementation should decide which function to call for dropping the type.
pub trait OpaqueType {
    /// only implement if the type actually needs to be dropped, otherwise, use `unimplemented!()`
    fn drop_my_self(ptr: *mut Self);
}
/// A helper struct to wrap the opaque types. This will allow us to drop the opaque types when they go out of scope.
/// The `T` is the "native ffi opaque type", and the drop impl will call its destructor by passing it to the Ownership
/// The `Ownership` is a phantom type that will allow us to drop the owned type, but not the borrowed types.
#[repr(C)]
pub struct FfiWrapper<T: OpaqueType, Ownership: MaybeDrop> {
    ptr: *mut T,
    ownership: Ownership,
}
impl<T, Ownership> Drop for FfiWrapper<T, Ownership>
where
    Ownership: MaybeDrop,
    T: OpaqueType,
{
    fn drop(&mut self) {
        self.ownership.maybe_drop(|| {
            T::drop_my_self(self.ptr);
        });
    }
}
pub type FfiOwned<T> = FfiWrapper<T, Owned>;
pub type FfiBorrowed<'a, T> = FfiWrapper<T, Borrowed<'a>>;
pub type FfiBorrowedMut<'a, T> = FfiWrapper<T, BorrowedMut<'a>>;

impl<T: OpaqueType> std::ops::Deref for FfiBorrowed<'_, T> {
    type Target = FfiOwned<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: OpaqueType> std::ops::Deref for FfiBorrowedMut<'_, T> {
    type Target = FfiOwned<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: OpaqueType> std::ops::DerefMut for FfiBorrowedMut<'_, T> {
    fn deref_mut(&mut self) -> &mut FfiOwned<T> {
        unsafe { std::mem::transmute(self) }
    }
}
pub type Entity = u64;
pub type Id = u64;
pub type Flags32 = u32;
pub type Flags16 = u16;
pub type Flags8 = u8;

pub type World = FfiOwned<flecsys::ecs_world_t>;
type WorldRef<'a> = FfiBorrowed<'a, flecsys::ecs_world_t>;
type WorldRefMut<'a> = FfiBorrowedMut<'a, flecsys::ecs_world_t>;
pub type Stage = FfiOwned<flecsys::ecs_stage_t>;
pub type Table = FfiOwned<flecsys::ecs_table_t>;
pub type IdRecord = FfiOwned<flecsys::ecs_id_record_t>;
pub type TableRecord = FfiOwned<flecsys::ecs_table_record_t>;
pub type Mixins = FfiOwned<flecsys::ecs_mixins_t>;
pub type Data = FfiOwned<flecsys::ecs_data_t>;
pub type QueryCacheTableMatch = FfiOwned<flecsys::ecs_query_cache_table_match_t>;
pub type HttpServer = FfiOwned<flecsys::ecs_http_server_t>;
pub type ScriptTemplate = FfiOwned<flecsys::ecs_script_template_t>;
pub trait Poly {
    fn as_poly_ptr(&self) -> *const flecsys::ecs_poly_t;
    fn as_poly_ptr_mut(&mut self) -> *mut flecsys::ecs_poly_t;
    fn get_world(&self) -> WorldRef<'_>
    where
        Self: Sized,
    {
        unsafe {
            WorldRef {
                ptr: flecsys::ecs_get_world(self.as_poly_ptr()) as _,
                ownership: Borrowed(PhantomData),
            }
        }
    }
    fn get_entity(&self) -> Entity
    where
        Self: Sized,
    {
        unsafe { flecsys::ecs_get_entity(self.as_poly_ptr()) }
    }
}
impl Poly for World {
    fn as_poly_ptr(&self) -> *const flecsys::ecs_poly_t {
        self.ptr as *const flecsys::ecs_poly_t
    }
    fn as_poly_ptr_mut(&mut self) -> *mut flecsys::ecs_poly_t {
        self.ptr as *mut flecsys::ecs_poly_t
    }
}
impl OpaqueType for flecsys::ecs_world_t {
    fn drop_my_self(ptr: *mut Self) {
        unsafe {
            flecsys::ecs_fini(ptr);
        }
    }
}

impl OpaqueType for flecsys::ecs_stage_t {
    fn drop_my_self(ptr: *mut Self) {
        unsafe {
            flecsys::ecs_stage_free(ptr as _);
        }
    }
}
impl OpaqueType for flecsys::ecs_table_t {
    fn drop_my_self(_: *mut Self) {
        unimplemented!("ecs_table_t is not supposed to be dropped");
    }
}
impl OpaqueType for flecsys::ecs_id_record_t {
    fn drop_my_self(_: *mut Self) {
        unimplemented!("ecs_id_record_t is not supposed to be dropped");
    }
}
impl OpaqueType for flecsys::ecs_table_record_t {
    fn drop_my_self(_: *mut Self) {
        unimplemented!("ecs_table_record_t is not supposed to be dropped");
    }
}
impl OpaqueType for flecsys::ecs_mixins_t {
    fn drop_my_self(_: *mut Self) {
        unimplemented!("ecs_mixins_t is not supposed to be dropped");
    }
}
impl OpaqueType for flecsys::ecs_data_t {
    fn drop_my_self(_: *mut Self) {
        unimplemented!("ecs_data_t is not supposed to be dropped");
    }
}

impl OpaqueType for flecsys::ecs_query_cache_table_match_t {
    fn drop_my_self(_: *mut Self) {
        unimplemented!("ecs_query_cache_table_match_t is not supposed to be dropped");
    }
}
impl OpaqueType for flecsys::ecs_http_server_t {
    fn drop_my_self(ptr: *mut Self) {
        unsafe { flecsys::ecs_http_server_fini(ptr) };
    }
}
pub const FLECS_VERSION_MAJOR: u32 = flecsys::FLECS_VERSION_MAJOR;
pub const FLECS_VERSION_MINOR: u32 = flecsys::FLECS_VERSION_MINOR;
pub const FLECS_VERSION_PATCH: u32 = flecsys::FLECS_VERSION_PATCH;
pub const FLECS_HI_COMPONENT_ID: u32 = flecsys::FLECS_HI_COMPONENT_ID;
pub const FLECS_HI_ID_RECORD_ID: u32 = flecsys::FLECS_HI_ID_RECORD_ID;
pub const FLECS_SPARSE_PAGE_BITS: u32 = flecsys::FLECS_SPARSE_PAGE_BITS;
pub const FLECS_ENTITY_PAGE_BITS: u32 = flecsys::FLECS_ENTITY_PAGE_BITS;
pub const FLECS_ID_DESC_MAX: u32 = flecsys::FLECS_ID_DESC_MAX;
pub const FLECS_EVENT_DESC_MAX: u32 = flecsys::FLECS_EVENT_DESC_MAX;
pub const FLECS_VARIABLE_COUNT_MAX: u32 = flecsys::FLECS_VARIABLE_COUNT_MAX;
pub const FLECS_TERM_COUNT_MAX: u32 = flecsys::FLECS_TERM_COUNT_MAX;
pub const FLECS_TERM_ARG_COUNT_MAX: u32 = flecsys::FLECS_TERM_ARG_COUNT_MAX;
pub const FLECS_QUERY_VARIABLE_COUNT_MAX: u32 = flecsys::FLECS_QUERY_VARIABLE_COUNT_MAX;
pub const FLECS_QUERY_SCOPE_NESTING_MAX: u32 = flecsys::FLECS_QUERY_SCOPE_NESTING_MAX;

pub const WORLD_T_MAGIC: u32 = flecsys::ecs_world_t_magic;
pub const STAGE_T_MAGIC: u32 = flecsys::ecs_stage_t_magic;
pub const QUERY_T_MAGIC: u32 = flecsys::ecs_query_t_magic;
pub const OBSERVER_T_MAGIC: u32 = flecsys::ecs_observer_t_magic;
pub const ROW_MASK: u32 = flecsys::ECS_ROW_MASK;
pub const ROW_FLAGS_MASK: i32 = -flecsys::ECS_ROW_FLAGS_MASK;
pub const ID_FLAGS_MASK: i64 = -flecsys::ECS_ID_FLAGS_MASK;
pub const ENTITY_MASK: u32 = flecsys::ECS_ENTITY_MASK;
pub const GENERATION_MASK: u64 = flecsys::ECS_GENERATION_MASK;
pub const COMPONENT_MASK: u64 = flecsys::ECS_COMPONENT_MASK;
pub const ITER_NEXT_YIELD: u32 = flecsys::EcsIterNextYield;
pub const ITER_YIELD: i32 = -flecsys::EcsIterYield;
pub const ITER_NEXT: u32 = flecsys::EcsIterNext;
pub const S_SPARSE_PAGE_SIZE: u32 = flecsys::FLECS_SPARSE_PAGE_SIZE;
pub const STACK_PAGE_SIZE: u32 = flecsys::ECS_STACK_PAGE_SIZE;
pub const STRBUF_SMALL_STRING_SIZE: u32 = flecsys::ECS_STRBUF_SMALL_STRING_SIZE;
pub const STRBUF_MAX_LIST_DEPTH: u32 = flecsys::ECS_STRBUF_MAX_LIST_DEPTH;
pub const SELF: i64 = flecsys::EcsSelf;
pub const UP: u64 = flecsys::EcsUp;
pub const TRAV: u64 = flecsys::EcsTrav;
pub const CASCADE: u64 = flecsys::EcsCascade;
pub const DESC: u64 = flecsys::EcsDesc;
pub const IS_VARIABLE: u64 = flecsys::EcsIsVariable;
pub const IS_ENTITY: u64 = flecsys::EcsIsEntity;
pub const IS_NAME: u64 = flecsys::EcsIsName;
pub const TRAVERSE_FLAGS: i64 = -flecsys::EcsTraverseFlags;
pub const TERM_REF_FLAGS: i64 = -flecsys::EcsTermRefFlags;
pub const MAX_RECURSION: u32 = flecsys::ECS_MAX_RECURSION;
pub const MAX_TOKEN_SIZE: u32 = flecsys::ECS_MAX_TOKEN_SIZE;
pub const QUERY_MATCH_PREFAB: u32 = flecsys::EcsQueryMatchPrefab;
pub const QUERY_MATCH_DISABLED: u32 = flecsys::EcsQueryMatchDisabled;
pub const QUERY_MATCH_EMPTY_TABLES: u32 = flecsys::EcsQueryMatchEmptyTables;
pub const QUERY_NO_DATA: u32 = flecsys::EcsQueryNoData;
pub const QUERY_IS_INSTANCED: u32 = flecsys::EcsQueryIsInstanced;
pub const QUERY_ALLOW_UNRESOLVED_BY_NAME: u32 = flecsys::EcsQueryAllowUnresolvedByName;
pub const QUERY_TABLE_ONLY: u32 = flecsys::EcsQueryTableOnly;
pub const FIRST_USER_COMPONENT_ID: u32 = flecsys::EcsFirstUserComponentId;
pub const FIRST_USER_ENTITY_ID: u32 = flecsys::EcsFirstUserEntityId;
pub const INVALID_OPERATION: u32 = flecsys::ECS_INVALID_OPERATION;
pub const INVALID_PARAMETER: u32 = flecsys::ECS_INVALID_PARAMETER;
pub const CONSTRAINT_VIOLATED: u32 = flecsys::ECS_CONSTRAINT_VIOLATED;
pub const OUT_OF_MEMORY: u32 = flecsys::ECS_OUT_OF_MEMORY;
pub const OUT_OF_RANGE: u32 = flecsys::ECS_OUT_OF_RANGE;
pub const UNSUPPORTED: u32 = flecsys::ECS_UNSUPPORTED;
pub const INTERNAL_ERROR: u32 = flecsys::ECS_INTERNAL_ERROR;
pub const ALREADY_DEFINED: u32 = flecsys::ECS_ALREADY_DEFINED;
pub const MISSING_OS_API: u32 = flecsys::ECS_MISSING_OS_API;
pub const OPERATION_FAILED: u32 = flecsys::ECS_OPERATION_FAILED;
pub const INVALID_CONVERSION: u32 = flecsys::ECS_INVALID_CONVERSION;
pub const ID_IN_USE: u32 = flecsys::ECS_ID_IN_USE;
pub const CYCLE_DETECTED: u32 = flecsys::ECS_CYCLE_DETECTED;
pub const LEAK_DETECTED: u32 = flecsys::ECS_LEAK_DETECTED;
pub const DOUBLE_FREE: u32 = flecsys::ECS_DOUBLE_FREE;
pub const INCONSISTENT_NAME: u32 = flecsys::ECS_INCONSISTENT_NAME;
pub const NAME_IN_USE: u32 = flecsys::ECS_NAME_IN_USE;
pub const NOT_A_COMPONENT: u32 = flecsys::ECS_NOT_A_COMPONENT;
pub const INVALID_COMPONENT_SIZE: u32 = flecsys::ECS_INVALID_COMPONENT_SIZE;
pub const INVALID_COMPONENT_ALIGNMENT: u32 = flecsys::ECS_INVALID_COMPONENT_ALIGNMENT;
pub const COMPONENT_NOT_REGISTERED: u32 = flecsys::ECS_COMPONENT_NOT_REGISTERED;
pub const INCONSISTENT_COMPONENT_ID: u32 = flecsys::ECS_INCONSISTENT_COMPONENT_ID;
pub const INCONSISTENT_COMPONENT_ACTION: u32 = flecsys::ECS_INCONSISTENT_COMPONENT_ACTION;
pub const MODULE_UNDEFINED: u32 = flecsys::ECS_MODULE_UNDEFINED;
pub const MISSING_SYMBOL: u32 = flecsys::ECS_MISSING_SYMBOL;
pub const ALREADY_IN_USE: u32 = flecsys::ECS_ALREADY_IN_USE;
pub const ACCESS_VIOLATION: u32 = flecsys::ECS_ACCESS_VIOLATION;
pub const COLUMN_INDEX_OUT_OF_RANGE: u32 = flecsys::ECS_COLUMN_INDEX_OUT_OF_RANGE;
pub const COLUMN_IS_NOT_SHARED: u32 = flecsys::ECS_COLUMN_IS_NOT_SHARED;
pub const COLUMN_IS_SHARED: u32 = flecsys::ECS_COLUMN_IS_SHARED;
pub const COLUMN_TYPE_MISMATCH: u32 = flecsys::ECS_COLUMN_TYPE_MISMATCH;
pub const INVALID_WHILE_READONLY: u32 = flecsys::ECS_INVALID_WHILE_READONLY;
pub const LOCKED_STORAGE: u32 = flecsys::ECS_LOCKED_STORAGE;
pub const INVALID_FROM_WORKER: u32 = flecsys::ECS_INVALID_FROM_WORKER;
pub const BLACK: &CStr = flecsys::ECS_BLACK;
pub const RED: &CStr = flecsys::ECS_RED;
pub const GREEN: &CStr = flecsys::ECS_GREEN;
pub const YELLOW: &CStr = flecsys::ECS_YELLOW;
pub const BLUE: &CStr = flecsys::ECS_BLUE;
pub const MAGENTA: &CStr = flecsys::ECS_MAGENTA;
pub const CYAN: &CStr = flecsys::ECS_CYAN;
pub const WHITE: &CStr = flecsys::ECS_WHITE;
pub const GREY: &CStr = flecsys::ECS_GREY;
pub const NORMAL: &CStr = flecsys::ECS_NORMAL;
pub const BOLD: &CStr = flecsys::ECS_BOLD;
pub const HTTP_HEADER_COUNT_MAX: u32 = flecsys::ECS_HTTP_HEADER_COUNT_MAX;
pub const HTTP_QUERY_PARAM_COUNT_MAX: u32 = flecsys::ECS_HTTP_QUERY_PARAM_COUNT_MAX;
pub const REST_DEFAULT_PORT: u32 = flecsys::ECS_REST_DEFAULT_PORT;
pub const STAT_WINDOW: u32 = flecsys::ECS_STAT_WINDOW;
pub const ALERT_MAX_SEVERITY_FILTERS: u32 = flecsys::ECS_ALERT_MAX_SEVERITY_FILTERS;
pub const MEMBER_DESC_CACHE_SIZE: u32 = flecsys::ECS_MEMBER_DESC_CACHE_SIZE;
pub const META_MAX_SCOPE_DEPTH: u32 = flecsys::ECS_META_MAX_SCOPE_DEPTH;

////////////////////////////////////////////////////////////////////////////////
//// World flags
////////////////////////////////////////////////////////////////////////////////

bitflags! {
    pub struct WorldFlags: u32 {
        const QUIT_WORKERS = flecsys::EcsWorldQuitWorkers;
        const READONLY = flecsys::EcsWorldReadonly;
        const INIT = flecsys::EcsWorldInit;
        const QUIT = flecsys::EcsWorldQuit;
        const FINI = flecsys::EcsWorldFini;
        const MEASURE_FRAME_TIME = flecsys::EcsWorldMeasureFrameTime;
        const MEASURE_SYSTEM_TIME = flecsys::EcsWorldMeasureSystemTime;
        const MULTI_THREADED = flecsys::EcsWorldMultiThreaded;
    }
}

bitflags! {
    pub struct OsApiFlags: u32 {
        const HIGH_RESOLUTION_TIMER = flecsys::EcsOsApiHighResolutionTimer;
        const LOG_WITH_COLORS = flecsys::EcsOsApiLogWithColors;
        const LOG_WITH_TIME_STAMP = flecsys::EcsOsApiLogWithTimeStamp;
        const LOG_WITH_TIME_DELTA = flecsys::EcsOsApiLogWithTimeDelta;
    }
}

bitflags! {
    pub struct EntityFlags: u32 {
        const IS_ID = flecsys::EcsEntityIsId;
        const IS_TARGET = flecsys::EcsEntityIsTarget;
        const IS_TRAVERSABLE = flecsys::EcsEntityIsTraversable;
    }
}

bitflags! {
    pub struct IdFlags: u32 {
        const ON_DELETE_REMOVE = flecsys::EcsIdOnDeleteRemove;
        const ON_DELETE_DELETE = flecsys::EcsIdOnDeleteDelete;
        const ON_DELETE_PANIC = flecsys::EcsIdOnDeletePanic;
        const ON_DELETE_MASK = flecsys::EcsIdOnDeleteMask;
        const ON_DELETE_OBJECT_REMOVE = flecsys::EcsIdOnDeleteObjectRemove;
        const ON_DELETE_OBJECT_DELETE = flecsys::EcsIdOnDeleteObjectDelete;
        const ON_DELETE_OBJECT_PANIC = flecsys::EcsIdOnDeleteObjectPanic;
        const ON_DELETE_OBJECT_MASK = flecsys::EcsIdOnDeleteObjectMask;
        const ON_INSTANTIATE_OVERRIDE = flecsys::EcsIdOnInstantiateOverride;
        const ON_INSTANTIATE_INHERIT = flecsys::EcsIdOnInstantiateInherit;
        const ON_INSTANTIATE_DONT_INHERIT = flecsys::EcsIdOnInstantiateDontInherit;
        const EXCLUSIVE = flecsys::EcsIdExclusive;
        const TRAVERSABLE = flecsys::EcsIdTraversable;
        const TAG = flecsys::EcsIdTag;
        const WITH = flecsys::EcsIdWith;
        const CAN_TOGGLE = flecsys::EcsIdCanToggle;
        const HAS_ON_ADD = flecsys::EcsIdHasOnAdd;
        const HAS_ON_REMOVE = flecsys::EcsIdHasOnRemove;
        const HAS_ON_SET = flecsys::EcsIdHasOnSet;
        const HAS_UN_SET = flecsys::EcsIdHasUnSet;
        const HAS_ON_TABLE_FILL = flecsys::EcsIdHasOnTableFill;
        const HAS_ON_TABLE_EMPTY = flecsys::EcsIdHasOnTableEmpty;
        const HAS_ON_TABLE_CREATE = flecsys::EcsIdHasOnTableCreate;
        const HAS_ON_TABLE_DELETE = flecsys::EcsIdHasOnTableDelete;
        const IS_SPARSE = flecsys::EcsIdIsSparse;
        const IS_UNION = flecsys::EcsIdIsUnion;
    }
}

bitflags! {
    /// Flags for delete policies
    pub struct IdDeletePolicy: u32 {
        const REMOVE = flecsys::EcsIdOnDeleteRemove;
        const DELETE = flecsys::EcsIdOnDeleteDelete;
        const PANIC = flecsys::EcsIdOnDeletePanic;
    }
}

bitflags! {
    /// Flags for instantiate policies
    pub struct IdInstantiatePolicy: u32 {
        const OVERRIDE = flecsys::EcsIdOnInstantiateOverride;
        const INHERIT = flecsys::EcsIdOnInstantiateInherit;
        const DONT_INHERIT = flecsys::EcsIdOnInstantiateDontInherit;
    }
}

/*
/* Utilities for converting from flags to delete policies and vice versa */
fn ecs_id_on_delete_flag(id: u32) -> u32 {
    1u32 << ((id) - EcsRemove)
}

fn ecs_id_on_delete(flags: u32) -> u32 {
    let id = (flags & Ecsu64OnDeleteMask) >> 0;
    [EcsRemove, EcsDelete, 0, EcsPanic][id as usize]
}

fn ecs_id_on_delete_target(flags: u32) -> u32 {
    let id = (flags & Ecsu64OnDeleteMask) >> 3;
    [EcsRemove, EcsDelete, 0, EcsPanic][id as usize]
}

/* Utilities for converting from flags to instantiate policies and vice versa */
fn ecs_id_on_instantiate_flag(id: u32) -> u32 {
    1u32 << (6 + ((id) - EcsOverride))
}

fn ecs_id_on_instantiate(flags: u32) -> u32 {
    let id = ((flags & Ecsu64OnInstantiateMask) >> 6) as usize;
    [EcsOverride, EcsOverride, EcsInherit, 0, EcsDontInherit][id]
}
*/

bitflags! {
    /// Flags for query events.
    pub struct IterFlags: u32 {
        /// Indicates whether the iterator contains a valid result.
        const IS_VALID = flecsys::EcsIterIsValid;
        /// Indicates whether the iterator provides (component) data.
        const NO_DATA = flecsys::EcsIterNoData;
        /// Indicates whether the iterator is instanced.
        const IS_INSTANCED = flecsys::EcsIterIsInstanced;
        /// Indicates whether the iterator has no results.
        const NO_RESULTS = flecsys::EcsIterNoResults;
        /// Indicates whether only non-this terms should be evaluated.
        const IGNORE_THIS = flecsys::EcsIterIgnoreThis;
        /// Indicates whether the iterator has conditionally set fields.
        const HAS_COND_SET = flecsys::EcsIterHasCondSet;
        /// Indicates whether the iterator should be profiled for performance.
        const PROFILE = flecsys::EcsIterProfile;
        /// Indicates whether the iterator is in trivial search mode.
        const TRIVIAL_SEARCH = flecsys::EcsIterTrivialSearch;
        /// Indicates whether the iterator is in trivial search mode with no data.
        const TRIVIAL_SEARCH_NO_DATA = flecsys::EcsIterTrivialSearchNoData;
        /// Indicates whether the iterator is in trivial test mode with constrained `$this`.
        const TRIVIAL_TEST = flecsys::EcsIterTrivialTest;
        /// Indicates whether the iterator is in trivial test mode with wildcards.
        const TRIVIAL_TEST_WILDCARD = flecsys::EcsIterTrivialTestWildcard;
        /// Indicates whether the iterator is in trivial search mode with wildcard ids.
        const TRIVIAL_SEARCH_WILDCARD = flecsys::EcsIterTrivialSearchWildcard;
        /// Indicates whether the iterator uses a cache search.
        const CACHE_SEARCH = flecsys::EcsIterCacheSearch;
        /// Indicates whether change detection for fixed in terms is done.
        const FIXED_IN_CHANGE_COMPUTED = flecsys::EcsIterFixedInChangeComputed;
        /// Indicates whether the fixed in terms have changed.
        const FIXED_IN_CHANGED = flecsys::EcsIterFixedInChanged;
        /// Indicates whether the result was skipped for change detection.
        const SKIP = flecsys::EcsIterSkip;
        /// Indicates whether the iterator uses the C++ `each` iterator.
        const CPP_EACH = flecsys::EcsIterCppEach;

        /// Indicates whether the result only populates the table.
        const TABLE_ONLY = flecsys::EcsIterTableOnly;
    }
}

////////////////////////////////////////////////////////////////////////////////
//// Event flags (used by ecs_event_decs_t::flags)
////////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Flags for query events.
    pub struct EventFlags: u32 {
        /// Indicates whether the event only populates the table.
        const TABLE_ONLY = flecsys::EcsEventTableOnly;
        /// Don't emit OnSet/UnSet for inherited ids.
        const NO_ON_SET = flecsys::EcsEventNoOnSet;
    }
}

////////////////////////////////////////////////////////////////////////////////
//// Query flags (used by ecs_query_t::flags)
////////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Flags for query events.
    pub struct QueryFlags: u32 {
        /// Query has terms with `$this` source.
        const MATCH_THIS = flecsys::EcsQueryMatchThis;
        /// Query only has terms with `$this` source.
        const MATCH_ONLY_THIS = flecsys::EcsQueryMatchOnlyThis;
        /// Query has no terms with up traversal.
        const MATCH_ONLY_SELF = flecsys::EcsQueryMatchOnlySelf;
        /// Query matches wildcards.
        const MATCH_WILDCARDS = flecsys::EcsQueryMatchWildcards;
        /// Query has conditionally set fields.
        const HAS_COND_SET = flecsys::EcsQueryHasCondSet;
        /// Query has equality predicates.
        const HAS_PRED = flecsys::EcsQueryHasPred;
        /// Query has query scopes.
        const HAS_SCOPES = flecsys::EcsQueryHasScopes;
        /// Query has terms with static source.
        const HAS_REFS = flecsys::EcsQueryHasRefs;
        /// Query has [out] terms.
        const HAS_OUT_TERMS = flecsys::EcsQueryHasOutTerms;
        /// Query has [out] terms with no `$this` source.
        const HAS_NON_THIS_OUT_TERMS = flecsys::EcsQueryHasNonThisOutTerms;
        /// Query has monitor for change detection.
        const HAS_MONITOR = flecsys::EcsQueryHasMonitor;
        /// Query can use trivial evaluation function.
        const IS_TRIVIAL = flecsys::EcsQueryIsTrivial;
        /// Query has cacheable terms.
        const HAS_CACHEABLE = flecsys::EcsQueryHasCacheable;
        /// All terms of query are cacheable.
        const IS_CACHEABLE = flecsys::EcsQueryIsCacheable;
        /// Query has `$this` table variable.
        const HAS_TABLE_THIS_VAR = flecsys::EcsQueryHasTableThisVar;
        /// Query has `$this` sparse fields.
        const HAS_SPARSE_THIS = flecsys::EcsQueryHasSparseThis;
    }
}

////////////////////////////////////////////////////////////////////////////////
//// Term flags (used by ecs_term_t::flags_)
////////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Flags for query terms.
    pub struct TermFlags: u32 {
        /// Term matches any entity.
        const MATCH_ANY = flecsys::EcsTermMatchAny;
        /// Term matches any source.
        const MATCH_ANY_SRC = flecsys::EcsTermMatchAnySrc;
        /// Term is transitive.
        const TRANSITIVE = flecsys::EcsTermTransitive;
        /// Term has reflexive (self) relationship.
        const REFLEXIVE = flecsys::EcsTermReflexive;
        /// Term has inheritance relationship.
        const ID_INHERITED = flecsys::EcsTermIdInherited;
        /// Term has trivial evaluation function.
        const IS_TRIVIAL = flecsys::EcsTermIsTrivial;
        /// Term has no data.
        const NO_DATA = flecsys::EcsTermNoData;
        /// Term is cacheable.
        const IS_CACHEABLE = flecsys::EcsTermIsCacheable;
        /// Term is a scope.
        const IS_SCOPE = flecsys::EcsTermIsScope;
        /// Term is a member.
        const IS_MEMBER = flecsys::EcsTermIsMember;
        /// Term is a toggle.
        const IS_TOGGLE = flecsys::EcsTermIsToggle;
        /// Term keeps alive.
        const KEEP_ALIVE = flecsys::EcsTermKeepAlive;
        /// Term is sparse.
        const IS_SPARSE = flecsys::EcsTermIsSparse;
        /// Term is a union.
        const IS_UNION = flecsys::EcsTermIsUnion;
        /// Term is an or term.
        const IS_OR = flecsys::EcsTermIsOr;
    }
}

////////////////////////////////////////////////////////////////////////////////
//// Observer flags (used by ecs_observer_t::flags)
////////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Flags for observers.
    pub struct ObserverFlags: u32 {
        /// Observer has multiple terms.
        const IS_MULTI = flecsys::EcsObserverIsMulti;
        /// Observer is a monitor.
        const IS_MONITOR = flecsys::EcsObserverIsMonitor;
        /// Observer entity is disabled.
        const IS_DISABLED = flecsys::EcsObserverIsDisabled;
        ///  Is module parent of observer disabled
        const IS_PARENT_DISABLE = flecsys::EcsObserverIsParentDisabled;
    }
}
////////////////////////////////////////////////////////////////////////////////
//// Table flags (used by ecs_table_t::flags)
////////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Flags for tables.
    pub struct TableFlags: u32 {
        /// Does table have builtin components.
        const HAS_BUILTINS = flecsys::EcsTableHasBuiltins;
        /// Does the table store prefabs.
        const IS_PREFAB = flecsys::EcsTableIsPrefab;
        /// Does the table have IsA relationship.
        const HAS_ISA = flecsys::EcsTableHasIsA;
        /// Does the table type ChildOf relationship.
        const HAS_CHILD_OF = flecsys::EcsTableHasChildOf;
        /// Does the table type have (u64entifier, Name)
        const HAS_NAME = flecsys::EcsTableHasName;
        /// Does the table type have pairs.
        const HAS_PAIRS = flecsys::EcsTableHasPairs;
        /// Does the table have module data.
        const HAS_MODULE = flecsys::EcsTableHasModule;
        /// Does the table entity has EcsDisabled.
        const IS_DISABLED = flecsys::EcsTableIsDisabled;
        /// Table should never be returned by queries.
        const NOT_QUERYABLE = flecsys::EcsTableNotQueryable;
        /// Does the table type have Construct/Destruct actions.
        const HAS_CTORS = flecsys::EcsTableHasCtors;
        /// Does the table type have Destruct actions.
        const HAS_DTORS = flecsys::EcsTableHasDtors;
        /// Does the table type have Copy actions.
        const HAS_COPY = flecsys::EcsTableHasCopy;
        /// Does the table type have Move actions.
        const HAS_MOVE = flecsys::EcsTableHasMove;
        /// Does the table type have Toggle actions.
        const HAS_TOGGLE = flecsys::EcsTableHasToggle;
        /// Does the table type have Overrides.
        const HAS_OVERRIDES = flecsys::EcsTableHasOverrides;
        /// Does the table type have OnAdd actions.
        const HAS_ON_ADD = flecsys::EcsTableHasOnAdd;
        /// Does the table type have OnRemove actions.
        const HAS_ON_REMOVE = flecsys::EcsTableHasOnRemove;
        /// Does the table type have OnSet actions.
        const HAS_ON_SET = flecsys::EcsTableHasOnSet;
        /// Does the table type have UnSet actions.
        const HAS_UN_SET = flecsys::EcsTableHasUnSet;
        /// Does the table type have OnTableFill actions.
        const HAS_ON_TABLE_FILL = flecsys::EcsTableHasOnTableFill;
        /// Does the table type have OnTableEmpty actions.
        const HAS_ON_TABLE_EMPTY = flecsys::EcsTableHasOnTableEmpty;
        /// Does the table type have OnTableCreate actions.
        const HAS_ON_TABLE_CREATE = flecsys::EcsTableHasOnTableCreate;
        /// Does the table type have OnTableDelete actions.
        const HAS_ON_TABLE_DELETE = flecsys::EcsTableHasOnTableDelete;
        /// Does the table type have Sparse.
        const HAS_SPARSE = flecsys::EcsTableHasSparse;
        /// Does the table type have Union.
        const HAS_UNION = flecsys::EcsTableHasUnion;
        /// Does the table type have Traversable.
        const HAS_TRAVERSABLE = flecsys::EcsTableHasTraversable;
        /// Table is marked for deletion.
        const MARKED_FOR_DELETE = flecsys::EcsTableMarkedForDelete;
    }
}

////////////////////////////////////////////////////////////////////////////////
//// Aperiodic action flags (used by ecs_run_aperiodic)
////////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// Flags for aperiodic actions.
    pub struct AperiodicFlags: u32 {
        /// Process pending empty table events.
        const EMPTY_TABLES = EcsAperiodicEmptyTables;
        /// Process component monitors.
        const COMPONENT_MONITORS = EcsAperiodicComponentMonitors;
        /// Process empty queries.
        const EMPTY_QUERIES = EcsAperiodicEmptyQueries;
    }
}
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

pub trait HttpServerTraitManual {
    fn as_ptr(&self) -> *const flecsys::ecs_http_server_t;
    fn as_ptr_mut(&mut self) -> *mut flecsys::ecs_http_server_t;
}
impl HttpServerTraitManual for HttpServer {
    fn as_ptr(&self) -> *const flecsys::ecs_http_server_t {
        self.ptr
    }
    fn as_ptr_mut(&mut self) -> *mut flecsys::ecs_http_server_t {
        self.ptr
    }
}
pub trait TableTraitManual {
    fn as_ptr(&self) -> *const flecsys::ecs_table_t;
    fn as_ptr_mut(&mut self) -> *mut flecsys::ecs_table_t;
}
impl TableTraitManual for Table {
    fn as_ptr(&self) -> *const flecsys::ecs_table_t {
        self.ptr
    }
    fn as_ptr_mut(&mut self) -> *mut flecsys::ecs_table_t {
        self.ptr
    }
}
impl auto::TableTrait for Table {}
pub trait WorldTraitManual {
    fn as_ptr(&self) -> *const flecsys::ecs_world_t;
    fn as_ptr_mut(&mut self) -> *mut flecsys::ecs_world_t;
    #[doc = " Returns whether the world is being deleted.\n This operation can be used in callbacks like type hooks or observers to\n detect if they are invoked while the world is being deleted.\n\n @param world The world.\n @return True if being deleted, false if not."]
    fn is_finishing(&self) -> bool {
        unsafe { ecs_is_fini(self.as_ptr()) }
    }
    #[doc = " Register action to be executed when world is destroyed.\n Fini actions are typically used when a module needs to clean up before a\n world shuts down.\n\n @param world The world.\n @param action The function to execute.\n @param data Userdata to pass to the function"]
    fn at_finish<T: Send + Sync + 'static + Sized>(&mut self, action: fn(&mut World, T), data: T) {
        let data = Box::leak(Box::new((action, data))) as *mut (fn(&mut World, T), T);
        unsafe {
            ecs_atfini(
                self.as_ptr_mut(),
                Some(trampoline_world_userdata::<T>),
                data as *mut std::ffi::c_void,
            );
        }
    }
    #[doc = " Begin frame.\n When an application does not use ecs_progress() to control the main loop, it\n can still use Flecs features such as FPS limiting and time measurements. This\n operation needs to be invoked whenever a new frame is about to get processed.\n\n Calls to ecs_frame_begin() must always be followed by ecs_frame_end().\n\n The function accepts a delta_time parameter, which will get passed to\n systems. This value is also used to compute the amount of time the function\n needs to sleep to ensure it does not exceed the target_fps, when it is set.\n When 0 is provided for delta_time, the time will be measured.\n\n This function should only be ran from the main thread.\n\n @param world The world.\n @param delta_time Time elapsed since the last frame.\n @return The provided delta_time, or measured time if 0 was provided."]
    fn frame_begin(&mut self, delta_time: f32) -> f32 {
        unsafe { ecs_frame_begin(self.as_ptr_mut(), delta_time) }
    }
    #[doc = " Register action to be executed once after frame.\n Post frame actions are typically used for calling operations that cannot be\n invoked during iteration, such as changing the number of threads.\n\n @param world The world.\n @param action The function to execute.\n @param ctx Userdata to pass to the function"]
    fn run_post_frame<T: Send + Sync + 'static + Sized>(
        &mut self,
        action: fn(&mut World, T),
        data: T,
    ) {
        let data = Box::leak(Box::new((action, data))) as *mut (fn(&mut World, T), T);

        unsafe {
            ecs_run_post_frame(
                self.as_ptr_mut(),
                Some(trampoline_world_userdata::<T>),
                data as *mut std::ffi::c_void,
            );
        }
    }
    #[doc = " Set a world context.\n This operation allows an application to register custom data with a world\n that can be accessed anywhere where the application has the world.\n\n @param world The world.\n @param ctx A pointer to a user defined structure.\n @param ctx_free A function that is invoked with ctx when the world is freed."]
    fn set_ctx<T: 'static + Send + Sync + Any>(&mut self, ctx: Option<T>) {
        let ctx = match ctx {
            Some(ctx) => {
                let ctx: Box<dyn Any> = Box::new(ctx);
                Box::leak(Box::new(ctx)) as *mut _ as *mut _
            }
            None => std::ptr::null_mut(),
        };
        unsafe { ecs_set_ctx(self.as_ptr_mut(), ctx, Some(drop_trampoline_userdata)) }
    }
    #[doc = " Set a world binding context.\n Same as ecs_set_ctx() but for binding context. A binding context is intended\n specifically for language bindings to store binding specific data.\n\n @param world The world.\n @param ctx A pointer to a user defined structure.\n @param ctx_free A function that is invoked with ctx when the world is freed."]
    fn set_binding_ctx<T: 'static + Send + Sync + Any>(&mut self, ctx: T) {
        let ctx: Box<dyn Any> = Box::new(ctx);
        let ctx = Box::leak(Box::new(ctx)) as *mut _;
        unsafe {
            ecs_set_binding_ctx(
                self.as_ptr_mut(),
                ctx as *mut ::std::os::raw::c_void,
                Some(drop_trampoline_userdata),
            )
        }
    }
    #[doc = " Get the world context.\n This operation retrieves a previously set world context.\n\n @param world The world.\n @return The context set with ecs_set_ctx(). If no context was set, the\n         function returns NULL."]
    fn get_ctx<T: 'static + Send + Sync + Any>(&self) -> Option<&T> {
        unsafe {
            (ecs_get_ctx(self.as_ptr()) as *mut Box<dyn Any>)
                .as_ref()?
                .downcast_ref()
        }
    }
    #[doc = " Get the world binding context.\n This operation retrieves a previously set world binding context.\n\n @param world The world.\n @return The context set with ecs_set_binding_ctx(). If no context was set, the\n         function returns NULL."]
    fn get_binding_ctx<T: 'static + Send + Sync + Any>(&self) -> Option<&T> {
        unsafe {
            (ecs_get_binding_ctx(self.as_ptr()) as *mut Box<dyn Any>)
                .as_ref()?
                .downcast_ref()
        }
    }
    #[doc = " Get build info.\n  Returns information about the current Flecs build.\n\n @return A struct with information about the current Flecs build."]
    fn get_build_info() -> &'static BuildInfo {
        BuildInfo::get_static()
    }
    #[doc = " Get world info.\n\n @param world The world.\n @return Pointer to the world info. Valid for as long as the world exists."]
    fn get_world_info(&self) -> WorldInfo {
        let info_ptr = unsafe { ecs_get_world_info(self.as_ptr()) };
        assert!(!info_ptr.is_null());

        let info = unsafe { &*info_ptr };

        WorldInfo {
            last_component_id: info.last_component_id,
            min_id: info.min_id,
            max_id: info.max_id,
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
            name_prefix: &unsafe { NullStr::from_ptr(info.name_prefix) }.unwrap(),
        }
    }
    /*
    #[doc = " Find or create an entity.\n This operation creates a new entity, or modifies an existing one. When a name\n is set in the ecs_entity_desc_t::name field and ecs_entity_desc_t::entity is\n not set, the operation will first attempt to find an existing entity by that\n name. If no entity with that name can be found, it will be created.\n\n If both a name and entity handle are provided, the operation will check if\n the entity name matches with the provided name. If the names do not match,\n the function will fail and return 0.\n\n If an id to a non-existing entity is provided, that entity id become alive.\n\n See the documentation of ecs_entity_desc_t for more details.\n\n @param world The world.\n @param desc Entity init parameters.\n @return A handle to the new or existing entity, or 0 if failed."]
    fn entity_init(&mut self, desc: &EntityDesc) -> u64 {
        unsafe { ecs_entity_init(self.as_ptr_mut(), desc) }
    }
    #[doc = " Bulk create/populate new entities.\n This operation bulk inserts a list of new or predefined entities into a\n single table.\n\n The operation does not take ownership of component arrays provided by the\n application. Components that are non-trivially copyable will be moved into\n the storage.\n\n The operation will emit OnAdd events for each added id, and OnSet events for\n each component that has been set.\n\n If no entity ids are provided by the application, the returned array of ids\n points to an internal data structure which changes when new entities are\n created/deleted.\n\n If as a result of the operation triggers are invoked that deletes\n entities and no entity ids were provided by the application, the returned\n array of identifiers may be incorrect. To avoid this problem, an application\n can first call ecs_bulk_init() to create empty entities, copy the array to one\n that is owned by the application, and then use this array to populate the\n entities.\n\n @param world The world.\n @param desc Bulk creation parameters.\n @return Array with the list of entity ids created/populated."]
    pub fn ecs_bulk_init(
        world: *mut ecs_world_t,
        desc: *const ecs_bulk_desc_t,
    ) -> *const ecs_entity_t;
    */
    #[doc = " Create N new entities.\n This operation is the same as ecs_new_w_id(), but creates N entities\n instead of one.\n\n @param world The world.\n @param id The component id to create the entities with.\n @param count The number of entities to create.\n @return The first entity id of the newly created entities."]
    fn bulk_new_w_id(&mut self, id: u64, count: u32) -> &[Entity] {
        unsafe {
            let entities = ecs_bulk_new_w_id(self.as_ptr_mut(), id, count.try_into().unwrap());
            std::slice::from_raw_parts(entities, count as _)
        }
    }
    #[doc = " Get the type of an entity.\n\n @param world The world.\n @param entity The entity.\n @return The type of the entity, NULL if the entity has no components."]
    fn get_type(&self, entity: u64) -> Option<&[u64]> {
        unsafe {
            let etype = ecs_get_type(self.as_ptr(), entity);
            if !etype.is_null() {
                let result = std::slice::from_raw_parts((*etype).array, (*etype).count as usize);
                Some(result)
            } else {
                None
            }
        }
    }
    #[doc = " Get an immutable pointer to a component.\n This operation obtains a const pointer to the requested component. The\n operation accepts the component entity id.\n\n This operation can return inherited components reachable through an IsA\n relationship.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component to get.\n @return The component pointer, NULL if the entity does not have the component."]
    fn get<T>(&self, entity: u64, id: u64) -> Option<&T> {
        unsafe { (ecs_get_id(self.as_ptr(), entity, id) as *const T).as_ref() }
    }
    #[doc = " Get a mutable pointer to a component.\n This operation obtains a mutable pointer to the requested component. The\n operation accepts the component entity id.\n\n Unlike ecs_get_id, this operation does not return inherited components.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component to get.\n @return The component pointer, NULL if the entity does not have the component."]
    fn get_mut<T: 'static>(&mut self, entity: u64) -> Option<&mut T> {
        let id = get_id_of_type::<T>(self.as_ptr())?;
        unsafe { (ecs_get_mut_id(self.as_ptr(), entity, id) as *mut T).as_mut() }
    }
    #[doc = " Get a mutable pointer to a component.\n This operation returns a mutable pointer to a component. If the component did\n not yet exist, it will be added.\n\n If ensure is called when the world is in deferred/readonly mode, the\n function will:\n - return a pointer to a temp storage if the component does not yet exist, or\n - return a pointer to the existing component if it exists\n\n @param world The world.\n @param entity The entity.\n @param id The entity id of the component to obtain.\n @return The component pointer."]
    fn ensure<T: 'static>(&mut self, entity: u64) -> Option<&mut T> {
        let id = get_id_of_type::<T>(self.as_ptr())?;
        unsafe { (ecs_ensure_id(self.as_ptr_mut(), entity, id) as *mut T).as_mut() }
    }
    #[doc = " Combines ensure + modified in single operation.\n This operation is a more efficient alternative to calling ecs_ensure_id() and\n ecs_modified_id() separately. This operation is only valid when the world is in\n deferred mode, which ensures that the Modified event is not emitted before\n the modification takes place.\n\n @param world The world.\n @param entity The entity.\n @param id The id of the component to obtain.\n @return The component pointer."]
    fn ensure_modified<T: 'static>(&mut self, entity: u64) -> Option<&mut T> {
        let id = get_id_of_type::<T>(self.as_ptr())?;
        unsafe { (ecs_ensure_modified_id(self.as_ptr_mut(), entity, id) as *mut T).as_mut() }
    }
}
struct BindingContext {
    components: std::collections::HashMap<TypeId, u64, NoOpHasher>,
}
impl BindingContext {
    pub fn get_component_id<T: 'static>(&self) -> Option<u64> {
        self.components.get(&TypeId::of::<T>()).copied()
    }
}
struct NoOpHasher(u64);
impl std::hash::BuildHasher for NoOpHasher {
    type Hasher = NoOpHasher;
    fn build_hasher(&self) -> Self {
        NoOpHasher(0)
    }
}
impl std::hash::Hasher for NoOpHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _bytes: &[u8]) {}
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}
fn get_id_of_type<T: 'static>(world: *const ecs_world_t) -> Option<u64> {
    WorldRef {
        ptr: world.cast_mut(),
        ownership: Borrowed(PhantomData),
    }
    .get_binding_ctx::<BindingContext>()?
    .get_component_id::<T>()
}
unsafe extern "C" fn trampoline_world_userdata<T>(
    world: *mut ecs_world_t,
    ctx: *mut std::ffi::c_void,
) {
    let (f, data) = *Box::from_raw(ctx as *mut (fn(&mut World, T), T));
    // we must not take ownership of world here.
    let mut world_mut = WorldRefMut {
        ptr: world,
        ownership: BorrowedMut(PhantomData),
    };

    (f)(&mut world_mut, data);
}
unsafe extern "C" fn drop_trampoline_userdata(ctx: *mut std::ffi::c_void) {
    if ctx.is_null() {
        return;
    }
    let _ = Box::from_raw(ctx as *mut Box<dyn Any>);
}
impl WorldTraitManual for World {
    fn as_ptr(&self) -> *const flecsys::ecs_world_t {
        self.ptr
    }
    fn as_ptr_mut(&mut self) -> *mut flecsys::ecs_world_t {
        self.ptr
    }
}
impl auto::WorldTrait for World {}

impl Default for World {
    #[doc = " Create a new world.\n This operation automatically imports modules from addons Flecs has been built\n with, except when the module specifies otherwise.\n\n @return A new world"]
    fn default() -> Self {
        Self {
            ptr: unsafe { ecs_init() },
            ownership: Owned,
        }
    }
}
impl World {
    #[doc = " Create a new world with just the core module.\n Same as ecs_init(), but doesn't import modules from addons. This operation is\n faster than ecs_init() and results in less memory utilization.\n\n @return A new tiny world"]
    pub fn mini() -> Self {
        Self {
            ptr: unsafe { ecs_mini() },
            ownership: Owned,
        }
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
        Ok(Self {
            ptr: unsafe { ecs_init_w_args(argc, argv.as_ptr() as _) },
            ownership: Owned,
        })
    }
}
pub fn make_pair(id: u64, id2: u64) -> u64 {
    unsafe { flecsys::ecs_make_pair(id, id2) }
}
/*
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
*/
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
    pub last_component_id: u64,
    #[doc = "< First allowed entity id"]
    pub min_id: u64,
    #[doc = "< Last allowed entity id"]
    pub max_id: u64,
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
/// Builder for entity description which can be used with `entity_init` method
pub struct EntityDescBuilder<'a> {
    desc: flecsys::ecs_entity_desc_t,
    _phantom: PhantomData<&'a ()>,
}
impl Default for EntityDescBuilder<'static> {
    fn default() -> Self {
        Self {
            desc: unsafe { std::mem::zeroed() },
            _phantom: PhantomData,
        }
    }
}
impl<'a> EntityDescBuilder<'a> {
    pub fn id(self, id: u64) -> Self {
        Self {
            desc: flecsys::ecs_entity_desc_t { id, ..self.desc },
            _phantom: PhantomData,
        }
    }
    pub fn parent(self, parent: u64) -> Self {
        Self {
            desc: flecsys::ecs_entity_desc_t {
                parent,
                ..self.desc
            },
            _phantom: PhantomData,
        }
    }
    pub fn name(self, name: &'a NullStr) -> Self {
        Self {
            desc: flecsys::ecs_entity_desc_t {
                name: name.as_ptr(),
                ..self.desc
            },
            _phantom: PhantomData,
        }
    }
    pub fn sep(self, sep: &'a NullStr) -> Self {
        Self {
            desc: flecsys::ecs_entity_desc_t {
                sep: sep.as_ptr(),
                ..self.desc
            },
            _phantom: PhantomData,
        }
    }
    pub fn root_sep(self, root_sep: &'a NullStr) -> Self {
        Self {
            desc: flecsys::ecs_entity_desc_t {
                root_sep: root_sep.as_ptr(),
                ..self.desc
            },
            _phantom: PhantomData,
        }
    }
    pub fn symbol(self, symbol: &'a NullStr) -> Self {
        Self {
            desc: flecsys::ecs_entity_desc_t {
                symbol: symbol.as_ptr(),
                ..self.desc
            },
            _phantom: PhantomData,
        }
    }
    pub fn use_low_id(self, use_low_id: bool) -> Self {
        Self {
            desc: flecsys::ecs_entity_desc_t {
                use_low_id,
                ..self.desc
            },
            _phantom: PhantomData,
        }
    }
}
