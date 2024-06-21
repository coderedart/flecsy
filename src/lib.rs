mod auto;
mod nullstr;
mod vec;
mod world;

use std::ffi::CStr;

use bitflags::bitflags;
use flecsys::*;
pub use nullstr::*;
pub use vec::*;
pub use world::*;

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
    let id = (flags & EcsIdOnDeleteMask) >> 0;
    [EcsRemove, EcsDelete, 0, EcsPanic][id as usize]
}

fn ecs_id_on_delete_target(flags: u32) -> u32 {
    let id = (flags & EcsIdOnDeleteMask) >> 3;
    [EcsRemove, EcsDelete, 0, EcsPanic][id as usize]
}

/* Utilities for converting from flags to instantiate policies and vice versa */
fn ecs_id_on_instantiate_flag(id: u32) -> u32 {
    1u32 << (6 + ((id) - EcsOverride))
}

fn ecs_id_on_instantiate(flags: u32) -> u32 {
    let id = ((flags & EcsIdOnInstantiateMask) >> 6) as usize;
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
        /// Does the table type have (Identifier, Name)
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

// TODO: copied from `ckia` bindings. maybe useful for `flecsys` too? not used at the moment.
macro_rules! pod_struct {
    ($svis: vis $name: ident, $opaque: ident {
        $($vis: vis $field: ident: $fty: ty ,)+
    }) => {
        /*
        #[derive(Debug, Copy, Clone)]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
        #[repr(C)]
        $svis struct $name {
            $($vis $field : $fty,)+
        }

        impl AsRef<$opaque> for $name {
            fn as_ref(&self) -> &$opaque {
                unsafe{std::mem::transmute(self)}
            }
        }
        impl AsMut<$opaque> for $name {
            fn as_mut(&mut self) -> &mut $opaque {
                unsafe {std::mem::transmute(self)}
            }
        }
        impl std::borrow::Borrow<$opaque> for $name {
            fn borrow(&self) -> &$opaque {
                unsafe {std::mem::transmute(self)}
            }
        }
        impl std::borrow::BorrowMut<$opaque> for $name {
            fn borrow_mut(&mut self) -> &mut $opaque {
                unsafe{std::mem::transmute(self)}
            }
        }
        */
        pub type $name = $opaque;
        #[allow(unused)]
        impl $name {
            pub(crate) fn as_ptr(&self) -> *const $opaque {
                self as * const Self as _
            }
            pub(crate) fn as_ptr_mut(&mut self) -> * mut $opaque {
                self as * mut Self as _
            }
            pub const fn into_native(&self) -> $opaque {
                $opaque {
                    $( $field : self. $field,)+
                }
            }
            $(
                paste::paste!(
                    $vis fn [<get_ $field>](&self) -> $fty {
                        self.$field
                    }
                    $vis fn [<set_ $field>](&mut self, $field: $fty) {
                        self.$field = $field;
                    }
                );
            )+
        }
        impl crate::SkiaOptPtr<$opaque> for Option<$name> {
            fn or_null(&self) -> *const $opaque {
                self.map(|s| s.as_ptr()).unwrap_or(std::ptr::null())
            }
        }
        impl crate::SkiaOptPtr<$opaque> for Option<&$name> {
            fn or_null(&self) -> *const $opaque {
                self.map(|s| s.as_ptr()).unwrap_or(std::ptr::null())
            }
        }
        impl crate::SkiaOptPtrMut<$opaque> for Option<&mut $name> {
            fn or_null_mut(self) -> *mut $opaque {
                match self {
                    Some(m) => m.as_ptr_mut(),
                    None => std::ptr::null_mut(),
                }
            }
        }
        paste::paste!(
        #[cfg(test)]
        #[test]
        fn [<$opaque _layout_tests>]() {
            assert_eq!(std::mem::size_of::<$name>(), std::mem::size_of::<$opaque>());
            assert_eq!(std::mem::align_of::<$name>(), std::mem::align_of::<$opaque>());
            // get a pointer to $name
            const UNINIT: ::std::mem::MaybeUninit<$name> =
            ::std::mem::MaybeUninit::uninit();
            let ptr = UNINIT.as_ptr();
            // get a pointer to $opaque
            const UNINIT_OPAQUE: ::std::mem::MaybeUninit<$opaque> =
            ::std::mem::MaybeUninit::uninit();
            let opaque_ptr = UNINIT_OPAQUE.as_ptr();
            // for each field, assert that the field offset (by subtracting field's address from struct's address) for all fields are same in both structs
            $(
                assert_eq!(
                    unsafe { ::std::ptr::addr_of!((*ptr).$field) as usize - ptr as usize },
                    unsafe { ::std::ptr::addr_of!((*opaque_ptr).$field) as usize - ptr as usize }
                );
            )+
        });
    };
}
