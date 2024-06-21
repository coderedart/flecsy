//! Bindings generator for flecs
//!
//! This crate does multiple things:
//! 1. generate bindings for native flecs. This is useful if you are just using it natively with rust
//! 2. generate bindings for wasm flecs. This is to "separate" bindings for wasm and native flecs
//! 3. generate bindings for wasm from hostside using wasmtime has the reference.
//!
mod generator;

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use bindgen::{callbacks::ParseCallbacks, Builder};

pub fn generate_bindings() -> Result<()> {
    let bindings = Builder::default()
        .header("./flecs.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate_cstr(true)
        .merge_extern_blocks(true)
        .layout_tests(false)
        .array_pointers_in_arguments(true)
        .wrap_unsafe_ops(true)
        .prepend_enum_name(false)
        // .parse_callbacks(Box::new(FlecsParseCB::default()))
        .generate()
        .expect("Unable to generate bindings")
        .to_string();
    generator::generate_safe_wrappers(&bindings).context("failed to generate safe wrappers")?;
    std::fs::write("./crates/flecsys/src/lib.rs", bindings)
        .context("failed to write raw bindings.flecsys/lib.rs")?;
    Ok(())
}
pub enum BindingsPlatform {
    Native,
    Wasm,
    WasmHost,
    LuaHost,
}
#[derive(Debug)]
struct FlecsParseCB {
    pub consts: BTreeMap<&'static str, String>,
}
impl FlecsParseCB {
    const CONSTANTS_WITH_NON_UPPER_NAMES: &'static [&'static str] = &[
        "EcsWorldQuitWorkers",
        "EcsWorldReadonly",
        "EcsWorldInit",
        "EcsWorldQuit",
        "EcsWorldFini",
        "EcsWorldMeasureFrameTime",
        "EcsWorldMeasureSystemTime",
        "EcsWorldMultiThreaded",
        "EcsOsApiHighResolutionTimer",
        "EcsOsApiLogWithColors",
        "EcsOsApiLogWithTimeStamp",
        "EcsOsApiLogWithTimeDelta",
        "EcsEntityIsId",
        "EcsEntityIsTarget",
        "EcsEntityIsTraversable",
        "EcsIdOnDeleteRemove",
        "EcsIdOnDeleteDelete",
        "EcsIdOnDeletePanic",
        "EcsIdOnDeleteMask",
        "EcsIdOnDeleteObjectRemove",
        "EcsIdOnDeleteObjectDelete",
        "EcsIdOnDeleteObjectPanic",
        "EcsIdOnDeleteObjectMask",
        "EcsIdOnInstantiateOverride",
        "EcsIdOnInstantiateInherit",
        "EcsIdOnInstantiateDontInherit",
        "EcsIdOnInstantiateMask",
        "EcsIdExclusive",
        "EcsIdTraversable",
        "EcsIdTag",
        "EcsIdWith",
        "EcsIdCanToggle",
        "EcsIdHasOnAdd",
        "EcsIdHasOnRemove",
        "EcsIdHasOnSet",
        "EcsIdHasUnSet",
        "EcsIdHasOnTableFill",
        "EcsIdHasOnTableEmpty",
        "EcsIdHasOnTableCreate",
        "EcsIdHasOnTableDelete",
        "EcsIdIsSparse",
        "EcsIdIsUnion",
        "EcsIdEventMask",
        "EcsIdMarkedForDelete",
        "EcsIterIsValid",
        "EcsIterNoData",
        "EcsIterIsInstanced",
        "EcsIterNoResults",
        "EcsIterIgnoreThis",
        "EcsIterHasCondSet",
        "EcsIterProfile",
        "EcsIterTrivialSearch",
        "EcsIterTrivialSearchNoData",
        "EcsIterTrivialTest",
        "EcsIterTrivialTestWildcard",
        "EcsIterTrivialSearchWildcard",
        "EcsIterCacheSearch",
        "EcsIterFixedInChangeComputed",
        "EcsIterFixedInChanged",
        "EcsIterSkip",
        "EcsIterCppEach",
        "EcsIterTableOnly",
        "EcsEventTableOnly",
        "EcsEventNoOnSet",
        "EcsQueryMatchThis",
        "EcsQueryMatchOnlyThis",
        "EcsQueryMatchOnlySelf",
        "EcsQueryMatchWildcards",
        "EcsQueryHasCondSet",
        "EcsQueryHasPred",
        "EcsQueryHasScopes",
        "EcsQueryHasRefs",
        "EcsQueryHasOutTerms",
        "EcsQueryHasNonThisOutTerms",
        "EcsQueryHasMonitor",
        "EcsQueryIsTrivial",
        "EcsQueryHasCacheable",
        "EcsQueryIsCacheable",
        "EcsQueryHasTableThisVar",
        "EcsQueryHasSparseThis",
        "EcsTermMatchAny",
        "EcsTermMatchAnySrc",
        "EcsTermTransitive",
        "EcsTermReflexive",
        "EcsTermIdInherited",
        "EcsTermIsTrivial",
        "EcsTermNoData",
        "EcsTermIsCacheable",
        "EcsTermIsScope",
        "EcsTermIsMember",
        "EcsTermIsToggle",
        "EcsTermKeepAlive",
        "EcsTermIsSparse",
        "EcsTermIsUnion",
        "EcsTermIsOr",
        "EcsObserverIsMulti",
        "EcsObserverIsMonitor",
        "EcsObserverIsDisabled",
        "EcsObserverIsParentDisabled",
        "EcsTableHasBuiltins",
        "EcsTableIsPrefab",
        "EcsTableHasIsA",
        "EcsTableHasChildOf",
        "EcsTableHasName",
        "EcsTableHasPairs",
        "EcsTableHasModule",
        "EcsTableIsDisabled",
        "EcsTableNotQueryable",
        "EcsTableHasCtors",
        "EcsTableHasDtors",
        "EcsTableHasCopy",
        "EcsTableHasMove",
        "EcsTableHasToggle",
        "EcsTableHasOverrides",
        "EcsTableHasOnAdd",
        "EcsTableHasOnRemove",
        "EcsTableHasOnSet",
        "EcsTableHasUnSet",
        "EcsTableHasOnTableFill",
        "EcsTableHasOnTableEmpty",
        "EcsTableHasOnTableCreate",
        "EcsTableHasOnTableDelete",
        "EcsTableHasSparse",
        "EcsTableHasUnion",
        "EcsTableHasTraversable",
        "EcsTableMarkedForDelete",
        "EcsTableHasLifecycle",
        "EcsTableIsComplex",
        "EcsTableHasAddActions",
        "EcsTableHasRemoveActions",
        "EcsAperiodicEmptyTables",
        "EcsAperiodicComponentMonitors",
        "EcsAperiodicEmptyQueries",
        "EcsSelf",
        "EcsUp",
        "EcsTrav",
        "EcsCascade",
        "EcsDesc",
        "EcsIsVariable",
        "EcsIsEntity",
        "EcsIsName",
        "EcsTraverseFlags",
        "EcsTermRefFlags",
        "EcsQueryMatchPrefab",
        "EcsQueryMatchDisabled",
        "EcsQueryMatchEmptyTables",
        "EcsQueryNoData",
        "EcsQueryIsInstanced",
        "EcsQueryAllowUnresolvedByName",
        "EcsQueryTableOnly",
        "EcsFirstUserComponentId",
        "EcsFirstUserEntityId",
        "EcsIterNextYield",
        "EcsIterYield",
        "EcsIterNext",
        "ecs_world_t_magic",
        "ecs_stage_t_magic",
        "ecs_query_t_magic",
        "ecs_observer_t_magic",
    ];
}
impl Default for FlecsParseCB {
    fn default() -> Self {
        Self {
            consts: Self::CONSTANTS_WITH_NON_UPPER_NAMES
                .into_iter()
                .map(|k| (*k, heck::AsShoutySnakeCase(k).to_string()))
                .collect(),
        }
    }
}
impl ParseCallbacks for FlecsParseCB {
    // fn enum_variant_name(
    //     &self,
    //     _enum_name: Option<&str>,
    //     _original_variant_name: &str,
    //     _variant_value: bindgen::callbacks::EnumVariantValue,
    // ) -> Option<String> {
    //     None
    // }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        // if let Some(v) = self.consts.get(original_item_name) {
        //     println!("cargo:warning={v}");
        //     return Some(v.clone());
        // }
        None
    }

    fn add_derives(&self, _info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        vec![]
    }
}
