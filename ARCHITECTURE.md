# Architecture


## Styling

Vizia uses a simple ECS (Entity Component System) for styling.

Each view has a unique `Entity` ID assigned to it on creation. This ID is used to store and retrieve style properties (components) from the `Context`. A series of systems operate on the style properties to perform 

### Storage
At the heart of the styling system are the storages which hold the style properties for each view. There are two types of storage:

1. `StyleSet`
2. `AnimatableSet`

These storages are based on sparse-sets. A typical sparse set contains two arrays, a sparse array and a dense array. The sparse array contains indicies into the dense array. The sparse array itself is indexed by the entity ID.

The `StyleSet` used in Vizia contains two sparse sets, one for `inline` data and one for `shared`. 

The inline sparse set contains style data which are declared in Rust code on a view. This is usually a one-to-one relationship with a particular entity, unless the property can be inheritted.

The shared sparse set contains style data which are declared in CSS. This can be a many-to-one relationship where multiple entities can reference the same style property.

For the `inline` 

This allows multiple entities to point to the same shared data, or the same inline or shared data if the property is inheritted.

### Parsing
The job of the parser is to translate style rules declared in CSS into properties which are stored in `Context`. The parser generates a list of rules, where each rule can have one or more selectors and some number of properties. Each rule is assigned an ID and is used to store the properties in the shared data array of the storage. 