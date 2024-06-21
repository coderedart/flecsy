pub use flecsys::ecs_block_allocator_alloc_count;
pub use flecsys::ecs_block_allocator_free_count;
pub use flecsys::ecs_http_busy_count;
pub use flecsys::ecs_http_request_handled_error_count;
pub use flecsys::ecs_http_request_handled_ok_count;
pub use flecsys::ecs_http_request_invalid_count;
pub use flecsys::ecs_http_request_not_handled_count;
pub use flecsys::ecs_http_request_preflight_count;
pub use flecsys::ecs_http_request_received_count;
pub use flecsys::ecs_http_send_error_count;
pub use flecsys::ecs_http_send_ok_count;
pub use flecsys::ecs_observer_t_magic;
pub use flecsys::ecs_os_api;
pub use flecsys::ecs_os_api_calloc_count;
pub use flecsys::ecs_os_api_free_count;
pub use flecsys::ecs_os_api_malloc_count;
pub use flecsys::ecs_os_api_realloc_count;
pub use flecsys::ecs_query_t_magic;
pub use flecsys::ecs_stack_allocator_alloc_count;
pub use flecsys::ecs_stack_allocator_free_count;
pub use flecsys::ecs_stage_t_magic;
pub use flecsys::ecs_world_t_magic;
pub use flecsys::flecs_iter_cache_all;
pub use flecsys::flecs_iter_cache_columns;
pub use flecsys::flecs_iter_cache_ids;
pub use flecsys::flecs_iter_cache_ptrs;
pub use flecsys::flecs_iter_cache_sources;
pub use flecsys::flecs_iter_cache_variables;
pub use flecsys::EcsAcceleration;
pub use flecsys::EcsAcyclic;
pub use flecsys::EcsAlertCritical;
pub use flecsys::EcsAlertError;
pub use flecsys::EcsAlertInfo;
pub use flecsys::EcsAlertWarning;
pub use flecsys::EcsAlias;
pub use flecsys::EcsAmount;
pub use flecsys::EcsAmpere;
pub use flecsys::EcsAngle;
pub use flecsys::EcsAny;
pub use flecsys::EcsAperiodicComponentMonitors;
pub use flecsys::EcsAperiodicEmptyQueries;
pub use flecsys::EcsAperiodicEmptyTables;
pub use flecsys::EcsAtto;
pub use flecsys::EcsBar;
pub use flecsys::EcsBel;
pub use flecsys::EcsBits;
pub use flecsys::EcsBitsPerSecond;
pub use flecsys::EcsBytes;
pub use flecsys::EcsBytesPerSecond;
pub use flecsys::EcsCanToggle;
pub use flecsys::EcsCandela;
pub use flecsys::EcsCascade;
pub use flecsys::EcsCelsius;
pub use flecsys::EcsCenti;
pub use flecsys::EcsCentiMeters;
pub use flecsys::EcsChildOf;
pub use flecsys::EcsColor;
pub use flecsys::EcsColorCss;
pub use flecsys::EcsColorHsl;
pub use flecsys::EcsColorRgb;
pub use flecsys::EcsConstant;
pub use flecsys::EcsCounter;
pub use flecsys::EcsCounterId;
pub use flecsys::EcsCounterIncrement;
pub use flecsys::EcsData;
pub use flecsys::EcsDataRate;
pub use flecsys::EcsDate;
pub use flecsys::EcsDays;
pub use flecsys::EcsDeca;
pub use flecsys::EcsDeci;
pub use flecsys::EcsDeciBel;
pub use flecsys::EcsDegrees;
pub use flecsys::EcsDelete;
pub use flecsys::EcsDependsOn;
pub use flecsys::EcsDesc;
pub use flecsys::EcsDisabled;
pub use flecsys::EcsDocBrief;
pub use flecsys::EcsDocColor;
pub use flecsys::EcsDocDetail;
pub use flecsys::EcsDocLink;
pub use flecsys::EcsDontInherit;
pub use flecsys::EcsDuration;
pub use flecsys::EcsElectricCurrent;
pub use flecsys::EcsEmpty;
pub use flecsys::EcsEntityIsId;
pub use flecsys::EcsEntityIsTarget;
pub use flecsys::EcsEntityIsTraversable;
pub use flecsys::EcsEventNoOnSet;
pub use flecsys::EcsEventTableOnly;
pub use flecsys::EcsExa;
pub use flecsys::EcsExbi;
pub use flecsys::EcsExclusive;
pub use flecsys::EcsFahrenheit;
pub use flecsys::EcsFemto;
pub use flecsys::EcsFinal;
pub use flecsys::EcsFirstUserComponentId;
pub use flecsys::EcsFirstUserEntityId;
pub use flecsys::EcsFlecs;
pub use flecsys::EcsFlecsCore;
pub use flecsys::EcsForce;
pub use flecsys::EcsFrequency;
pub use flecsys::EcsGauge;
pub use flecsys::EcsGibi;
pub use flecsys::EcsGibiBytes;
pub use flecsys::EcsGiga;
pub use flecsys::EcsGigaBits;
pub use flecsys::EcsGigaBitsPerSecond;
pub use flecsys::EcsGigaBytes;
pub use flecsys::EcsGigaBytesPerSecond;
pub use flecsys::EcsGigaHertz;
pub use flecsys::EcsGrams;
pub use flecsys::EcsHecto;
pub use flecsys::EcsHertz;
pub use flecsys::EcsHours;
pub use flecsys::EcsIdCanToggle;
pub use flecsys::EcsIdEventMask;
pub use flecsys::EcsIdExclusive;
pub use flecsys::EcsIdHasOnAdd;
pub use flecsys::EcsIdHasOnRemove;
pub use flecsys::EcsIdHasOnSet;
pub use flecsys::EcsIdHasOnTableCreate;
pub use flecsys::EcsIdHasOnTableDelete;
pub use flecsys::EcsIdHasOnTableEmpty;
pub use flecsys::EcsIdHasOnTableFill;
pub use flecsys::EcsIdHasUnSet;
pub use flecsys::EcsIdIsSparse;
pub use flecsys::EcsIdIsUnion;
pub use flecsys::EcsIdMarkedForDelete;
pub use flecsys::EcsIdOnDeleteDelete;
pub use flecsys::EcsIdOnDeleteMask;
pub use flecsys::EcsIdOnDeleteObjectDelete;
pub use flecsys::EcsIdOnDeleteObjectMask;
pub use flecsys::EcsIdOnDeleteObjectPanic;
pub use flecsys::EcsIdOnDeleteObjectRemove;
pub use flecsys::EcsIdOnDeletePanic;
pub use flecsys::EcsIdOnDeleteRemove;
pub use flecsys::EcsIdOnInstantiateDontInherit;
pub use flecsys::EcsIdOnInstantiateInherit;
pub use flecsys::EcsIdOnInstantiateMask;
pub use flecsys::EcsIdOnInstantiateOverride;
pub use flecsys::EcsIdTag;
pub use flecsys::EcsIdTraversable;
pub use flecsys::EcsIdWith;
pub use flecsys::EcsInherit;
pub use flecsys::EcsIsA;
pub use flecsys::EcsIsEntity;
pub use flecsys::EcsIsName;
pub use flecsys::EcsIsVariable;
pub use flecsys::EcsIterCacheSearch;
pub use flecsys::EcsIterCppEach;
pub use flecsys::EcsIterFixedInChangeComputed;
pub use flecsys::EcsIterFixedInChanged;
pub use flecsys::EcsIterHasCondSet;
pub use flecsys::EcsIterIgnoreThis;
pub use flecsys::EcsIterIsInstanced;
pub use flecsys::EcsIterIsValid;
pub use flecsys::EcsIterNext;
pub use flecsys::EcsIterNextYield;
pub use flecsys::EcsIterNoData;
pub use flecsys::EcsIterNoResults;
pub use flecsys::EcsIterProfile;
pub use flecsys::EcsIterSkip;
pub use flecsys::EcsIterTableOnly;
pub use flecsys::EcsIterTrivialSearch;
pub use flecsys::EcsIterTrivialSearchNoData;
pub use flecsys::EcsIterTrivialSearchWildcard;
pub use flecsys::EcsIterTrivialTest;
pub use flecsys::EcsIterTrivialTestWildcard;
pub use flecsys::EcsIterYield;
pub use flecsys::EcsKelvin;
pub use flecsys::EcsKibi;
pub use flecsys::EcsKibiBytes;
pub use flecsys::EcsKilo;
pub use flecsys::EcsKiloBits;
pub use flecsys::EcsKiloBitsPerSecond;
pub use flecsys::EcsKiloBytes;
pub use flecsys::EcsKiloBytesPerSecond;
pub use flecsys::EcsKiloGrams;
pub use flecsys::EcsKiloHertz;
pub use flecsys::EcsKiloMeters;
pub use flecsys::EcsKiloMetersPerHour;
pub use flecsys::EcsKiloMetersPerSecond;
pub use flecsys::EcsLength;
pub use flecsys::EcsLuminousIntensity;
pub use flecsys::EcsMass;
pub use flecsys::EcsMebi;
pub use flecsys::EcsMebiBytes;
pub use flecsys::EcsMega;
pub use flecsys::EcsMegaBits;
pub use flecsys::EcsMegaBitsPerSecond;
pub use flecsys::EcsMegaBytes;
pub use flecsys::EcsMegaBytesPerSecond;
pub use flecsys::EcsMegaHertz;
pub use flecsys::EcsMeters;
pub use flecsys::EcsMetersPerSecond;
pub use flecsys::EcsMetric;
pub use flecsys::EcsMetricInstance;
pub use flecsys::EcsMicro;
pub use flecsys::EcsMicroMeters;
pub use flecsys::EcsMicroSeconds;
pub use flecsys::EcsMiles;
pub use flecsys::EcsMilesPerHour;
pub use flecsys::EcsMilli;
pub use flecsys::EcsMilliMeters;
pub use flecsys::EcsMilliSeconds;
pub use flecsys::EcsMinutes;
pub use flecsys::EcsModule;
pub use flecsys::EcsMole;
pub use flecsys::EcsMonitor;
pub use flecsys::EcsName;
pub use flecsys::EcsNano;
pub use flecsys::EcsNanoMeters;
pub use flecsys::EcsNanoSeconds;
pub use flecsys::EcsNewton;
pub use flecsys::EcsNotQueryable;
pub use flecsys::EcsObserver;
pub use flecsys::EcsObserverIsDisabled;
pub use flecsys::EcsObserverIsMonitor;
pub use flecsys::EcsObserverIsMulti;
pub use flecsys::EcsObserverIsParentDisabled;
pub use flecsys::EcsOnAdd;
pub use flecsys::EcsOnDelete;
pub use flecsys::EcsOnDeleteTarget;
pub use flecsys::EcsOnInstantiate;
pub use flecsys::EcsOnLoad;
pub use flecsys::EcsOnRemove;
pub use flecsys::EcsOnSet;
pub use flecsys::EcsOnStart;
pub use flecsys::EcsOnStore;
pub use flecsys::EcsOnTableCreate;
pub use flecsys::EcsOnTableDelete;
pub use flecsys::EcsOnTableEmpty;
pub use flecsys::EcsOnTableFill;
pub use flecsys::EcsOnUpdate;
pub use flecsys::EcsOnValidate;
pub use flecsys::EcsOneOf;
pub use flecsys::EcsOsApiHighResolutionTimer;
pub use flecsys::EcsOsApiLogWithColors;
pub use flecsys::EcsOsApiLogWithTimeDelta;
pub use flecsys::EcsOsApiLogWithTimeStamp;
pub use flecsys::EcsOverride;
pub use flecsys::EcsPairIsTag;
pub use flecsys::EcsPanic;
pub use flecsys::EcsPascal;
pub use flecsys::EcsPebi;
pub use flecsys::EcsPercentage;
pub use flecsys::EcsPeriod1d;
pub use flecsys::EcsPeriod1h;
pub use flecsys::EcsPeriod1m;
pub use flecsys::EcsPeriod1s;
pub use flecsys::EcsPeriod1w;
pub use flecsys::EcsPeta;
pub use flecsys::EcsPhase;
pub use flecsys::EcsPico;
pub use flecsys::EcsPicoMeters;
pub use flecsys::EcsPicoSeconds;
pub use flecsys::EcsPixels;
pub use flecsys::EcsPostFrame;
pub use flecsys::EcsPostLoad;
pub use flecsys::EcsPostUpdate;
pub use flecsys::EcsPreFrame;
pub use flecsys::EcsPreStore;
pub use flecsys::EcsPreUpdate;
pub use flecsys::EcsPredEq;
pub use flecsys::EcsPredLookup;
pub use flecsys::EcsPredMatch;
pub use flecsys::EcsPrefab;
pub use flecsys::EcsPressure;
pub use flecsys::EcsPrivate;
pub use flecsys::EcsQuantity;
pub use flecsys::EcsQuery;
pub use flecsys::EcsQueryAllowUnresolvedByName;
pub use flecsys::EcsQueryHasCacheable;
pub use flecsys::EcsQueryHasCondSet;
pub use flecsys::EcsQueryHasMonitor;
pub use flecsys::EcsQueryHasNonThisOutTerms;
pub use flecsys::EcsQueryHasOutTerms;
pub use flecsys::EcsQueryHasPred;
pub use flecsys::EcsQueryHasRefs;
pub use flecsys::EcsQueryHasScopes;
pub use flecsys::EcsQueryHasSparseThis;
pub use flecsys::EcsQueryHasTableThisVar;
pub use flecsys::EcsQueryIsCacheable;
pub use flecsys::EcsQueryIsInstanced;
pub use flecsys::EcsQueryIsTrivial;
pub use flecsys::EcsQueryMatchDisabled;
pub use flecsys::EcsQueryMatchEmptyTables;
pub use flecsys::EcsQueryMatchOnlySelf;
pub use flecsys::EcsQueryMatchOnlyThis;
pub use flecsys::EcsQueryMatchPrefab;
pub use flecsys::EcsQueryMatchThis;
pub use flecsys::EcsQueryMatchWildcards;
pub use flecsys::EcsQueryNoData;
pub use flecsys::EcsQueryTableOnly;
pub use flecsys::EcsRadians;
pub use flecsys::EcsReflexive;
pub use flecsys::EcsRelationship;
pub use flecsys::EcsRemove;
pub use flecsys::EcsScopeClose;
pub use flecsys::EcsScopeOpen;
pub use flecsys::EcsSeconds;
pub use flecsys::EcsSelf;
pub use flecsys::EcsSlotOf;
pub use flecsys::EcsSparse;
pub use flecsys::EcsSpeed;
pub use flecsys::EcsSymbol;
pub use flecsys::EcsSymmetric;
pub use flecsys::EcsSystem;
pub use flecsys::EcsTableHasAddActions;
pub use flecsys::EcsTableHasBuiltins;
pub use flecsys::EcsTableHasChildOf;
pub use flecsys::EcsTableHasCopy;
pub use flecsys::EcsTableHasCtors;
pub use flecsys::EcsTableHasDtors;
pub use flecsys::EcsTableHasIsA;
pub use flecsys::EcsTableHasLifecycle;
pub use flecsys::EcsTableHasModule;
pub use flecsys::EcsTableHasMove;
pub use flecsys::EcsTableHasName;
pub use flecsys::EcsTableHasOnAdd;
pub use flecsys::EcsTableHasOnRemove;
pub use flecsys::EcsTableHasOnSet;
pub use flecsys::EcsTableHasOnTableCreate;
pub use flecsys::EcsTableHasOnTableDelete;
pub use flecsys::EcsTableHasOnTableEmpty;
pub use flecsys::EcsTableHasOnTableFill;
pub use flecsys::EcsTableHasOverrides;
pub use flecsys::EcsTableHasPairs;
pub use flecsys::EcsTableHasRemoveActions;
pub use flecsys::EcsTableHasSparse;
pub use flecsys::EcsTableHasToggle;
pub use flecsys::EcsTableHasTraversable;
pub use flecsys::EcsTableHasUnSet;
pub use flecsys::EcsTableHasUnion;
pub use flecsys::EcsTableIsComplex;
pub use flecsys::EcsTableIsDisabled;
pub use flecsys::EcsTableIsPrefab;
pub use flecsys::EcsTableMarkedForDelete;
pub use flecsys::EcsTableNotQueryable;
pub use flecsys::EcsTarget;
pub use flecsys::EcsTebi;
pub use flecsys::EcsTemperature;
pub use flecsys::EcsTera;
pub use flecsys::EcsTermIdInherited;
pub use flecsys::EcsTermIsCacheable;
pub use flecsys::EcsTermIsMember;
pub use flecsys::EcsTermIsOr;
pub use flecsys::EcsTermIsScope;
pub use flecsys::EcsTermIsSparse;
pub use flecsys::EcsTermIsToggle;
pub use flecsys::EcsTermIsTrivial;
pub use flecsys::EcsTermIsUnion;
pub use flecsys::EcsTermKeepAlive;
pub use flecsys::EcsTermMatchAny;
pub use flecsys::EcsTermMatchAnySrc;
pub use flecsys::EcsTermNoData;
pub use flecsys::EcsTermRefFlags;
pub use flecsys::EcsTermReflexive;
pub use flecsys::EcsTermTransitive;
pub use flecsys::EcsThis;
pub use flecsys::EcsTime;
pub use flecsys::EcsTrait;
pub use flecsys::EcsTransitive;
pub use flecsys::EcsTrav;
pub use flecsys::EcsTraversable;
pub use flecsys::EcsTraverseFlags;
pub use flecsys::EcsUnSet;
pub use flecsys::EcsUnion;
pub use flecsys::EcsUnitPrefixes;
pub use flecsys::EcsUp;
pub use flecsys::EcsUri;
pub use flecsys::EcsUriFile;
pub use flecsys::EcsUriHyperlink;
pub use flecsys::EcsUriImage;
pub use flecsys::EcsVariable;
pub use flecsys::EcsWildcard;
pub use flecsys::EcsWith;
pub use flecsys::EcsWorld;
pub use flecsys::EcsWorldFini;
pub use flecsys::EcsWorldInit;
pub use flecsys::EcsWorldMeasureFrameTime;
pub use flecsys::EcsWorldMeasureSystemTime;
pub use flecsys::EcsWorldMultiThreaded;
pub use flecsys::EcsWorldQuit;
pub use flecsys::EcsWorldQuitWorkers;
pub use flecsys::EcsWorldReadonly;
pub use flecsys::EcsYobi;
pub use flecsys::EcsYocto;
pub use flecsys::EcsYotta;
pub use flecsys::EcsZebi;
pub use flecsys::EcsZepto;
pub use flecsys::EcsZetta;
pub use flecsys::FLECS_IDEcsAccelerationID_;
pub use flecsys::FLECS_IDEcsAlertCriticalID_;
pub use flecsys::FLECS_IDEcsAlertErrorID_;
pub use flecsys::FLECS_IDEcsAlertID_;
pub use flecsys::FLECS_IDEcsAlertInfoID_;
pub use flecsys::FLECS_IDEcsAlertInstanceID_;
pub use flecsys::FLECS_IDEcsAlertTimeoutID_;
pub use flecsys::FLECS_IDEcsAlertWarningID_;
pub use flecsys::FLECS_IDEcsAlertsActiveID_;
pub use flecsys::FLECS_IDEcsAmountID_;
pub use flecsys::FLECS_IDEcsAmpereID_;
pub use flecsys::FLECS_IDEcsAngleID_;
pub use flecsys::FLECS_IDEcsArrayID_;
pub use flecsys::FLECS_IDEcsAttoID_;
pub use flecsys::FLECS_IDEcsBarID_;
pub use flecsys::FLECS_IDEcsBelID_;
pub use flecsys::FLECS_IDEcsBitmaskID_;
pub use flecsys::FLECS_IDEcsBitsID_;
pub use flecsys::FLECS_IDEcsBitsPerSecondID_;
pub use flecsys::FLECS_IDEcsBytesID_;
pub use flecsys::FLECS_IDEcsBytesPerSecondID_;
pub use flecsys::FLECS_IDEcsCandelaID_;
pub use flecsys::FLECS_IDEcsCelsiusID_;
pub use flecsys::FLECS_IDEcsCentiID_;
pub use flecsys::FLECS_IDEcsCentiMetersID_;
pub use flecsys::FLECS_IDEcsColorCssID_;
pub use flecsys::FLECS_IDEcsColorHslID_;
pub use flecsys::FLECS_IDEcsColorID_;
pub use flecsys::FLECS_IDEcsColorRgbID_;
pub use flecsys::FLECS_IDEcsComponentID_;
pub use flecsys::FLECS_IDEcsCounterID_;
pub use flecsys::FLECS_IDEcsCounterIdID_;
pub use flecsys::FLECS_IDEcsCounterIncrementID_;
pub use flecsys::FLECS_IDEcsDataID_;
pub use flecsys::FLECS_IDEcsDataRateID_;
pub use flecsys::FLECS_IDEcsDateID_;
pub use flecsys::FLECS_IDEcsDaysID_;
pub use flecsys::FLECS_IDEcsDecaID_;
pub use flecsys::FLECS_IDEcsDeciBelID_;
pub use flecsys::FLECS_IDEcsDeciID_;
pub use flecsys::FLECS_IDEcsDefaultChildComponentID_;
pub use flecsys::FLECS_IDEcsDegreesID_;
pub use flecsys::FLECS_IDEcsDocDescriptionID_;
pub use flecsys::FLECS_IDEcsDurationID_;
pub use flecsys::FLECS_IDEcsElectricCurrentID_;
pub use flecsys::FLECS_IDEcsEnumID_;
pub use flecsys::FLECS_IDEcsExaID_;
pub use flecsys::FLECS_IDEcsExbiID_;
pub use flecsys::FLECS_IDEcsFahrenheitID_;
pub use flecsys::FLECS_IDEcsFemtoID_;
pub use flecsys::FLECS_IDEcsForceID_;
pub use flecsys::FLECS_IDEcsFrequencyID_;
pub use flecsys::FLECS_IDEcsGaugeID_;
pub use flecsys::FLECS_IDEcsGibiBytesID_;
pub use flecsys::FLECS_IDEcsGibiID_;
pub use flecsys::FLECS_IDEcsGigaBitsID_;
pub use flecsys::FLECS_IDEcsGigaBitsPerSecondID_;
pub use flecsys::FLECS_IDEcsGigaBytesID_;
pub use flecsys::FLECS_IDEcsGigaBytesPerSecondID_;
pub use flecsys::FLECS_IDEcsGigaHertzID_;
pub use flecsys::FLECS_IDEcsGigaID_;
pub use flecsys::FLECS_IDEcsGramsID_;
pub use flecsys::FLECS_IDEcsHectoID_;
pub use flecsys::FLECS_IDEcsHertzID_;
pub use flecsys::FLECS_IDEcsHoursID_;
pub use flecsys::FLECS_IDEcsIdentifierID_;
pub use flecsys::FLECS_IDEcsKelvinID_;
pub use flecsys::FLECS_IDEcsKibiBytesID_;
pub use flecsys::FLECS_IDEcsKibiID_;
pub use flecsys::FLECS_IDEcsKiloBitsID_;
pub use flecsys::FLECS_IDEcsKiloBitsPerSecondID_;
pub use flecsys::FLECS_IDEcsKiloBytesID_;
pub use flecsys::FLECS_IDEcsKiloBytesPerSecondID_;
pub use flecsys::FLECS_IDEcsKiloGramsID_;
pub use flecsys::FLECS_IDEcsKiloHertzID_;
pub use flecsys::FLECS_IDEcsKiloID_;
pub use flecsys::FLECS_IDEcsKiloMetersID_;
pub use flecsys::FLECS_IDEcsKiloMetersPerHourID_;
pub use flecsys::FLECS_IDEcsKiloMetersPerSecondID_;
pub use flecsys::FLECS_IDEcsLengthID_;
pub use flecsys::FLECS_IDEcsLuminousIntensityID_;
pub use flecsys::FLECS_IDEcsMassID_;
pub use flecsys::FLECS_IDEcsMebiBytesID_;
pub use flecsys::FLECS_IDEcsMebiID_;
pub use flecsys::FLECS_IDEcsMegaBitsID_;
pub use flecsys::FLECS_IDEcsMegaBitsPerSecondID_;
pub use flecsys::FLECS_IDEcsMegaBytesID_;
pub use flecsys::FLECS_IDEcsMegaBytesPerSecondID_;
pub use flecsys::FLECS_IDEcsMegaHertzID_;
pub use flecsys::FLECS_IDEcsMegaID_;
pub use flecsys::FLECS_IDEcsMemberID_;
pub use flecsys::FLECS_IDEcsMemberRangesID_;
pub use flecsys::FLECS_IDEcsMetersID_;
pub use flecsys::FLECS_IDEcsMetersPerSecondID_;
pub use flecsys::FLECS_IDEcsMetricID_;
pub use flecsys::FLECS_IDEcsMetricInstanceID_;
pub use flecsys::FLECS_IDEcsMetricSourceID_;
pub use flecsys::FLECS_IDEcsMetricValueID_;
pub use flecsys::FLECS_IDEcsMicroID_;
pub use flecsys::FLECS_IDEcsMicroMetersID_;
pub use flecsys::FLECS_IDEcsMicroSecondsID_;
pub use flecsys::FLECS_IDEcsMilesID_;
pub use flecsys::FLECS_IDEcsMilesPerHourID_;
pub use flecsys::FLECS_IDEcsMilliID_;
pub use flecsys::FLECS_IDEcsMilliMetersID_;
pub use flecsys::FLECS_IDEcsMilliSecondsID_;
pub use flecsys::FLECS_IDEcsMinutesID_;
pub use flecsys::FLECS_IDEcsMoleID_;
pub use flecsys::FLECS_IDEcsNanoID_;
pub use flecsys::FLECS_IDEcsNanoMetersID_;
pub use flecsys::FLECS_IDEcsNanoSecondsID_;
pub use flecsys::FLECS_IDEcsNewtonID_;
pub use flecsys::FLECS_IDEcsOpaqueID_;
pub use flecsys::FLECS_IDEcsPascalID_;
pub use flecsys::FLECS_IDEcsPebiID_;
pub use flecsys::FLECS_IDEcsPercentageID_;
pub use flecsys::FLECS_IDEcsPetaID_;
pub use flecsys::FLECS_IDEcsPicoID_;
pub use flecsys::FLECS_IDEcsPicoMetersID_;
pub use flecsys::FLECS_IDEcsPicoSecondsID_;
pub use flecsys::FLECS_IDEcsPipelineID_;
pub use flecsys::FLECS_IDEcsPipelineQueryID_;
pub use flecsys::FLECS_IDEcsPipelineStatsID_;
pub use flecsys::FLECS_IDEcsPixelsID_;
pub use flecsys::FLECS_IDEcsPolyID_;
pub use flecsys::FLECS_IDEcsPressureID_;
pub use flecsys::FLECS_IDEcsPrimitiveID_;
pub use flecsys::FLECS_IDEcsRadiansID_;
pub use flecsys::FLECS_IDEcsRateFilterID_;
pub use flecsys::FLECS_IDEcsRestID_;
pub use flecsys::FLECS_IDEcsScriptID_;
pub use flecsys::FLECS_IDEcsSecondsID_;
pub use flecsys::FLECS_IDEcsSpeedID_;
pub use flecsys::FLECS_IDEcsStructID_;
pub use flecsys::FLECS_IDEcsSystemStatsID_;
pub use flecsys::FLECS_IDEcsTebiID_;
pub use flecsys::FLECS_IDEcsTemperatureID_;
pub use flecsys::FLECS_IDEcsTeraID_;
pub use flecsys::FLECS_IDEcsTickSourceID_;
pub use flecsys::FLECS_IDEcsTimeID_;
pub use flecsys::FLECS_IDEcsTimerID_;
pub use flecsys::FLECS_IDEcsTypeID_;
pub use flecsys::FLECS_IDEcsTypeSerializerID_;
pub use flecsys::FLECS_IDEcsUnitID_;
pub use flecsys::FLECS_IDEcsUnitPrefixID_;
pub use flecsys::FLECS_IDEcsUnitPrefixesID_;
pub use flecsys::FLECS_IDEcsUriFileID_;
pub use flecsys::FLECS_IDEcsUriHyperlinkID_;
pub use flecsys::FLECS_IDEcsUriID_;
pub use flecsys::FLECS_IDEcsUriImageID_;
pub use flecsys::FLECS_IDEcsVectorID_;
pub use flecsys::FLECS_IDEcsWorldStatsID_;
pub use flecsys::FLECS_IDEcsWorldSummaryID_;
pub use flecsys::FLECS_IDEcsYobiID_;
pub use flecsys::FLECS_IDEcsYoctoID_;
pub use flecsys::FLECS_IDEcsYottaID_;
pub use flecsys::FLECS_IDEcsZebiID_;
pub use flecsys::FLECS_IDEcsZeptoID_;
pub use flecsys::FLECS_IDEcsZettaID_;
pub use flecsys::FLECS_IDFlecsAlertsID_;
pub use flecsys::FLECS_IDFlecsMetricsID_;
pub use flecsys::FLECS_IDFlecsStatsID_;
pub use flecsys::FLECS_IDecs_bool_tID_;
pub use flecsys::FLECS_IDecs_byte_tID_;
pub use flecsys::FLECS_IDecs_char_tID_;
pub use flecsys::FLECS_IDecs_entity_tID_;
pub use flecsys::FLECS_IDecs_f32_tID_;
pub use flecsys::FLECS_IDecs_f64_tID_;
pub use flecsys::FLECS_IDecs_i16_tID_;
pub use flecsys::FLECS_IDecs_i32_tID_;
pub use flecsys::FLECS_IDecs_i64_tID_;
pub use flecsys::FLECS_IDecs_i8_tID_;
pub use flecsys::FLECS_IDecs_id_tID_;
pub use flecsys::FLECS_IDecs_iptr_tID_;
pub use flecsys::FLECS_IDecs_string_tID_;
pub use flecsys::FLECS_IDecs_u16_tID_;
pub use flecsys::FLECS_IDecs_u32_tID_;
pub use flecsys::FLECS_IDecs_u64_tID_;
pub use flecsys::FLECS_IDecs_u8_tID_;
pub use flecsys::FLECS_IDecs_uptr_tID_;
pub use flecsys::ECS_ACCESS_VIOLATION;
pub use flecsys::ECS_ALERT_MAX_SEVERITY_FILTERS;
pub use flecsys::ECS_ALREADY_DEFINED;
pub use flecsys::ECS_ALREADY_IN_USE;
pub use flecsys::ECS_AUTO_OVERRIDE;
pub use flecsys::ECS_BLACK;
pub use flecsys::ECS_BLUE;
pub use flecsys::ECS_BOLD;
pub use flecsys::ECS_COLUMN_INDEX_OUT_OF_RANGE;
pub use flecsys::ECS_COLUMN_IS_NOT_SHARED;
pub use flecsys::ECS_COLUMN_IS_SHARED;
pub use flecsys::ECS_COLUMN_TYPE_MISMATCH;
pub use flecsys::ECS_COMPONENT_MASK;
pub use flecsys::ECS_COMPONENT_NOT_REGISTERED;
pub use flecsys::ECS_CONSTRAINT_VIOLATED;
pub use flecsys::ECS_CYAN;
pub use flecsys::ECS_CYCLE_DETECTED;
pub use flecsys::ECS_DOUBLE_FREE;
pub use flecsys::ECS_ENTITY_MASK;
pub use flecsys::ECS_GENERATION_MASK;
pub use flecsys::ECS_GREEN;
pub use flecsys::ECS_GREY;
pub use flecsys::ECS_HTTP_HEADER_COUNT_MAX;
pub use flecsys::ECS_HTTP_QUERY_PARAM_COUNT_MAX;
pub use flecsys::ECS_ID_FLAGS_MASK;
pub use flecsys::ECS_ID_IN_USE;
pub use flecsys::ECS_INCONSISTENT_COMPONENT_ACTION;
pub use flecsys::ECS_INCONSISTENT_COMPONENT_ID;
pub use flecsys::ECS_INCONSISTENT_NAME;
pub use flecsys::ECS_INTERNAL_ERROR;
pub use flecsys::ECS_INVALID_COMPONENT_ALIGNMENT;
pub use flecsys::ECS_INVALID_COMPONENT_SIZE;
pub use flecsys::ECS_INVALID_CONVERSION;
pub use flecsys::ECS_INVALID_FROM_WORKER;
pub use flecsys::ECS_INVALID_OPERATION;
pub use flecsys::ECS_INVALID_PARAMETER;
pub use flecsys::ECS_INVALID_WHILE_READONLY;
pub use flecsys::ECS_LEAK_DETECTED;
pub use flecsys::ECS_LOCKED_STORAGE;
pub use flecsys::ECS_MAGENTA;
pub use flecsys::ECS_MAX_RECURSION;
pub use flecsys::ECS_MAX_TOKEN_SIZE;
pub use flecsys::ECS_MEMBER_DESC_CACHE_SIZE;
pub use flecsys::ECS_META_MAX_SCOPE_DEPTH;
pub use flecsys::ECS_MISSING_OS_API;
pub use flecsys::ECS_MISSING_SYMBOL;
pub use flecsys::ECS_MODULE_UNDEFINED;
pub use flecsys::ECS_NAME_IN_USE;
pub use flecsys::ECS_NORMAL;
pub use flecsys::ECS_NOT_A_COMPONENT;
pub use flecsys::ECS_OPERATION_FAILED;
pub use flecsys::ECS_OUT_OF_MEMORY;
pub use flecsys::ECS_OUT_OF_RANGE;
pub use flecsys::ECS_PAIR;
pub use flecsys::ECS_RED;
pub use flecsys::ECS_REST_DEFAULT_PORT;
pub use flecsys::ECS_ROW_FLAGS_MASK;
pub use flecsys::ECS_ROW_MASK;
pub use flecsys::ECS_STACK_PAGE_SIZE;
pub use flecsys::ECS_STAT_WINDOW;
pub use flecsys::ECS_STRBUF_MAX_LIST_DEPTH;
pub use flecsys::ECS_STRBUF_SMALL_STRING_SIZE;
pub use flecsys::ECS_TOGGLE;
pub use flecsys::ECS_UNSUPPORTED;
pub use flecsys::ECS_WHITE;
pub use flecsys::ECS_YELLOW;
pub use flecsys::FLECS_ENTITY_PAGE_BITS;
pub use flecsys::FLECS_EVENT_DESC_MAX;
pub use flecsys::FLECS_HI_COMPONENT_ID;
pub use flecsys::FLECS_HI_ID_RECORD_ID;
pub use flecsys::FLECS_ID_DESC_MAX;
pub use flecsys::FLECS_QUERY_SCOPE_NESTING_MAX;
pub use flecsys::FLECS_QUERY_VARIABLE_COUNT_MAX;
pub use flecsys::FLECS_SPARSE_PAGE_BITS;
pub use flecsys::FLECS_SPARSE_PAGE_SIZE;
pub use flecsys::FLECS_TERM_ARG_COUNT_MAX;
pub use flecsys::FLECS_TERM_COUNT_MAX;
pub use flecsys::FLECS_VARIABLE_COUNT_MAX;
pub use flecsys::FLECS_VERSION_MAJOR;
pub use flecsys::FLECS_VERSION_MINOR;
pub use flecsys::FLECS_VERSION_PATCH;
pub struct HttpServerMut {
    ptr: *mut flecsys::ecs_http_server_t,
}
impl HttpServerMut {
    /** Destroy server.
    This operation will stop the server if it was still running.

    @param server The server to destroy.*/
    pub fn http_server_fini(&mut self) {
        let server = self.ptr;
        let result = unsafe { flecsys::ecs_http_server_fini(server) };
    }
    /** Start server.
    After this operation the server will be able to accept requests.

    @param server The server to start.
    @return Zero if successful, non-zero if failed.*/
    pub fn http_server_start(&mut self) -> i32 {
        let server = self.ptr;
        let result = unsafe { flecsys::ecs_http_server_start(server) };
        return result;
    }
    /** Process server requests.
    This operation invokes the reply callback for each received request. No new
    requests will be enqueued while processing requests.

    @param server The server for which to process requests.*/
    pub fn http_server_dequeue(&mut self, delta_time: f32) {
        let server = self.ptr;
        let result = unsafe { flecsys::ecs_http_server_dequeue(server, delta_time) };
    }
    /** Stop server.
    After this operation no new requests can be received.

    @param server The server.*/
    pub fn http_server_stop(&mut self) {
        let server = self.ptr;
        let result = unsafe { flecsys::ecs_http_server_stop(server) };
    }
    /** Cleanup REST HTTP server.
    The server must have been created with ecs_rest_server_init().*/
    pub fn rest_server_fini(&mut self) {
        let srv = self.ptr;
        let result = unsafe { flecsys::ecs_rest_server_fini(srv) };
    }
}
pub struct TableRef {
    ptr: *mut flecsys::ecs_table_t,
}
impl TableRef {
    /** Return number of columns in table.
    Similar to ecs_table_get_type(table)->count, except that the column count
    only counts the number of components in a table.

    @param table The table.
    @return The number of columns in the table.*/
    pub fn table_column_count(&self) -> i32 {
        let table = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_column_count(table) };
        return result;
    }
    /** Convert type index to column index.
    Tables have an array of columns for each component in the table. This array
    does not include elements for tags, which means that the index for a
    component in the table type is not necessarily the same as the index in the
    column array. This operation converts from an index in the table type to an
    index in the column array.

    @param table The table.
    @param index The index in the table type.
    @return The index in the table column array.*/
    pub fn table_type_to_column_index(&self, index: i32) -> i32 {
        let table = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_type_to_column_index(table, index) };
        return result;
    }
    /** Convert column index to type index.
    Same as ecs_table_type_to_column_index(), but converts from an index in the
    column array to an index in the table type.

    @param table The table.
    @param index The column index.
    @return The index in the table type.*/
    pub fn table_column_to_type_index(&self, index: i32) -> i32 {
        let table = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_column_to_type_index(table, index) };
        return result;
    }
    /** Get column size from table.
    This operation returns the component size for the provided index.

    @param table The table.
    @param index The column index.
    @return The component size, or 0 if the index is not a component.*/
    pub fn table_get_column_size(&self, index: i32) -> usize {
        let table = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_get_column_size(table, index) };
        return result;
    }
    /** Returns the number of records in the table.
    This operation returns the number of records that have been populated through
    the regular (entity) API as well as the number of records that have been
    inserted using the direct access API.

    @param table The table.
    @return The number of records in a table.*/
    pub fn table_count(&self) -> i32 {
        let table = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_count(table) };
        return result;
    }
}
pub struct TableMut {
    ptr: *mut flecsys::ecs_table_t,
}
impl TableMut {
    /** Test table for flags.
    Test if table has all of the provided flags. See
    include/flecs/private/api_flags.h for a list of table flags that can be used
    with this function.

    @param table The table.
    @param flags The flags to test for.
    @return Whether the specified flags are set for the table.*/
    pub fn table_has_flags(&mut self, flags: u32) -> bool {
        let table = self.ptr;
        let result = unsafe { flecsys::ecs_table_has_flags(table, flags) };
        return result;
    }
}
pub struct WorldRef {
    ptr: *mut flecsys::ecs_world_t,
}
impl WorldRef {
    /** Returns whether the world is being deleted.
    This operation can be used in callbacks like type hooks or observers to
    detect if they are invoked while the world is being deleted.

    @param world The world.
    @return True if being deleted, false if not.*/
    pub fn is_fini(&self) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_is_fini(world) };
        return result;
    }
    /** Return whether a quit has been requested.

    @param world The world.
    @return Whether a quit has been requested.
    @see ecs_quit()*/
    pub fn should_quit(&self) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_should_quit(world) };
        return result;
    }
    /** Test if deferring is enabled for current stage.

    @param world The world.
    @return True if deferred, false if not.*/
    pub fn is_deferred(&self) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_is_deferred(world) };
        return result;
    }
    /** Get number of configured stages.
    Return number of stages set by ecs_set_stage_count().

    @param world The world.
    @return The number of stages used for threading.*/
    pub fn get_stage_count(&self) -> i32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_stage_count(world) };
        return result;
    }
    /** Test whether the current world is readonly.
    This function allows the code to test whether the currently used world
    is readonly or whether it allows for writing.

    @param world A pointer to a stage or the world.
    @return True if the world or stage is readonly.*/
    pub fn stage_is_readonly(&self) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_stage_is_readonly(world) };
        return result;
    }
    /** Get stage id.
    The stage id can be used by an application to learn about which stage it is
    using, which typically corresponds with the worker thread id.

    @param world The world.
    @return The stage id.*/
    pub fn stage_get_id(&self) -> i32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_stage_get_id(world) };
        return result;
    }
    /** Get the largest issued entity id (not counting generation).

    @param world The world.
    @return The largest issued entity id.*/
    pub fn get_max_id(&self) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_max_id(world) };
        return result;
    }
    /** Get current with id.
    Get the id set with ecs_set_with().

    @param world The world.
    @return The last id provided to ecs_set_with().*/
    pub fn get_with(&self) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_with(world) };
        return result;
    }
    /** Test if component is enabled.
    Test whether a component is currently enabled or disabled. This operation
    will return true when the entity has the component and if it has not been
    disabled by ecs_enable_component().

    @param world The world.
    @param entity The entity.
    @param id The component.
    @return True if the component is enabled, otherwise false.*/
    pub fn is_enabled_id(&self, entity: u64, id: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_is_enabled_id(world, entity, id) };
        return result;
    }
    /** Test whether an entity is valid.
    Entities that are valid can be used with API functions. Using invalid
    entities with API operations will cause the function to panic.

    An entity is valid if it is not 0 and if it is alive.

    ecs_is_valid() will return true for ids that don't exist (alive or not alive). This
    allows for using ids that have never been created by ecs_new_w() or similar. In
    this the function differs from ecs_is_alive(), which will return false for
    entities that do not yet exist.

    The operation will return false for an id that exists and is not alive, as
    using this id with an API operation would cause it to assert.

    @param world The world.
    @param e The entity.
    @return True if the entity is valid, false if the entity is not valid.*/
    pub fn is_valid(&self, e: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_is_valid(world, e) };
        return result;
    }
    /** Test whether an entity is alive.
    Entities are alive after they are created, and become not alive when they are
    deleted. Operations that return alive ids are (amongst others) ecs_new(),
    ecs_new_low_id() and ecs_entity_init(). Ids can be made alive with the ecs_make_alive()
    function.

    After an id is deleted it can be recycled. Recycled ids are different from
    the original id in that they have a different generation count. This makes it
    possible for the API to distinguish between the two. An example:

    @code
    ecs_entity_t e1 = ecs_new(world);
    ecs_is_alive(world, e1);             // true
    ecs_delete(world, e1);
    ecs_is_alive(world, e1);             // false

    ecs_entity_t e2 = ecs_new(world); // recycles e1
    ecs_is_alive(world, e2);             // true
    ecs_is_alive(world, e1);             // false
    @endcode

    @param world The world.
    @param e The entity.
    @return True if the entity is alive, false if the entity is not alive.*/
    pub fn is_alive(&self, e: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_is_alive(world, e) };
        return result;
    }
    /** Get alive identifier.
    In some cases an application may need to work with identifiers from which
    the generation has been stripped. A typical scenario in which this happens is
    when iterating relationships in an entity type.

    For example, when obtaining the parent id from a ChildOf relationship, the parent
    (second element of the pair) will have been stored in a 32 bit value, which
    cannot store the entity generation. This function can retrieve the identifier
    with the current generation for that id.

    If the provided identifier is not alive, the function will return 0.

    @param world The world.
    @param e The for which to obtain the current alive entity id.
    @return The alive entity id if there is one, or 0 if the id is not alive.*/
    pub fn get_alive(&self, e: u64) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_alive(world, e) };
        return result;
    }
    /** Test whether an entity exists.
    Similar as ecs_is_alive(), but ignores entity generation count.

    @param world The world.
    @param entity The entity.
    @return True if the entity exists, false if the entity does not exist.*/
    pub fn exists(&self, entity: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_exists(world, entity) };
        return result;
    }
    /** Test if an entity has an id.
    This operation returns true if the entity has or inherits the specified id.

    @param world The world.
    @param entity The entity.
    @param id The id to test for.
    @return True if the entity has the id, false if not.*/
    pub fn has_id(&self, entity: u64, id: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_has_id(world, entity, id) };
        return result;
    }
    /** Test if an entity owns an id.
    This operation returns true if the entity has the specified id. The operation
    behaves the same as ecs_has_id(), except that it will return false for
    components that are inherited through an IsA relationship.

    @param world The world.
    @param entity The entity.
    @param id The id to test for.
    @return True if the entity has the id, false if not.*/
    pub fn owns_id(&self, entity: u64, id: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_owns_id(world, entity, id) };
        return result;
    }
    /** Get the target of a relationship.
    This will return a target (second element of a pair) of the entity for the
    specified relationship. The index allows for iterating through the targets,
    if a single entity has multiple targets for the same relationship.

    If the index is larger than the total number of instances the entity has for
    the relationship, the operation will return 0.

    @param world The world.
    @param entity The entity.
    @param rel The relationship between the entity and the target.
    @param index The index of the relationship instance.
    @return The target for the relationship at the specified index.*/
    pub fn get_target(&self, entity: u64, rel: u64, index: i32) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_target(world, entity, rel, index) };
        return result;
    }
    /** Get parent (target of ChildOf relationship) for entity.
    This operation is the same as calling:

    @code
    ecs_get_target(world, entity, EcsChildOf, 0);
    @endcode

    @param world The world.
    @param entity The entity.
    @return The parent of the entity, 0 if the entity has no parent.*/
    pub fn get_parent(&self, entity: u64) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_parent(world, entity) };
        return result;
    }
    /** Get the target of a relationship for a given id.
    This operation returns the first entity that has the provided id by following
    the specified relationship. If the entity itself has the id then entity will
    be returned. If the id cannot be found on the entity or by following the
    relationship, the operation will return 0.

    This operation can be used to lookup, for example, which prefab is providing
    a component by specifying the IsA relationship:

    @code
    // Is Position provided by the entity or one of its base entities?
    ecs_get_target_for_id(world, entity, EcsIsA, ecs_id(Position))
    @endcode

    @param world The world.
    @param entity The entity.
    @param rel The relationship to follow.
    @param id The id to lookup.
    @return The entity for which the target has been found.*/
    pub fn get_target_for_id(&self, entity: u64, rel: u64, id: u64) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_target_for_id(world, entity, rel, id) };
        return result;
    }
    /** Return depth for entity in tree for the specified relationship.
    Depth is determined by counting the number of targets encountered while
    traversing up the relationship tree for rel. Only acyclic relationships are
    supported.

    @param world The world.
    @param entity The entity.
    @param rel The relationship.
    @return The depth of the entity in the tree.*/
    pub fn get_depth(&self, entity: u64, rel: u64) -> i32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_depth(world, entity, rel) };
        return result;
    }
    /** Count entities that have the specified id.
    Returns the number of entities that have the specified id.

    @param world The world.
    @param entity The id to search for.
    @return The number of entities that have the id.*/
    pub fn count_id(&self, entity: u64) -> i32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_count_id(world, entity) };
        return result;
    }
    /** Get the current scope.
    Get the scope set by ecs_set_scope(). If no scope is set, this operation will
    return 0.

    @param world The world.
    @return The current scope.*/
    pub fn get_scope(&self) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_scope(world) };
        return result;
    }
    /** Returns whether specified id a tag.
    This operation returns whether the specified type is a tag (a component
    without data/size).

    An id is a tag when:
    - it is an entity without the EcsComponent component
    - it has an EcsComponent with size member set to 0
    - it is a pair where both elements are a tag
    - it is a pair where the first element has the EcsPairIsTag tag

    @param world The world.
    @param id The id.
    @return Whether the provided id is a tag.*/
    pub fn id_is_tag(&self, id: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_id_is_tag(world, id) };
        return result;
    }
    /** Returns whether specified id is in use.
    This operation returns whether an id is in use in the world. An id is in use
    if it has been added to one or more tables.

    @param world The world.
    @param id The id.
    @return Whether the id is in use.*/
    pub fn id_in_use(&self, id: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_id_in_use(world, id) };
        return result;
    }
    /** Get the type for an id.
    This operation returns the component id for an id, if the id is associated
    with a type. For a regular component with a non-zero size (an entity with the
    EcsComponent component) the operation will return the entity itself.

    For an entity that does not have the EcsComponent component, or with an
    EcsComponent value with size 0, the operation will return 0.

    For a pair id the operation will return the type associated with the pair, by
    applying the following querys in order:
    - The first pair element is returned if it is a component
    - 0 is returned if the relationship entity has the Tag property
    - The second pair element is returned if it is a component
    - 0 is returned.

    @param world The world.
    @param id The id.
    @return The type id of the id.*/
    pub fn get_typeid(&self, id: u64) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_typeid(world, id) };
        return result;
    }
    /** Utility to check if id is valid.
    A valid id is an id that can be added to an entity. Invalid ids are:
    - ids that contain wildcards
    - ids that contain invalid entities
    - ids that are 0 or contain 0 entities

    Note that the same rules apply to removing from an entity, with the exception
    of wildcards.

    @param world The world.
    @param id The id.
    @return True if the id is valid.*/
    pub fn id_is_valid(&self, id: u64) -> bool {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_id_is_valid(world, id) };
        return result;
    }
    /** Get flags associated with id.
    This operation returns the internal flags (see api_flags.h) that are
    associated with the provided id.

    @param world The world.
    @param id The id.
    @return Flags associated with the id, or 0 if the id is not in use.*/
    pub fn id_get_flags(&self, id: u64) -> u32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_id_get_flags(world, id) };
        return result;
    }
    /** Get type index for id.
    This operation returns the index for an id in the table's type.

    @param world The world.
    @param table The table.
    @param id The id.
    @return The index of the id in the table type, or -1 if not found.*/
    pub fn table_get_type_index(&self, table: &TableRef, id: u64) -> i32 {
        let world = self.ptr.cast_const();
        let table = table.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_get_type_index(world, table, id) };
        return result;
    }
    /** Get column index for id.
    This operation returns the column index for an id in the table's type. If the
    id is not a component, the function will return -1.

    @param world The world.
    @param table The table.
    @param id The component id.
    @return The column index of the id, or -1 if not found/not a component.*/
    pub fn table_get_column_index(&self, table: &TableRef, id: u64) -> i32 {
        let world = self.ptr.cast_const();
        let table = table.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_get_column_index(world, table, id) };
        return result;
    }
    /** Test if table has id.
    Same as ecs_table_get_type_index(world, table, id) != -1.

    @param world The world.
    @param table The table.
    @param id The id.
    @return True if the table has the id, false if the table doesn't.*/
    pub fn table_has_id(&self, table: &TableRef, id: u64) -> bool {
        let world = self.ptr.cast_const();
        let table = table.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_has_id(world, table, id) };
        return result;
    }
    /** Return depth for table in tree for relationship rel.
    Depth is determined by counting the number of targets encountered while
    traversing up the relationship tree for rel. Only acyclic relationships are
    supported.

    @param world The world.
    @param table The table.
    @param rel The relationship.
    @return The depth of the table in the tree.*/
    pub fn table_get_depth(&self, table: &TableRef, rel: u64) -> i32 {
        let world = self.ptr.cast_const();
        let table = table.ptr.cast_const();
        let result = unsafe { flecsys::ecs_table_get_depth(world, table, rel) };
        return result;
    }
    /** Get current timeout value for the specified timer.
    This operation returns the value set by ecs_set_timeout(). If no timer is
    active for this entity, the operation returns 0.

    After the timeout expires the EcsTimer component is removed from the entity.
    This means that if ecs_get_timeout() is invoked after the timer is expired, the
    operation will return 0.

    The timer is synchronous, and is incremented each frame by delta_time.

    The tick_source entity will be a tick source after this operation. Tick
    sources can be read by getting the EcsTickSource component. If the tick
    source ticked this frame, the 'tick' member will be true. When the tick
    source is a system, the system will tick when the timer ticks.

    @param world The world.
    @param tick_source The timer.
    @return The current timeout value, or 0 if no timer is active.*/
    pub fn get_timeout(&self, tick_source: u64) -> f32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_timeout(world, tick_source) };
        return result;
    }
    /** Get current interval value for the specified timer.
    This operation returns the value set by ecs_set_interval(). If the entity is
    not a timer, the operation will return 0.

    @param world The world.
    @param tick_source The timer for which to set the interval.
    @return The current interval value, or 0 if no timer is active.*/
    pub fn get_interval(&self, tick_source: u64) -> f32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_interval(world, tick_source) };
        return result;
    }
    /** Get the current pipeline.
    This operation gets the current pipeline.

    @param world The world.
    @return The current pipeline.*/
    pub fn get_pipeline(&self) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_pipeline(world) };
        return result;
    }
    /** Return number of active alerts for entity.
    When a valid alert entity is specified for the alert parameter, the operation
    will return whether the specified alert is active for the entity. When no
    alert is specified, the operation will return the total number of active
    alerts for the entity.

    @param world The world.
    @param entity The entity.
    @param alert The alert to test for (optional).
    @return The number of active alerts for the entity.*/
    pub fn get_alert_count(&self, entity: u64, alert: u64) -> i32 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_alert_count(world, entity, alert) };
        return result;
    }
    /** Return alert instance for specified alert.
    This operation returns the alert instance for the specified alert. If the
    alert is not active for the entity, the operation will return 0.

    @param world The world.
    @param entity The entity.
    @param alert The alert to test for.
    @return The alert instance for the specified alert.*/
    pub fn get_alert(&self, entity: u64, alert: u64) -> u64 {
        let world = self.ptr.cast_const();
        let result = unsafe { flecsys::ecs_get_alert(world, entity, alert) };
        return result;
    }
}
pub struct WorldMut {
    ptr: *mut flecsys::ecs_world_t,
}
impl WorldMut {
    /** Delete a world.
    This operation deletes the world, and everything it contains.

    @param world The world to delete.
    @return Zero if successful, non-zero if failed.*/
    pub fn fini(&mut self) -> i32 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_fini(world) };
        return result;
    }
    /** Begin frame.
    When an application does not use ecs_progress() to control the main loop, it
    can still use Flecs features such as FPS limiting and time measurements. This
    operation needs to be invoked whenever a new frame is about to get processed.

    Calls to ecs_frame_begin() must always be followed by ecs_frame_end().

    The function accepts a delta_time parameter, which will get passed to
    systems. This value is also used to compute the amount of time the function
    needs to sleep to ensure it does not exceed the target_fps, when it is set.
    When 0 is provided for delta_time, the time will be measured.

    This function should only be ran from the main thread.

    @param world The world.
    @param delta_time Time elapsed since the last frame.
    @return The provided delta_time, or measured time if 0 was provided.*/
    pub fn frame_begin(&mut self, delta_time: f32) -> f32 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_frame_begin(world, delta_time) };
        return result;
    }
    /** End frame.
    This operation must be called at the end of the frame, and always after
    ecs_frame_begin().

    @param world The world.*/
    pub fn frame_end(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_frame_end(world) };
    }
    /** Signal exit
    This operation signals that the application should quit. It will cause
    ecs_progress() to return false.

    @param world The world to quit.*/
    pub fn quit(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_quit(world) };
    }
    /** Measure frame time.
    Frame time measurements measure the total time passed in a single frame, and
    how much of that time was spent on systems and on merging.

    Frame time measurements add a small constant-time overhead to an application.
    When an application sets a target FPS, frame time measurements are enabled by
    default.

    @param world The world.
    @param enable Whether to enable or disable frame time measuring.*/
    pub fn measure_frame_time(&mut self, enable: bool) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_measure_frame_time(world, enable) };
    }
    /** Measure system time.
    System time measurements measure the time spent in each system.

    System time measurements add overhead to every system invocation and
    therefore have a small but measurable impact on application performance.
    System time measurements must be enabled before obtaining system statistics.

    @param world The world.
    @param enable Whether to enable or disable system time measuring.*/
    pub fn measure_system_time(&mut self, enable: bool) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_measure_system_time(world, enable) };
    }
    /** Set target frames per second (FPS) for application.
    Setting the target FPS ensures that ecs_progress() is not invoked faster than
    the specified FPS. When enabled, ecs_progress() tracks the time passed since
    the last invocation, and sleeps the remaining time of the frame (if any).

    This feature ensures systems are ran at a consistent interval, as well as
    conserving CPU time by not running systems more often than required.

    Note that ecs_progress() only sleeps if there is time left in the frame. Both
    time spent in flecs as time spent outside of flecs are taken into
    account.

    @param world The world.
    @param fps The target FPS.*/
    pub fn set_target_fps(&mut self, fps: f32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_target_fps(world, fps) };
    }
    /** Begin readonly mode.
     This operation puts the world in readonly mode, which disallows mutations on
     the world. Readonly mode exists so that internal mechanisms can implement
     optimizations that certain aspects of the world to not change, while also
     providing a mechanism for applications to prevent accidental mutations in,
     for example, multithreaded applications.

     Readonly mode is a stronger version of deferred mode. In deferred mode
     ECS operations such as add/remove/set/delete etc. are added to a command
     queue to be executed later. In readonly mode, operations that could break
     scheduler logic (such as creating systems, queries) are also disallowed.

     Readonly mode itself has a single threaded and a multi threaded mode. In
     single threaded mode certain mutations on the world are still allowed, for
     example:
     - Entity liveliness operations (such as new, make_alive), so that systems are
       able to create new entities.
     - Implicit component registration, so that this works from systems
     - Mutations to supporting data structures for the evaluation of uncached
       queries (filters), so that these can be created on the fly.

     These mutations are safe in a single threaded applications, but for
     multithreaded applications the world needs to be entirely immutable. For this
     purpose multi threaded readonly mode exists, which disallows all mutations on
     the world. This means that in multi threaded applications, entity liveliness
     operations, implicit component registration, and on-the-fly query creation
     are not guaranteed to work.

     While in readonly mode, applications can still enqueue ECS operations on a
     stage. Stages are managed automatically when using the pipeline addon and
     ecs_progress(), but they can also be configured manually as shown here:

     @code
     // Number of stages typically corresponds with number of threads
     ecs_set_stage_count(world, 2);
     ecs_stage_t *stage = ecs_get_stage(world, 1);

     ecs_readonly_begin(world);
     ecs_add(world, e, Tag); // readonly assert
     ecs_add(stage, e, Tag); // OK
     @endcode

     When an attempt is made to perform an operation on a world in readonly mode,
     the code will throw an assert saying that the world is in readonly mode.

     A call to ecs_readonly_begin() must be followed up with ecs_readonly_end().
     When ecs_readonly_end() is called, all enqueued commands from configured
     stages are merged back into the world. Calls to ecs_readonly_begin() and
     ecs_readonly_end() should always happen from a context where the code has
     exclusive access to the world. The functions themselves are not thread safe.

     In a typical application, a (non-exhaustive) call stack that uses
     ecs_readonly_begin() and ecs_readonly_end() will look like this:

     @code
     ecs_progress()
       ecs_readonly_begin()
         ecs_defer_begin()

           // user code

       ecs_readonly_end()
         ecs_defer_end()
    @endcode

     @param world The world
     @param multi_threaded Whether to enable readonly/multi threaded mode.
     @return Whether world is in readonly mode.*/
    pub fn readonly_begin(&mut self, multi_threaded: bool) -> bool {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_readonly_begin(world, multi_threaded) };
        return result;
    }
    /** End readonly mode.
    This operation ends readonly mode, and must be called after
    ecs_readonly_begin(). Operations that were deferred while the world was in
    readonly mode will be flushed.

    @param world The world*/
    pub fn readonly_end(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_readonly_end(world) };
    }
    /** Merge world or stage.
    When automatic merging is disabled, an application can call this
    operation on either an individual stage, or on the world which will merge
    all stages. This operation may only be called when staging is not enabled
    (either after ecs_progress() or after ecs_readonly_end()).

    This operation may be called on an already merged stage or world.

    @param world The world.*/
    pub fn merge(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_merge(world) };
    }
    /** Defer operations until end of frame.
    When this operation is invoked while iterating, operations inbetween the
    ecs_defer_begin() and ecs_defer_end() operations are executed at the end
    of the frame.

    This operation is thread safe.

    @param world The world.
    @return true if world changed from non-deferred mode to deferred mode.*/
    pub fn defer_begin(&mut self) -> bool {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_defer_begin(world) };
        return result;
    }
    /** End block of operations to defer.
    See ecs_defer_begin().

    This operation is thread safe.

    @param world The world.
    @return true if world changed from deferred mode to non-deferred mode.*/
    pub fn defer_end(&mut self) -> bool {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_defer_end(world) };
        return result;
    }
    /** Suspend deferring but do not flush queue.
    This operation can be used to do an undeferred operation while not flushing
    the operations in the queue.

    An application should invoke ecs_defer_resume() before ecs_defer_end() is called.
    The operation may only be called when deferring is enabled.

    @param world The world.*/
    pub fn defer_suspend(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_defer_suspend(world) };
    }
    /** Resume deferring.
    See ecs_defer_suspend().

    @param world The world.*/
    pub fn defer_resume(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_defer_resume(world) };
    }
    /** Configure world to have N stages.
    This initializes N stages, which allows applications to defer operations to
    multiple isolated defer queues. This is typically used for applications with
    multiple threads, where each thread gets its own queue, and commands are
    merged when threads are synchronized.

    Note that the ecs_set_threads() function already creates the appropriate
    number of stages. The ecs_set_stage_count() operation is useful for applications
    that want to manage their own stages and/or threads.

    @param world The world.
    @param stages The number of stages.*/
    pub fn set_stage_count(&mut self, stages: i32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_stage_count(world, stages) };
    }
    /** Free unmanaged stage.

    @param stage The stage to free.*/
    pub fn stage_free(&mut self) {
        let stage = self.ptr;
        let result = unsafe { flecsys::ecs_stage_free(stage) };
    }
    /** Dimension the world for a specified number of entities.
    This operation will preallocate memory in the world for the specified number
    of entities. Specifying a number lower than the current number of entities in
    the world will have no effect.

    @param world The world.
    @param entity_count The number of entities to preallocate.*/
    pub fn dim(&mut self, entity_count: i32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_dim(world, entity_count) };
    }
    /** Set a range for issuing new entity ids.
    This function constrains the entity identifiers returned by ecs_new_w() to the
    specified range. This operation can be used to ensure that multiple processes
    can run in the same simulation without requiring a central service that
    coordinates issuing identifiers.

    If id_end is set to 0, the range is infinite. If id_end is set to a non-zero
    value, it has to be larger than id_start. If id_end is set and ecs_new is
    invoked after an id is issued that is equal to id_end, the application will
    abort.

    @param world The world.
    @param id_start The start of the range.
    @param id_end The end of the range.*/
    pub fn set_entity_range(&mut self, id_start: u64, id_end: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_entity_range(world, id_start, id_end) };
    }
    /** Enable/disable range limits.
    When an application is both a receiver of range-limited entities and a
    producer of range-limited entities, range checking needs to be temporarily
    disabled when inserting received entities. Range checking is disabled on a
    stage, so setting this value is thread safe.

    @param world The world.
    @param enable True if range checking should be enabled, false to disable.
    @return The previous value.*/
    pub fn enable_range_check(&mut self, enable: bool) -> bool {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_enable_range_check(world, enable) };
        return result;
    }
    /** Force aperiodic actions.
    The world may delay certain operations until they are necessary for the
    application to function correctly. This may cause observable side effects
    such as delayed triggering of events, which can be inconvenient when for
    example running a test suite.

    The flags parameter specifies which aperiodic actions to run. Specify 0 to
    run all actions. Supported flags start with 'EcsAperiodic'. Flags identify
    internal mechanisms and may change unannounced.

    @param world The world.
    @param flags The flags specifying which actions to run.*/
    pub fn run_aperiodic(&mut self, flags: u32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_run_aperiodic(world, flags) };
    }
    /** Cleanup empty tables.
    This operation cleans up empty tables that meet certain conditions. Having
    large amounts of empty tables does not negatively impact performance of the
    ECS, but can take up considerable amounts of memory, especially in
    applications with many components, and many components per entity.

    The generation specifies the minimum number of times this operation has
    to be called before an empty table is cleaned up. If a table becomes non
    empty, the generation is reset.

    The operation allows for both a "clear" generation and a "delete"
    generation. When the clear generation is reached, the table's
    resources are freed (like component arrays) but the table itself is not
    deleted. When the delete generation is reached, the empty table is deleted.

    By specifying a non-zero id the cleanup logic can be limited to tables with
    a specific (component) id. The operation will only increase the generation
    count of matching tables.

    The min_id_count specifies a lower bound for the number of components a table
    should have. Often the more components a table has, the more specific it is
    and therefore less likely to be reused.

    The time budget specifies how long the operation should take at most.

    @param world The world.
    @param id Optional component filter for the tables to evaluate.
    @param clear_generation Free table data when generation > clear_generation.
    @param delete_generation Delete table when generation > delete_generation.
    @param min_id_count Minimum number of component ids the table should have.
    @param time_budget_seconds Amount of time operation is allowed to spend.
    @return Number of deleted tables.*/
    pub fn delete_empty_tables(
        &mut self,
        id: u64,
        clear_generation: u16,
        delete_generation: u16,
        min_id_count: i32,
        time_budget_seconds: f64,
    ) -> i32 {
        let world = self.ptr;
        let result = unsafe {
            flecsys::ecs_delete_empty_tables(
                world,
                id,
                clear_generation,
                delete_generation,
                min_id_count,
                time_budget_seconds,
            )
        };
        return result;
    }
    /** Create new entity id.
    This operation returns an unused entity id. This operation is guaranteed to
    return an empty entity as it does not use values set by ecs_set_scope() or
    ecs_set_with().

    @param world The world.
    @return The new entity id.*/
    pub fn new(&mut self) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_new(world) };
        return result;
    }
    /** Create new low id.
    This operation returns a new low id. Entity ids start after the
    FLECS_HI_COMPONENT_ID constant. This reserves a range of low ids for things
    like components, and allows parts of the code to optimize operations.

    Note that FLECS_HI_COMPONENT_ID does not represent the maximum number of
    components that can be created, only the maximum number of components that
    can take advantage of these optimizations.

    This operation is guaranteed to return an empty entity as it does not use
    values set by ecs_set_scope() or ecs_set_with().

    This operation does not recycle ids.

    @param world The world.
    @return The new component id.*/
    pub fn new_low_id(&mut self) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_new_low_id(world) };
        return result;
    }
    /** Create new entity with (component) id.
    This operation creates a new entity with an optional (component) id. When 0
    is passed to the id parameter, no component is added to the new entity.

    @param world The world.
    @param id The component id to initialize the new entity with.
    @return The new entity.*/
    pub fn new_w_id(&mut self, id: u64) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_new_w_id(world, id) };
        return result;
    }
    /** Create new entity in table.
    This operation creates a new entity in the specified table.

    @param world The world.
    @param table The table to which to add the new entity.
    @return The new entity.*/
    pub fn new_w_table(&mut self, table: &mut TableMut) -> u64 {
        let world = self.ptr;
        let table = table.ptr;
        let result = unsafe { flecsys::ecs_new_w_table(world, table) };
        return result;
    }
    /** Clone an entity
    This operation clones the components of one entity into another entity. If
    no destination entity is provided, a new entity will be created. Component
    values are not copied unless copy_value is true.

    If the source entity has a name, it will not be copied to the destination
    entity. This is to prevent having two entities with the same name under the
    same parent, which is not allowed.

    @param world The world.
    @param dst The entity to copy the components to.
    @param src The entity to copy the components from.
    @param copy_value If true, the value of components will be copied to dst.
    @return The destination entity.*/
    pub fn clone(&mut self, dst: u64, src: u64, copy_value: bool) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_clone(world, dst, src, copy_value) };
        return result;
    }
    /** Delete an entity.
    This operation will delete an entity and all of its components. The entity id
    will be made available for recycling. If the entity passed to ecs_delete() is
    not alive, the operation will have no side effects.

    @param world The world.
    @param entity The entity.*/
    pub fn delete(&mut self, entity: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_delete(world, entity) };
    }
    /** Delete all entities with the specified id.
    This will delete all entities (tables) that have the specified id. The id
    may be a wildcard and/or a pair.

    @param world The world.
    @param id The id.*/
    pub fn delete_with(&mut self, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_delete_with(world, id) };
    }
    /** Add a (component) id to an entity.
    This operation adds a single (component) id to an entity. If the entity
    already has the id, this operation will have no side effects.

    @param world The world.
    @param entity The entity.
    @param id The id to add.*/
    pub fn add_id(&mut self, entity: u64, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_add_id(world, entity, id) };
    }
    /** Remove a (component) id from an entity.
    This operation removes a single (component) id to an entity. If the entity
    does not have the id, this operation will have no side effects.

    @param world The world.
    @param entity The entity.
    @param id The id to remove.*/
    pub fn remove_id(&mut self, entity: u64, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_remove_id(world, entity, id) };
    }
    /** Add auto override for (component) id.
    An auto override is a component that is automatically added to an entity when
    it is instantiated from a prefab. Auto overrides are added to the entity that
    is inherited from (usually a prefab). For example:

    @code
    ecs_entity_t prefab = ecs_insert(world,
      ecs_value(Position, {10, 20}),
      ecs_value(Mass, {100}));

    ecs_auto_override(world, prefab, Position);

    ecs_entity_t inst = ecs_new_w_pair(world, EcsIsA, prefab);
    assert(ecs_owns(world, inst, Position)); // true
    assert(ecs_owns(world, inst, Mass)); // false
    @endcode

    An auto override is equivalent to a manual override:

    @code
    ecs_entity_t prefab = ecs_insert(world,
      ecs_value(Position, {10, 20}),
      ecs_value(Mass, {100}));

    ecs_entity_t inst = ecs_new_w_pair(world, EcsIsA, prefab);
    assert(ecs_owns(world, inst, Position)); // false
    ecs_add(world, inst, Position); // manual override
    assert(ecs_owns(world, inst, Position)); // true
    assert(ecs_owns(world, inst, Mass)); // false
    @endcode

    This operation is equivalent to manually adding the id with the AUTO_OVERRIDE
    bit applied:

    @code
    ecs_add_id(world, entity, ECS_AUTO_OVERRIDE | id);
    @endcode

    When a component is overridden and inherited from a prefab, the value from
    the prefab component is copied to the instance. When the component is not
    inherited from a prefab, it is added to the instance as if using ecs_add_id.

    Overriding is the default behavior on prefab instantiation. Auto overriding
    is only useful for components with the (OnInstantiate, Inherit) trait.
    When a component has the (OnInstantiate, DontInherit) trait and is overridden
    the component is added, but the value from the prefab will not be copied.

    @param world The world.
    @param entity The entity.
    @param id The (component) id to auto override.*/
    pub fn auto_override_id(&mut self, entity: u64, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_auto_override_id(world, entity, id) };
    }
    /** Clear all components.
    This operation will remove all components from an entity.

    @param world The world.
    @param entity The entity.*/
    pub fn clear(&mut self, entity: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_clear(world, entity) };
    }
    /** Remove all instances of the specified (component) id.
    This will remove the specified id from all entities (tables). The id may be
    a wildcard and/or a pair.

    @param world The world.
    @param id The id.*/
    pub fn remove_all(&mut self, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_remove_all(world, id) };
    }
    /** Set current with id.
    New entities are automatically created with the specified id.

    @param world The world.
    @param id The id.
    @return The previous id.*/
    pub fn set_with(&mut self, id: u64) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_with(world, id) };
        return result;
    }
    /** Enable or disable entity.
    This operation enables or disables an entity by adding or removing the
    EcsDisabled tag. A disabled entity will not be matched with any systems,
    unless the system explicitly specifies the EcsDisabled tag.

    @param world The world.
    @param entity The entity to enable or disable.
    @param enabled true to enable the entity, false to disable.*/
    pub fn enable(&mut self, entity: u64, enabled: bool) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_enable(world, entity, enabled) };
    }
    /** Enable or disable component.
    Enabling or disabling a component does not add or remove a component from an
    entity, but prevents it from being matched with queries. This operation can
    be useful when a component must be temporarily disabled without destroying
    its value. It is also a more performant operation for when an application
    needs to add/remove components at high frequency, as enabling/disabling is
    cheaper than a regular add or remove.

    @param world The world.
    @param entity The entity.
    @param id The component.
    @param enable True to enable the component, false to disable.*/
    pub fn enable_id(&mut self, entity: u64, id: u64, enable: bool) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_enable_id(world, entity, id, enable) };
    }
    /** Signal that a component has been modified.
    This operation is usually used after modifying a component value obtained by
    ecs_ensure_id(). The operation will mark the component as dirty, and invoke
    OnSet observers and hooks.

    @param world The world.
    @param entity The entity.
    @param id The id of the component that was modified.*/
    pub fn modified_id(&mut self, entity: u64, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_modified_id(world, entity, id) };
    }
    /** Ensure id is alive.
    This operation ensures that the provided id is alive. This is useful in
    scenarios where an application has an existing id that has not been created
    with ecs_new_w() (such as a global constant or an id from a remote application).

    When this operation is successful it guarantees that the provided id exists,
    is valid and is alive.

    Before this operation the id must either not be alive or have a generation
    that is equal to the passed in entity.

    If the provided id has a non-zero generation count and the id does not exist
    in the world, the id will be created with the specified generation.

    If the provided id is alive and has a generation count that does not match
    the provided id, the operation will fail.

    @param world The world.
    @param entity The entity id to make alive.*/
    pub fn make_alive(&mut self, entity: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_make_alive(world, entity) };
    }
    /** Same as ecs_make_alive(), but for (component) ids.
    An id can be an entity or pair, and can contain id flags. This operation
    ensures that the entity (or entities, for a pair) are alive.

    When this operation is successful it guarantees that the provided id can be
    used in operations that accept an id.

    Since entities in a pair do not encode their generation ids, this operation
    will not fail when an entity with non-zero generation count already exists in
    the world.

    This is different from ecs_make_alive(), which will fail if attempted with an id
    that has generation 0 and an entity with a non-zero generation is currently
    alive.

    @param world The world.
    @param id The id to make alive.*/
    pub fn make_alive_id(&mut self, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_make_alive_id(world, id) };
    }
    /** Override the generation of an entity.
    The generation count of an entity is increased each time an entity is deleted
    and is used to test whether an entity id is alive.

    This operation overrides the current generation of an entity with the
    specified generation, which can be useful if an entity is externally managed,
    like for external pools, savefiles or netcode.

    This operation is similar to ecs_make_alive, except that it will also
    override the generation of an alive entity.

    @param world The world.
    @param entity Entity for which to set the generation with the new generation.*/
    pub fn set_generation(&mut self, entity: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_generation(world, entity) };
    }
    /** Set the current scope.
    This operation sets the scope of the current stage to the provided entity.
    As a result new entities will be created in this scope, and lookups will be
    relative to the provided scope.

    It is considered good practice to restore the scope to the old value.

    @param world The world.
    @param scope The entity to use as scope.
    @return The previous scope.*/
    pub fn set_scope(&mut self, scope: u64) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_scope(world, scope) };
        return result;
    }
    /** Lock a table.
    When a table is locked, modifications to it will throw an assert. When the
    table is locked recursively, it will take an equal amount of unlock
    operations to actually unlock the table.

    Table locks can be used to build safe iterators where it is guaranteed that
    the contents of a table are not modified while it is being iterated.

    The operation only works when called on the world, and has no side effects
    when called on a stage. The assumption is that when called on a stage,
    operations are deferred already.

    @param world The world.
    @param table The table to lock.*/
    pub fn table_lock(&mut self, table: &mut TableMut) {
        let world = self.ptr;
        let table = table.ptr;
        let result = unsafe { flecsys::ecs_table_lock(world, table) };
    }
    /** Unlock a table.
    Must be called after calling ecs_table_lock().

    @param world The world.
    @param table The table to unlock.*/
    pub fn table_unlock(&mut self, table: &mut TableMut) {
        let world = self.ptr;
        let table = table.ptr;
        let result = unsafe { flecsys::ecs_table_unlock(world, table) };
    }
    /** Swaps two elements inside the table. This is useful for implementing custom
    table sorting algorithms.
    @param world The world
    @param table The table to swap elements in
    @param row_1 Table element to swap with row_2
    @param row_2 Table element to swap with row_1*/
    pub fn table_swap_rows(&mut self, table: &mut TableMut, row_1: i32, row_2: i32) {
        let world = self.ptr;
        let table = table.ptr;
        let result = unsafe { flecsys::ecs_table_swap_rows(world, table, row_1, row_2) };
    }
    /** Set timer timeout.
    This operation executes any systems associated with the timer after the
    specified timeout value. If the entity contains an existing timer, the
    timeout value will be reset. The timer can be started and stopped with
    ecs_start_timer() and ecs_stop_timer().

    The timer is synchronous, and is incremented each frame by delta_time.

    The tick_source entity will be a tick source after this operation. Tick
    sources can be read by getting the EcsTickSource component. If the tick
    source ticked this frame, the 'tick' member will be true. When the tick
    source is a system, the system will tick when the timer ticks.

    @param world The world.
    @param tick_source The timer for which to set the timeout (0 to create one).
    @param timeout The timeout value.
    @return The timer entity.*/
    pub fn set_timeout(&mut self, tick_source: u64, timeout: f32) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_timeout(world, tick_source, timeout) };
        return result;
    }
    /** Set timer interval.
    This operation will continuously invoke systems associated with the timer
    after the interval period expires. If the entity contains an existing timer,
    the interval value will be reset.

    The timer is synchronous, and is incremented each frame by delta_time.

    The tick_source entity will be a tick source after this operation. Tick
    sources can be read by getting the EcsTickSource component. If the tick
    source ticked this frame, the 'tick' member will be true. When the tick
    source is a system, the system will tick when the timer ticks.

    @param world The world.
    @param tick_source The timer for which to set the interval (0 to create one).
    @param interval The interval value.
    @return The timer entity.*/
    pub fn set_interval(&mut self, tick_source: u64, interval: f32) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_interval(world, tick_source, interval) };
        return result;
    }
    /** Start timer.
    This operation resets the timer and starts it with the specified timeout.

    @param world The world.
    @param tick_source The timer to start.*/
    pub fn start_timer(&mut self, tick_source: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_start_timer(world, tick_source) };
    }
    /** Stop timer
    This operation stops a timer from triggering.

    @param world The world.
    @param tick_source The timer to stop.*/
    pub fn stop_timer(&mut self, tick_source: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_stop_timer(world, tick_source) };
    }
    /** Reset time value of timer to 0.
    This operation resets the timer value to 0.

    @param world The world.
    @param tick_source The timer to reset.*/
    pub fn reset_timer(&mut self, tick_source: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_reset_timer(world, tick_source) };
    }
    /** Enable randomizing initial time value of timers.
    Initializes timers with a random time value, which can improve scheduling as
    systems/timers for the same interval don't all happen on the same tick.

    @param world The world.*/
    pub fn randomize_timers(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_randomize_timers(world) };
    }
    /** Set rate filter.
    This operation initializes a rate filter. Rate filters sample tick sources
    and tick at a configurable multiple. A rate filter is a tick source itself,
    which means that rate filters can be chained.

    Rate filters enable deterministic system execution which cannot be achieved
    with interval timers alone. For example, if timer A has interval 2.0 and
    timer B has interval 4.0, it is not guaranteed that B will tick at exactly
    twice the multiple of A. This is partly due to the indeterministic nature of
    timers, and partly due to floating point rounding errors.

    Rate filters can be combined with timers (or other rate filters) to ensure
    that a system ticks at an exact multiple of a tick source (which can be
    another system). If a rate filter is created with a rate of 1 it will tick
    at the exact same time as its source.

    If no tick source is provided, the rate filter will use the frame tick as
    source, which corresponds with the number of times ecs_progress() is called.

    The tick_source entity will be a tick source after this operation. Tick
    sources can be read by getting the EcsTickSource component. If the tick
    source ticked this frame, the 'tick' member will be true. When the tick
    source is a system, the system will tick when the timer ticks.

    @param world The world.
    @param tick_source The rate filter entity (0 to create one).
    @param rate The rate to apply.
    @param source The tick source (0 to use frames)
    @return The filter entity.*/
    pub fn set_rate(&mut self, tick_source: u64, rate: i32, source: u64) -> u64 {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_rate(world, tick_source, rate, source) };
        return result;
    }
    /** Assign tick source to system.
    Systems can be their own tick source, which can be any of the tick sources
    (one shot timers, interval times and rate filters). However, in some cases it
    is must be guaranteed that different systems tick on the exact same frame.

    This cannot be guaranteed by giving two systems the same interval/rate filter
    as it is possible that one system is (for example) disabled, which would
    cause the systems to go out of sync. To provide these guarantees, systems
    must use the same tick source, which is what this operation enables.

    When two systems share the same tick source, it is guaranteed that they tick
    in the same frame. The provided tick source can be any entity that is a tick
    source, including another system. If the provided entity is not a tick source
    the system will not be ran.

    To disassociate a tick source from a system, use 0 for the tick_source
    parameter.

    @param world The world.
    @param system The system to associate with the timer.
    @param tick_source The tick source to associate with the system.*/
    pub fn set_tick_source(&mut self, system: u64, tick_source: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_tick_source(world, system, tick_source) };
    }
    /** Set a custom pipeline.
    This operation sets the pipeline to run when ecs_progress() is invoked.

    @param world The world.
    @param pipeline The pipeline to set.*/
    pub fn set_pipeline(&mut self, pipeline: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_pipeline(world, pipeline) };
    }
    /** Progress a world.
    This operation progresses the world by running all systems that are both
    enabled and periodic on their matching entities.

    An application can pass a delta_time into the function, which is the time
    passed since the last frame. This value is passed to systems so they can
    update entity values proportional to the elapsed time since their last
    invocation.

    When an application passes 0 to delta_time, ecs_progress() will automatically
    measure the time passed since the last frame. If an application does not uses
    time management, it should pass a non-zero value for delta_time (1.0 is
    recommended). That way, no time will be wasted measuring the time.

    @param world The world to progress.
    @param delta_time The time passed since the last frame.
    @return false if ecs_quit() has been called, true otherwise.*/
    pub fn progress(&mut self, delta_time: f32) -> bool {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_progress(world, delta_time) };
        return result;
    }
    /** Set time scale.
    Increase or decrease simulation speed by the provided multiplier.

    @param world The world.
    @param scale The scale to apply (default = 1).*/
    pub fn set_time_scale(&mut self, scale: f32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_time_scale(world, scale) };
    }
    /** Reset world clock.
    Reset the clock that keeps track of the total time passed in the simulation.

    @param world The world.*/
    pub fn reset_clock(&mut self) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_reset_clock(world) };
    }
    /** Run pipeline.
    This will run all systems in the provided pipeline. This operation may be
    invoked from multiple threads, and only when staging is disabled, as the
    pipeline manages staging and, if necessary, synchronization between threads.

    If 0 is provided for the pipeline id, the default pipeline will be ran (this
    is either the builtin pipeline or the pipeline set with set_pipeline()).

    When using progress() this operation will be invoked automatically for the
    default pipeline (either the builtin pipeline or the pipeline set with
    set_pipeline()). An application may run additional pipelines.

    @param world The world.
    @param pipeline The pipeline to run.
    @param delta_time The delta_time to pass to systems.*/
    pub fn run_pipeline(&mut self, pipeline: u64, delta_time: f32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_run_pipeline(world, pipeline, delta_time) };
    }
    /** Set number of worker threads.
    Setting this value to a value higher than 1 will start as many threads and
    will cause systems to evenly distribute matched entities across threads. The
    operation may be called multiple times to reconfigure the number of threads
    used, but never while running a system / pipeline.
    Calling ecs_set_threads() will also end the use of task threads setup with
    ecs_set_task_threads() and vice-versa.

    @param world The world.
    @param threads The number of threads to create.*/
    pub fn set_threads(&mut self, threads: i32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_threads(world, threads) };
    }
    /** Set number of worker task threads.
    ecs_set_task_threads() is similar to ecs_set_threads(), except threads are treated
    as short-lived tasks and will be created and joined around each update of the world.
    Creation and joining of these tasks will use the os_api_t tasks APIs rather than the
    the standard thread API functions, although they may be the same if desired.
    This function is useful for multithreading world updates using an external
    asynchronous job system rather than long running threads by providing the APIs
    to create tasks for your job system and then wait on their conclusion.
    The operation may be called multiple times to reconfigure the number of task threads
    used, but never while running a system / pipeline.
    Calling ecs_set_task_threads() will also end the use of threads setup with
    ecs_set_threads() and vice-versa

    @param world The world.
    @param task_threads The number of task threads to create.*/
    pub fn set_task_threads(&mut self, task_threads: i32) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_set_task_threads(world, task_threads) };
    }
    /** Returns true if task thread use have been requested.

    @param world The world.
    @result Whether the world is using task threads.*/
    pub fn using_task_threads(&mut self) -> bool {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_using_task_threads(world) };
        return result;
    }
    /** Clear all entities associated with script.

    @param world The world.
    @param script The script entity.
    @param instance The script instance.*/
    pub fn script_clear(&mut self, script: u64, instance: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_script_clear(world, script, instance) };
    }
    pub fn cpp_enum_init(&mut self, id: u64) {
        let world = self.ptr;
        let result = unsafe { flecsys::ecs_cpp_enum_init(world, id) };
    }
}
