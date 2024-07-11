use super::*;
pub trait HttpServerTrait: HttpServerTraitManual {
    /** Start server.
 After this operation the server will be able to accept requests.

 @param server The server to start.
 @return Zero if successful, non-zero if failed.*/
    fn http_server_start(&mut self) -> i32 {
        let server = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_http_server_start(server) };
        return result;
    }
    /** Process server requests.
 This operation invokes the reply callback for each received request. No new
 requests will be enqueued while processing requests.

 @param server The server for which to process requests.*/
    fn http_server_dequeue(&mut self, delta_time: f32) {
        let server = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_http_server_dequeue(server, delta_time) };
        let _ = result;
    }
    /** Stop server.
 After this operation no new requests can be received.

 @param server The server.*/
    fn http_server_stop(&mut self) {
        let server = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_http_server_stop(server) };
        let _ = result;
    }
}
pub trait TableTrait: TableTraitManual {
    /** Return number of columns in table.
 Similar to ecs_table_get_type(table)->count, except that the column count
 only counts the number of components in a table.

 @param table The table.
 @return The number of columns in the table.*/
    fn table_column_count(&self) -> i32 {
        let table = self.as_ptr();
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
    fn table_type_to_column_index(&self, index: i32) -> i32 {
        let table = self.as_ptr();
        let result = unsafe { flecsys::ecs_table_type_to_column_index(table, index) };
        return result;
    }
    /** Convert column index to type index.
 Same as ecs_table_type_to_column_index(), but converts from an index in the
 column array to an index in the table type.

 @param table The table.
 @param index The column index.
 @return The index in the table type.*/
    fn table_column_to_type_index(&self, index: i32) -> i32 {
        let table = self.as_ptr();
        let result = unsafe { flecsys::ecs_table_column_to_type_index(table, index) };
        return result;
    }
    /** Get column size from table.
 This operation returns the component size for the provided index.

 @param table The table.
 @param index The column index.
 @return The component size, or 0 if the index is not a component.*/
    fn table_get_column_size(&self, index: i32) -> usize {
        let table = self.as_ptr();
        let result = unsafe { flecsys::ecs_table_get_column_size(table, index) };
        return result;
    }
    /** Returns the number of records in the table.
 This operation returns the number of records that have been populated through
 the regular (entity) API as well as the number of records that have been
 inserted using the direct access API.

 @param table The table.
 @return The number of records in a table.*/
    fn table_count(&self) -> i32 {
        let table = self.as_ptr();
        let result = unsafe { flecsys::ecs_table_count(table) };
        return result;
    }
    /** Test table for flags.
 Test if table has all of the provided flags. See
 include/flecs/private/api_flags.h for a list of table flags that can be used
 with this function.

 @param table The table.
 @param flags The flags to test for.
 @return Whether the specified flags are set for the table.*/
    fn table_has_flags(&mut self, flags: Flags32) -> bool {
        let table = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_table_has_flags(table, flags) };
        return result;
    }
}
pub trait WorldTrait: WorldTraitManual {
    /** Return whether a quit has been requested.

 @param world The world.
 @return Whether a quit has been requested.
 @see ecs_quit()*/
    fn should_quit(&self) -> bool {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_should_quit(world) };
        return result;
    }
    /** Test if deferring is enabled for current stage.

 @param world The world.
 @return True if deferred, false if not.*/
    fn is_deferred(&self) -> bool {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_is_deferred(world) };
        return result;
    }
    /** Get number of configured stages.
 Return number of stages set by ecs_set_stage_count().

 @param world The world.
 @return The number of stages used for threading.*/
    fn get_stage_count(&self) -> i32 {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_stage_count(world) };
        return result;
    }
    /** Test whether the current world is readonly.
 This function allows the code to test whether the currently used world
 is readonly or whether it allows for writing.

 @param world A pointer to a stage or the world.
 @return True if the world or stage is readonly.*/
    fn stage_is_readonly(&self) -> bool {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_stage_is_readonly(world) };
        return result;
    }
    /** Get stage id.
 The stage id can be used by an application to learn about which stage it is
 using, which typically corresponds with the worker thread id.

 @param world The world.
 @return The stage id.*/
    fn stage_get_id(&self) -> i32 {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_stage_get_id(world) };
        return result;
    }
    /** Get the largest issued entity id (not counting generation).

 @param world The world.
 @return The largest issued entity id.*/
    fn get_max_id(&self) -> Entity {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_max_id(world) };
        return result;
    }
    /** Get current with id.
 Get the id set with ecs_set_with().

 @param world The world.
 @return The last id provided to ecs_set_with().*/
    fn get_with(&self) -> Id {
        let world = self.as_ptr();
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
    fn is_enabled_id(&self, entity: Entity, id: Id) -> bool {
        let world = self.as_ptr();
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
    fn is_valid(&self, e: Entity) -> bool {
        let world = self.as_ptr();
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
    fn is_alive(&self, e: Entity) -> bool {
        let world = self.as_ptr();
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
    fn get_alive(&self, e: Entity) -> Entity {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_alive(world, e) };
        return result;
    }
    /** Test whether an entity exists.
 Similar as ecs_is_alive(), but ignores entity generation count.

 @param world The world.
 @param entity The entity.
 @return True if the entity exists, false if the entity does not exist.*/
    fn exists(&self, entity: Entity) -> bool {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_exists(world, entity) };
        return result;
    }
    /** Convert table to string.
 Same as ecs_type_str(world, ecs_table_get_type(table)). The result of this
 operation must be freed with ecs_os_free().

 @param world The world.
 @param table The table.
 @return The stringified table type.*/
    fn table_str(&self, table: &Table) -> Option<NullString> {
        let world = self.as_ptr();
        let table = table.as_ptr();
        let result = unsafe { flecsys::ecs_table_str(world, table) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer")
            .to_owned();
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Convert entity to string.
 Same as combining:
 - ecs_get_fullpath(world, entity)
 - ecs_type_str(world, ecs_get_type(world, entity))

 The result of this operation must be freed with ecs_os_free().

 @param world The world.
 @param entity The entity.
 @return The entity path with stringified type.*/
    fn entity_str(&self, entity: Entity) -> Option<NullString> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_entity_str(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer")
            .to_owned();
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Test if an entity has an id.
 This operation returns true if the entity has or inherits the specified id.

 @param world The world.
 @param entity The entity.
 @param id The id to test for.
 @return True if the entity has the id, false if not.*/
    fn has_id(&self, entity: Entity, id: Id) -> bool {
        let world = self.as_ptr();
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
    fn owns_id(&self, entity: Entity, id: Id) -> bool {
        let world = self.as_ptr();
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
    fn get_target(&self, entity: Entity, rel: Entity, index: i32) -> Entity {
        let world = self.as_ptr();
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
    fn get_parent(&self, entity: Entity) -> Entity {
        let world = self.as_ptr();
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
    fn get_target_for_id(&self, entity: Entity, rel: Entity, id: Id) -> Entity {
        let world = self.as_ptr();
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
    fn get_depth(&self, entity: Entity, rel: Entity) -> i32 {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_depth(world, entity, rel) };
        return result;
    }
    /** Count entities that have the specified id.
 Returns the number of entities that have the specified id.

 @param world The world.
 @param entity The id to search for.
 @return The number of entities that have the id.*/
    fn count_id(&self, entity: Id) -> i32 {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_count_id(world, entity) };
        return result;
    }
    /** Get the name of an entity.
 This will return the name stored in (EcsIdentifier, EcsName).

 @param world The world.
 @param entity The entity.
 @return The type of the entity, NULL if the entity has no name.*/
    fn get_name(&self, entity: Entity) -> Option<&NullStr> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_name(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Get the symbol of an entity.
 This will return the symbol stored in (EcsIdentifier, EcsSymbol).

 @param world The world.
 @param entity The entity.
 @return The type of the entity, NULL if the entity has no name.*/
    fn get_symbol(&self, entity: Entity) -> Option<&NullStr> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_symbol(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Lookup an entity by it's path.
 This operation is equivalent to calling:

 @code
 ecs_lookup_path_w_sep(world, 0, path, ".", NULL, true);
 @endcode

 @param world The world.
 @param path The entity path.
 @return The entity with the specified path, or 0 if no entity was found.*/
    fn lookup(&self, path: &NullStr) -> Entity {
        let world = self.as_ptr();
        let path = path.as_ptr();
        let result = unsafe { flecsys::ecs_lookup(world, path) };
        return result;
    }
    /** Lookup a child entity by name.
 Returns an entity that matches the specified name. Only looks for entities in
 the provided parent. If no parent is provided, look in the current scope (
 root if no scope is provided).

 @param world The world.
 @param parent The parent for which to lookup the child.
 @param name The entity name.
 @return The entity with the specified name, or 0 if no entity was found.*/
    fn lookup_child(&self, parent: Entity, name: &NullStr) -> Entity {
        let world = self.as_ptr();
        let name = name.as_ptr();
        let result = unsafe { flecsys::ecs_lookup_child(world, parent, name) };
        return result;
    }
    /** Lookup an entity from a path.
 Lookup an entity from a provided path, relative to the provided parent. The
 operation will use the provided separator to tokenize the path expression. If
 the provided path contains the prefix, the search will start from the root.

 If the entity is not found in the provided parent, the operation will
 continue to search in the parent of the parent, until the root is reached. If
 the entity is still not found, the lookup will search in the flecs.core
 scope. If the entity is not found there either, the function returns 0.

 @param world The world.
 @param parent The entity from which to resolve the path.
 @param path The path to resolve.
 @param sep The path separator.
 @param prefix The path prefix.
 @param recursive Recursively traverse up the tree until entity is found.
 @return The entity if found, else 0.*/
    fn lookup_path_w_sep(
        &self,
        parent: Entity,
        path: &NullStr,
        sep: &NullStr,
        prefix: &NullStr,
        recursive: bool,
    ) -> Entity {
        let world = self.as_ptr();
        let path = path.as_ptr();
        let sep = sep.as_ptr();
        let prefix = prefix.as_ptr();
        let result = unsafe {
            flecsys::ecs_lookup_path_w_sep(world, parent, path, sep, prefix, recursive)
        };
        return result;
    }
    /** Lookup an entity by its symbol name.
 This looks up an entity by symbol stored in (EcsIdentifier, EcsSymbol). The
 operation does not take into account hierarchies.

 This operation can be useful to resolve, for example, a type by its C
 identifier, which does not include the Flecs namespacing.

 @param world The world.
 @param symbol The symbol.
 @param lookup_as_path If not found as a symbol, lookup as path.
 @param recursive If looking up as path, recursively traverse up the tree.
 @return The entity if found, else 0.*/
    fn lookup_symbol(
        &self,
        symbol: &NullStr,
        lookup_as_path: bool,
        recursive: bool,
    ) -> Entity {
        let world = self.as_ptr();
        let symbol = symbol.as_ptr();
        let result = unsafe {
            flecsys::ecs_lookup_symbol(world, symbol, lookup_as_path, recursive)
        };
        return result;
    }
    /** Get a path identifier for an entity.
 This operation creates a path that contains the names of the entities from
 the specified parent to the provided entity, separated by the provided
 separator. If no parent is provided the path will be relative to the root. If
 a prefix is provided, the path will be prefixed by the prefix.

 If the parent is equal to the provided child, the operation will return an
 empty string. If a nonzero component is provided, the path will be created by
 looking for parents with that component.

 The returned path should be freed by the application.

 @param world The world.
 @param parent The entity from which to create the path.
 @param child The entity to which to create the path.
 @param sep The separator to use between path elements.
 @param prefix The initial character to use for root elements.
 @return The relative entity path.*/
    fn get_path_w_sep(
        &self,
        parent: Entity,
        child: Entity,
        sep: &NullStr,
        prefix: &NullStr,
    ) -> Option<NullString> {
        let world = self.as_ptr();
        let sep = sep.as_ptr();
        let prefix = prefix.as_ptr();
        let result = unsafe {
            flecsys::ecs_get_path_w_sep(world, parent, child, sep, prefix)
        };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer")
            .to_owned();
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Get the current scope.
 Get the scope set by ecs_set_scope(). If no scope is set, this operation will
 return 0.

 @param world The world.
 @return The current scope.*/
    fn get_scope(&self) -> Entity {
        let world = self.as_ptr();
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
    fn id_is_tag(&self, id: Id) -> bool {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_id_is_tag(world, id) };
        return result;
    }
    /** Returns whether specified id is in use.
 This operation returns whether an id is in use in the world. An id is in use
 if it has been added to one or more tables.

 @param world The world.
 @param id The id.
 @return Whether the id is in use.*/
    fn id_in_use(&self, id: Id) -> bool {
        let world = self.as_ptr();
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
    fn get_typeid(&self, id: Id) -> Entity {
        let world = self.as_ptr();
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
    fn id_is_valid(&self, id: Id) -> bool {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_id_is_valid(world, id) };
        return result;
    }
    /** Get flags associated with id.
 This operation returns the internal flags (see api_flags.h) that are
 associated with the provided id.

 @param world The world.
 @param id The id.
 @return Flags associated with the id, or 0 if the id is not in use.*/
    fn id_get_flags(&self, id: Id) -> Flags32 {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_id_get_flags(world, id) };
        return result;
    }
    /** Convert id to string.
 This operation interprets the structure of an id and converts it to a string.

 @param world The world.
 @param id The id to convert to a string.
 @return The id converted to a string.*/
    fn id_str(&self, id: Id) -> Option<NullString> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_id_str(world, id) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer")
            .to_owned();
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Get type index for id.
 This operation returns the index for an id in the table's type.

 @param world The world.
 @param table The table.
 @param id The id.
 @return The index of the id in the table type, or -1 if not found.*/
    fn table_get_type_index(&self, table: &Table, id: Id) -> i32 {
        let world = self.as_ptr();
        let table = table.as_ptr();
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
    fn table_get_column_index(&self, table: &Table, id: Id) -> i32 {
        let world = self.as_ptr();
        let table = table.as_ptr();
        let result = unsafe { flecsys::ecs_table_get_column_index(world, table, id) };
        return result;
    }
    /** Test if table has id.
 Same as ecs_table_get_type_index(world, table, id) != -1.

 @param world The world.
 @param table The table.
 @param id The id.
 @return True if the table has the id, false if the table doesn't.*/
    fn table_has_id(&self, table: &Table, id: Id) -> bool {
        let world = self.as_ptr();
        let table = table.as_ptr();
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
    fn table_get_depth(&self, table: &Table, rel: Entity) -> i32 {
        let world = self.as_ptr();
        let table = table.as_ptr();
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
    fn get_timeout(&self, tick_source: Entity) -> f32 {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_timeout(world, tick_source) };
        return result;
    }
    /** Get current interval value for the specified timer.
 This operation returns the value set by ecs_set_interval(). If the entity is
 not a timer, the operation will return 0.

 @param world The world.
 @param tick_source The timer for which to set the interval.
 @return The current interval value, or 0 if no timer is active.*/
    fn get_interval(&self, tick_source: Entity) -> f32 {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_interval(world, tick_source) };
        return result;
    }
    /** Get the current pipeline.
 This operation gets the current pipeline.

 @param world The world.
 @return The current pipeline.*/
    fn get_pipeline(&self) -> Entity {
        let world = self.as_ptr();
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
    fn get_alert_count(&self, entity: Entity, alert: Entity) -> i32 {
        let world = self.as_ptr();
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
    fn get_alert(&self, entity: Entity, alert: Entity) -> Entity {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_get_alert(world, entity, alert) };
        return result;
    }
    /** Get human readable name from entity.
 If entity does not have an explicit human readable name, this operation will
 return the entity name.

 To test if an entity has a human readable name, use:

 @code
 ecs_has_pair(world, e, ecs_id(EcsDocDescription), EcsName);
 @endcode

 Or in C++:

 @code
 e.has<flecs::doc::Description>(flecs::Name);
 @endcode

 @param world The world.
 @param entity The entity from which to get the name.
 @return The name.

 @see ecs_doc_set_name()
 @see flecs::doc::get_name()
 @see flecs::entity_view::get_doc_name()*/
    fn doc_get_name(&self, entity: Entity) -> Option<&NullStr> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_doc_get_name(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Get brief description from entity.

 @param world The world.
 @param entity The entity from which to get the description.
 @return The description.

 @see ecs_doc_set_brief()
 @see flecs::doc::get_brief()
 @see flecs::entity_view::get_doc_brief()*/
    fn doc_get_brief(&self, entity: Entity) -> Option<&NullStr> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_doc_get_brief(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Get detailed description from entity.

 @param world The world.
 @param entity The entity from which to get the description.
 @return The description.

 @see ecs_doc_set_detail()
 @see flecs::doc::get_detail()
 @see flecs::entity_view::get_doc_detail()*/
    fn doc_get_detail(&self, entity: Entity) -> Option<&NullStr> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_doc_get_detail(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Get link to external documentation from entity.

 @param world The world.
 @param entity The entity from which to get the link.
 @return The link.

 @see ecs_doc_set_link()
 @see flecs::doc::get_link()
 @see flecs::entity_view::get_doc_link()*/
    fn doc_get_link(&self, entity: Entity) -> Option<&NullStr> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_doc_get_link(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    /** Get color from entity.

 @param world The world.
 @param entity The entity from which to get the color.
 @return The color.

 @see ecs_doc_set_color()
 @see flecs::doc::get_color()
 @see flecs::entity_view::get_doc_color()*/
    fn doc_get_color(&self, entity: Entity) -> Option<&NullStr> {
        let world = self.as_ptr();
        let result = unsafe { flecsys::ecs_doc_get_color(world, entity) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
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
    fn frame_begin(&mut self, delta_time: f32) -> f32 {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_frame_begin(world, delta_time) };
        return result;
    }
    /** End frame.
 This operation must be called at the end of the frame, and always after
 ecs_frame_begin().

 @param world The world.*/
    fn frame_end(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_frame_end(world) };
        let _ = result;
    }
    /** Signal exit
 This operation signals that the application should quit. It will cause
 ecs_progress() to return false.

 @param world The world to quit.*/
    fn quit(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_quit(world) };
        let _ = result;
    }
    /** Measure frame time.
 Frame time measurements measure the total time passed in a single frame, and
 how much of that time was spent on systems and on merging.

 Frame time measurements add a small constant-time overhead to an application.
 When an application sets a target FPS, frame time measurements are enabled by
 default.

 @param world The world.
 @param enable Whether to enable or disable frame time measuring.*/
    fn measure_frame_time(&mut self, enable: bool) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_measure_frame_time(world, enable) };
        let _ = result;
    }
    /** Measure system time.
 System time measurements measure the time spent in each system.

 System time measurements add overhead to every system invocation and
 therefore have a small but measurable impact on application performance.
 System time measurements must be enabled before obtaining system statistics.

 @param world The world.
 @param enable Whether to enable or disable system time measuring.*/
    fn measure_system_time(&mut self, enable: bool) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_measure_system_time(world, enable) };
        let _ = result;
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
    fn set_target_fps(&mut self, fps: f32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_target_fps(world, fps) };
        let _ = result;
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
    fn readonly_begin(&mut self, multi_threaded: bool) -> bool {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_readonly_begin(world, multi_threaded) };
        return result;
    }
    /** End readonly mode.
 This operation ends readonly mode, and must be called after
 ecs_readonly_begin(). Operations that were deferred while the world was in
 readonly mode will be flushed.

 @param world The world*/
    fn readonly_end(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_readonly_end(world) };
        let _ = result;
    }
    /** Merge world or stage.
 When automatic merging is disabled, an application can call this
 operation on either an individual stage, or on the world which will merge
 all stages. This operation may only be called when staging is not enabled
 (either after ecs_progress() or after ecs_readonly_end()).

 This operation may be called on an already merged stage or world.

 @param world The world.*/
    fn merge(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_merge(world) };
        let _ = result;
    }
    /** Defer operations until end of frame.
 When this operation is invoked while iterating, operations inbetween the
 ecs_defer_begin() and ecs_defer_end() operations are executed at the end
 of the frame.

 This operation is thread safe.

 @param world The world.
 @return true if world changed from non-deferred mode to deferred mode.*/
    fn defer_begin(&mut self) -> bool {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_defer_begin(world) };
        return result;
    }
    /** End block of operations to defer.
 See ecs_defer_begin().

 This operation is thread safe.

 @param world The world.
 @return true if world changed from deferred mode to non-deferred mode.*/
    fn defer_end(&mut self) -> bool {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_defer_end(world) };
        return result;
    }
    /** Suspend deferring but do not flush queue.
 This operation can be used to do an undeferred operation while not flushing
 the operations in the queue.

 An application should invoke ecs_defer_resume() before ecs_defer_end() is called.
 The operation may only be called when deferring is enabled.

 @param world The world.*/
    fn defer_suspend(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_defer_suspend(world) };
        let _ = result;
    }
    /** Resume deferring.
 See ecs_defer_suspend().

 @param world The world.*/
    fn defer_resume(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_defer_resume(world) };
        let _ = result;
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
    fn set_stage_count(&mut self, stages: i32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_stage_count(world, stages) };
        let _ = result;
    }
    /** Free unmanaged stage.

 @param stage The stage to free.*/
    fn stage_free(&mut self) {
        let stage = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_stage_free(stage) };
        let _ = result;
    }
    /** Dimension the world for a specified number of entities.
 This operation will preallocate memory in the world for the specified number
 of entities. Specifying a number lower than the current number of entities in
 the world will have no effect.

 @param world The world.
 @param entity_count The number of entities to preallocate.*/
    fn dim(&mut self, entity_count: i32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_dim(world, entity_count) };
        let _ = result;
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
    fn set_entity_range(&mut self, id_start: Entity, id_end: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_entity_range(world, id_start, id_end) };
        let _ = result;
    }
    /** Enable/disable range limits.
 When an application is both a receiver of range-limited entities and a
 producer of range-limited entities, range checking needs to be temporarily
 disabled when inserting received entities. Range checking is disabled on a
 stage, so setting this value is thread safe.

 @param world The world.
 @param enable True if range checking should be enabled, false to disable.
 @return The previous value.*/
    fn enable_range_check(&mut self, enable: bool) -> bool {
        let world = self.as_ptr_mut();
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
    fn run_aperiodic(&mut self, flags: Flags32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_run_aperiodic(world, flags) };
        let _ = result;
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
    fn delete_empty_tables(
        &mut self,
        id: Id,
        clear_generation: u16,
        delete_generation: u16,
        min_id_count: i32,
        time_budget_seconds: f64,
    ) -> i32 {
        let world = self.as_ptr_mut();
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
    fn new(&mut self) -> Entity {
        let world = self.as_ptr_mut();
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
    fn new_low_id(&mut self) -> Entity {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_new_low_id(world) };
        return result;
    }
    /** Create new entity with (component) id.
 This operation creates a new entity with an optional (component) id. When 0
 is passed to the id parameter, no component is added to the new entity.

 @param world The world.
 @param id The component id to initialize the new entity with.
 @return The new entity.*/
    fn new_w_id(&mut self, id: Id) -> Entity {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_new_w_id(world, id) };
        return result;
    }
    /** Create new entity in table.
 This operation creates a new entity in the specified table.

 @param world The world.
 @param table The table to which to add the new entity.
 @return The new entity.*/
    fn new_w_table(&mut self, table: &mut Table) -> Entity {
        let world = self.as_ptr_mut();
        let table = table.as_ptr_mut();
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
    fn clone(&mut self, dst: Entity, src: Entity, copy_value: bool) -> Entity {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_clone(world, dst, src, copy_value) };
        return result;
    }
    /** Delete an entity.
 This operation will delete an entity and all of its components. The entity id
 will be made available for recycling. If the entity passed to ecs_delete() is
 not alive, the operation will have no side effects.

 @param world The world.
 @param entity The entity.*/
    fn delete(&mut self, entity: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_delete(world, entity) };
        let _ = result;
    }
    /** Delete all entities with the specified id.
 This will delete all entities (tables) that have the specified id. The id
 may be a wildcard and/or a pair.

 @param world The world.
 @param id The id.*/
    fn delete_with(&mut self, id: Id) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_delete_with(world, id) };
        let _ = result;
    }
    /** Add a (component) id to an entity.
 This operation adds a single (component) id to an entity. If the entity
 already has the id, this operation will have no side effects.

 @param world The world.
 @param entity The entity.
 @param id The id to add.*/
    fn add_id(&mut self, entity: Entity, id: Id) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_add_id(world, entity, id) };
        let _ = result;
    }
    /** Remove a (component) id from an entity.
 This operation removes a single (component) id to an entity. If the entity
 does not have the id, this operation will have no side effects.

 @param world The world.
 @param entity The entity.
 @param id The id to remove.*/
    fn remove_id(&mut self, entity: Entity, id: Id) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_remove_id(world, entity, id) };
        let _ = result;
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
    fn auto_override_id(&mut self, entity: Entity, id: Id) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_auto_override_id(world, entity, id) };
        let _ = result;
    }
    /** Clear all components.
 This operation will remove all components from an entity.

 @param world The world.
 @param entity The entity.*/
    fn clear(&mut self, entity: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_clear(world, entity) };
        let _ = result;
    }
    /** Remove all instances of the specified (component) id.
 This will remove the specified id from all entities (tables). The id may be
 a wildcard and/or a pair.

 @param world The world.
 @param id The id.*/
    fn remove_all(&mut self, id: Id) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_remove_all(world, id) };
        let _ = result;
    }
    /** Set current with id.
 New entities are automatically created with the specified id.

 @param world The world.
 @param id The id.
 @return The previous id.*/
    fn set_with(&mut self, id: Id) -> Entity {
        let world = self.as_ptr_mut();
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
    fn enable(&mut self, entity: Entity, enabled: bool) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_enable(world, entity, enabled) };
        let _ = result;
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
    fn enable_id(&mut self, entity: Entity, id: Id, enable: bool) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_enable_id(world, entity, id, enable) };
        let _ = result;
    }
    /** Signal that a component has been modified.
 This operation is usually used after modifying a component value obtained by
 ecs_ensure_id(). The operation will mark the component as dirty, and invoke
 OnSet observers and hooks.

 @param world The world.
 @param entity The entity.
 @param id The id of the component that was modified.*/
    fn modified_id(&mut self, entity: Entity, id: Id) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_modified_id(world, entity, id) };
        let _ = result;
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
    fn make_alive(&mut self, entity: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_make_alive(world, entity) };
        let _ = result;
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
    fn make_alive_id(&mut self, id: Id) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_make_alive_id(world, id) };
        let _ = result;
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
    fn set_generation(&mut self, entity: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_generation(world, entity) };
        let _ = result;
    }
    /** Set the name of an entity.
 This will set or overwrite the name of an entity. If no entity is provided,
 a new entity will be created.

 The name is stored in (EcsIdentifier, EcsName).

 @param world The world.
 @param entity The entity.
 @param name The name.
 @return The provided entity, or a new entity if 0 was provided.*/
    fn set_name(&mut self, entity: Entity, name: &NullStr) -> Entity {
        let world = self.as_ptr_mut();
        let name = name.as_ptr();
        let result = unsafe { flecsys::ecs_set_name(world, entity, name) };
        return result;
    }
    /** Set the symbol of an entity.
 This will set or overwrite the symbol of an entity. If no entity is provided,
 a new entity will be created.

 The symbol is stored in (EcsIdentifier, EcsSymbol).

 @param world The world.
 @param entity The entity.
 @param symbol The symbol.
 @return The provided entity, or a new entity if 0 was provided.*/
    fn set_symbol(&mut self, entity: Entity, symbol: &NullStr) -> Entity {
        let world = self.as_ptr_mut();
        let symbol = symbol.as_ptr();
        let result = unsafe { flecsys::ecs_set_symbol(world, entity, symbol) };
        return result;
    }
    /** Set alias for entity.
 An entity can be looked up using its alias from the root scope without
 providing the fully qualified name if its parent. An entity can only have
 a single alias.

 The symbol is stored in (EcsIdentifier, EcsAlias).

 @param world The world.
 @param entity The entity.
 @param alias The alias.*/
    fn set_alias(&mut self, entity: Entity, alias: &NullStr) {
        let world = self.as_ptr_mut();
        let alias = alias.as_ptr();
        let result = unsafe { flecsys::ecs_set_alias(world, entity, alias) };
        let _ = result;
    }
    /** Find or create entity from path.
 This operation will find or create an entity from a path, and will create any
 intermediate entities if required. If the entity already exists, no entities
 will be created.

 If the path starts with the prefix, then the entity will be created from the
 root scope.

 @param world The world.
 @param parent The entity relative to which the entity should be created.
 @param path The path to create the entity for.
 @param sep The separator used in the path.
 @param prefix The prefix used in the path.
 @return The entity.*/
    fn new_from_path_w_sep(
        &mut self,
        parent: Entity,
        path: &NullStr,
        sep: &NullStr,
        prefix: &NullStr,
    ) -> Entity {
        let world = self.as_ptr_mut();
        let path = path.as_ptr();
        let sep = sep.as_ptr();
        let prefix = prefix.as_ptr();
        let result = unsafe {
            flecsys::ecs_new_from_path_w_sep(world, parent, path, sep, prefix)
        };
        return result;
    }
    /** Add specified path to entity.
 This operation is similar to ecs_new_from_path(), but will instead add the path
 to an existing entity.

 If an entity already exists for the path, it will be returned instead.

 @param world The world.
 @param entity The entity to which to add the path.
 @param parent The entity relative to which the entity should be created.
 @param path The path to create the entity for.
 @param sep The separator used in the path.
 @param prefix The prefix used in the path.
 @return The entity.*/
    fn add_path_w_sep(
        &mut self,
        entity: Entity,
        parent: Entity,
        path: &NullStr,
        sep: &NullStr,
        prefix: &NullStr,
    ) -> Entity {
        let world = self.as_ptr_mut();
        let path = path.as_ptr();
        let sep = sep.as_ptr();
        let prefix = prefix.as_ptr();
        let result = unsafe {
            flecsys::ecs_add_path_w_sep(world, entity, parent, path, sep, prefix)
        };
        return result;
    }
    /** Set the current scope.
 This operation sets the scope of the current stage to the provided entity.
 As a result new entities will be created in this scope, and lookups will be
 relative to the provided scope.

 It is considered good practice to restore the scope to the old value.

 @param world The world.
 @param scope The entity to use as scope.
 @return The previous scope.*/
    fn set_scope(&mut self, scope: Entity) -> Entity {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_scope(world, scope) };
        return result;
    }
    /** Set a name prefix for newly created entities.
 This is a utility that lets C modules use prefixed names for C types and
 C functions, while using names for the entity names that do not have the
 prefix. The name prefix is currently only used by ECS_COMPONENT.

 @param world The world.
 @param prefix The name prefix to use.
 @return The previous prefix.*/
    fn set_name_prefix(&mut self, prefix: &NullStr) -> Option<&NullStr> {
        let world = self.as_ptr_mut();
        let prefix = prefix.as_ptr();
        let result = unsafe { flecsys::ecs_set_name_prefix(world, prefix) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
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
    fn table_lock(&mut self, table: &mut Table) {
        let world = self.as_ptr_mut();
        let table = table.as_ptr_mut();
        let result = unsafe { flecsys::ecs_table_lock(world, table) };
        let _ = result;
    }
    /** Unlock a table.
 Must be called after calling ecs_table_lock().

 @param world The world.
 @param table The table to unlock.*/
    fn table_unlock(&mut self, table: &mut Table) {
        let world = self.as_ptr_mut();
        let table = table.as_ptr_mut();
        let result = unsafe { flecsys::ecs_table_unlock(world, table) };
        let _ = result;
    }
    /** Swaps two elements inside the table. This is useful for implementing custom
 table sorting algorithms.
 @param world The world
 @param table The table to swap elements in
 @param row_1 Table element to swap with row_2
 @param row_2 Table element to swap with row_1*/
    fn table_swap_rows(&mut self, table: &mut Table, row_1: i32, row_2: i32) {
        let world = self.as_ptr_mut();
        let table = table.as_ptr_mut();
        let result = unsafe { flecsys::ecs_table_swap_rows(world, table, row_1, row_2) };
        let _ = result;
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
    fn set_timeout(&mut self, tick_source: Entity, timeout: f32) -> Entity {
        let world = self.as_ptr_mut();
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
    fn set_interval(&mut self, tick_source: Entity, interval: f32) -> Entity {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_interval(world, tick_source, interval) };
        return result;
    }
    /** Start timer.
 This operation resets the timer and starts it with the specified timeout.

 @param world The world.
 @param tick_source The timer to start.*/
    fn start_timer(&mut self, tick_source: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_start_timer(world, tick_source) };
        let _ = result;
    }
    /** Stop timer
 This operation stops a timer from triggering.

 @param world The world.
 @param tick_source The timer to stop.*/
    fn stop_timer(&mut self, tick_source: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_stop_timer(world, tick_source) };
        let _ = result;
    }
    /** Reset time value of timer to 0.
 This operation resets the timer value to 0.

 @param world The world.
 @param tick_source The timer to reset.*/
    fn reset_timer(&mut self, tick_source: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_reset_timer(world, tick_source) };
        let _ = result;
    }
    /** Enable randomizing initial time value of timers.
 Initializes timers with a random time value, which can improve scheduling as
 systems/timers for the same interval don't all happen on the same tick.

 @param world The world.*/
    fn randomize_timers(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_randomize_timers(world) };
        let _ = result;
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
    fn set_rate(&mut self, tick_source: Entity, rate: i32, source: Entity) -> Entity {
        let world = self.as_ptr_mut();
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
    fn set_tick_source(&mut self, system: Entity, tick_source: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_tick_source(world, system, tick_source) };
        let _ = result;
    }
    /** Set a custom pipeline.
 This operation sets the pipeline to run when ecs_progress() is invoked.

 @param world The world.
 @param pipeline The pipeline to set.*/
    fn set_pipeline(&mut self, pipeline: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_pipeline(world, pipeline) };
        let _ = result;
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
    fn progress(&mut self, delta_time: f32) -> bool {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_progress(world, delta_time) };
        return result;
    }
    /** Set time scale.
 Increase or decrease simulation speed by the provided multiplier.

 @param world The world.
 @param scale The scale to apply (default = 1).*/
    fn set_time_scale(&mut self, scale: f32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_time_scale(world, scale) };
        let _ = result;
    }
    /** Reset world clock.
 Reset the clock that keeps track of the total time passed in the simulation.

 @param world The world.*/
    fn reset_clock(&mut self) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_reset_clock(world) };
        let _ = result;
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
    fn run_pipeline(&mut self, pipeline: Entity, delta_time: f32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_run_pipeline(world, pipeline, delta_time) };
        let _ = result;
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
    fn set_threads(&mut self, threads: i32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_threads(world, threads) };
        let _ = result;
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
    fn set_task_threads(&mut self, task_threads: i32) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_set_task_threads(world, task_threads) };
        let _ = result;
    }
    /** Returns true if task thread use have been requested.

 @param world The world.
 @result Whether the world is using task threads.*/
    fn using_task_threads(&mut self) -> bool {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_using_task_threads(world) };
        return result;
    }
    /** Parse script.
 This parses a script and instantiates the entities in the world.
 This operation is the equivalent to doing:

 @code
 ecs_script_t *script = ecs_script_parse(world, name, code);
 ecs_script_eval(script);
 ecs_script_free(script);
 @endcode

 @param world The world.
 @param name The script name (typically the file).
 @param code The script.
 @return Zero if success, non-zero otherwise.*/
    fn script_run(&mut self, name: &NullStr, code: &NullStr) -> i32 {
        let world = self.as_ptr_mut();
        let name = name.as_ptr();
        let code = code.as_ptr();
        let result = unsafe { flecsys::ecs_script_run(world, name, code) };
        return result;
    }
    /** Parse script file.
 This parses a script file and instantiates the entities in the world. This
 operation is equivalent to loading the file contents and passing it to
 ecs_script_run().

 @param world The world.
 @param filename The script file name.
 @return Zero if success, non-zero if failed.*/
    fn script_run_file(&mut self, filename: &NullStr) -> i32 {
        let world = self.as_ptr_mut();
        let filename = filename.as_ptr();
        let result = unsafe { flecsys::ecs_script_run_file(world, filename) };
        return result;
    }
    /** Update script with new code.

 @param world The world.
 @param script The script entity.
 @param instance An template instance (optional).
 @param code The script code.*/
    fn script_update(
        &mut self,
        script: Entity,
        instance: Entity,
        code: &NullStr,
    ) -> i32 {
        let world = self.as_ptr_mut();
        let code = code.as_ptr();
        let result = unsafe {
            flecsys::ecs_script_update(world, script, instance, code)
        };
        return result;
    }
    /** Clear all entities associated with script.

 @param world The world.
 @param script The script entity.
 @param instance The script instance.*/
    fn script_clear(&mut self, script: Entity, instance: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_script_clear(world, script, instance) };
        let _ = result;
    }
    /** Add human-readable name to entity.
 Contrary to entity names, human readable names do not have to be unique and
 can contain special characters used in the query language like '*'.

 @param world The world.
 @param entity The entity to which to add the name.
 @param name The name to add.

 @see ecs_doc_get_name()
 @see flecs::doc::set_name()
 @see flecs::entity_builder::set_doc_name()*/
    fn doc_set_name(&mut self, entity: Entity, name: &NullStr) {
        let world = self.as_ptr_mut();
        let name = name.as_ptr();
        let result = unsafe { flecsys::ecs_doc_set_name(world, entity, name) };
        let _ = result;
    }
    /** Add brief description to entity.

 @param world The world.
 @param entity The entity to which to add the description.
 @param description The description to add.

 @see ecs_doc_get_brief()
 @see flecs::doc::set_brief()
 @see flecs::entity_builder::set_doc_brief()*/
    fn doc_set_brief(&mut self, entity: Entity, description: &NullStr) {
        let world = self.as_ptr_mut();
        let description = description.as_ptr();
        let result = unsafe { flecsys::ecs_doc_set_brief(world, entity, description) };
        let _ = result;
    }
    /** Add detailed description to entity.

 @param world The world.
 @param entity The entity to which to add the description.
 @param description The description to add.

 @see ecs_doc_get_detail()
 @see flecs::doc::set_detail()
 @see flecs::entity_builder::set_doc_detail()*/
    fn doc_set_detail(&mut self, entity: Entity, description: &NullStr) {
        let world = self.as_ptr_mut();
        let description = description.as_ptr();
        let result = unsafe { flecsys::ecs_doc_set_detail(world, entity, description) };
        let _ = result;
    }
    /** Add link to external documentation to entity.

 @param world The world.
 @param entity The entity to which to add the link.
 @param link The link to add.

 @see ecs_doc_get_link()
 @see flecs::doc::set_link()
 @see flecs::entity_builder::set_doc_link()*/
    fn doc_set_link(&mut self, entity: Entity, link: &NullStr) {
        let world = self.as_ptr_mut();
        let link = link.as_ptr();
        let result = unsafe { flecsys::ecs_doc_set_link(world, entity, link) };
        let _ = result;
    }
    /** Add color to entity.
 UIs can use color as hint to improve visualizing entities.

 @param world The world.
 @param entity The entity to which to add the link.
 @param color The color to add.

 @see ecs_doc_get_color()
 @see flecs::doc::set_color()
 @see flecs::entity_builder::set_doc_color()*/
    fn doc_set_color(&mut self, entity: Entity, color: &NullStr) {
        let world = self.as_ptr_mut();
        let color = color.as_ptr();
        let result = unsafe { flecsys::ecs_doc_set_color(world, entity, color) };
        let _ = result;
    }
    /// Populate meta information from type descriptor.
    fn meta_from_desc(
        &mut self,
        component: Entity,
        kind: flecsys::ecs_type_kind_t,
        desc: &NullStr,
    ) -> i32 {
        let world = self.as_ptr_mut();
        let desc = desc.as_ptr();
        let result = unsafe {
            flecsys::ecs_meta_from_desc(world, component, kind, desc)
        };
        return result;
    }
    /** Import a module from a library.
 Similar to ecs_import(), except that this operation will attempt to load the
 module from a dynamic library.

 A library may contain multiple modules, which is why both a library name and
 a module name need to be provided. If only a library name is provided, the
 library name will be reused for the module name.

 The library will be looked up using a canonical name, which is in the same
 form as a module, like `flecs.components.transform`. To transform this
 identifier to a platform specific library name, the operation relies on the
 module_to_dl callback of the os_api which the application has to override if
 the default does not yield the correct library name.

 @param world The world.
 @param library_name The name of the library to load.
 @param module_name The name of the module to load.*/
    fn import_from_library(
        &mut self,
        library_name: &NullStr,
        module_name: &NullStr,
    ) -> Entity {
        let world = self.as_ptr_mut();
        let library_name = library_name.as_ptr();
        let module_name = module_name.as_ptr();
        let result = unsafe {
            flecsys::ecs_import_from_library(world, library_name, module_name)
        };
        return result;
    }
    fn cpp_trim_module(&mut self, type_name: &NullStr) -> Option<&NullStr> {
        let world = self.as_ptr_mut();
        let type_name = type_name.as_ptr();
        let result = unsafe { flecsys::ecs_cpp_trim_module(world, type_name) };
        if result.is_null() {
            return None;
        }
        let nstr = unsafe { NullStr::from_ptr(result) }
            .expect("failed to create null str from pointer");
        unsafe { (flecsys::ecs_os_get_api().free_.unwrap())(result as *mut _) };
        Some(nstr)
    }
    fn cpp_component_validate(
        &mut self,
        id: Entity,
        name: &NullStr,
        symbol: &NullStr,
        size: usize,
        alignment: usize,
        implicit_name: bool,
    ) {
        let world = self.as_ptr_mut();
        let name = name.as_ptr();
        let symbol = symbol.as_ptr();
        let result = unsafe {
            flecsys::ecs_cpp_component_validate(
                world,
                id,
                name,
                symbol,
                size,
                alignment,
                implicit_name,
            )
        };
        let _ = result;
    }
    fn cpp_enum_init(&mut self, id: Entity) {
        let world = self.as_ptr_mut();
        let result = unsafe { flecsys::ecs_cpp_enum_init(world, id) };
        let _ = result;
    }
    fn cpp_enum_constant_register(
        &mut self,
        parent: Entity,
        id: Entity,
        name: &NullStr,
        value: i32,
    ) -> Entity {
        let world = self.as_ptr_mut();
        let name = name.as_ptr();
        let result = unsafe {
            flecsys::ecs_cpp_enum_constant_register(world, parent, id, name, value)
        };
        return result;
    }
}
