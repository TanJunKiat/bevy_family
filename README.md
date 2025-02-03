# bevy_family
A bevy package that contains services for parent-child components

# Motivation
This package aims to solve CUD (creating, updating, and deleting) components that have a parent-child relationship with one another.

The assumption is that all such components contains a unique identifier (think of it like a name of a person, or a identification number!).

The unique identifier can be in any form (a string for intuitiveness or a UUID for robustness), but can only exist once per generation.

This package still abies to the rule of 1 child only can have 1 parent, and 1 parent can have multiple children.

# Usage

To initialise the application, the `FamilyPlugin` must be added and the unique identifier, `T` must be declared.

```rust
.add_plugins(FamilyPlugin::<T>::default())
```

## Unique identifier
This packages uses a variable type as the identifier to distinguish between different components, rather than using the components themselves.

It is like two person can have the same name, but has different identification number.

The code is written such a way that the user can decide what data type to use as your identifier.

The examples mainly uses `String` for simplicity, but there is also an example that uses `uuid` for the identifier.

You can also create your own `struct` for the identifier to hold even more information that you might want your own systems to utilize. This library minimumly requires `PartialEq` to be implement for the comparsion to be done in the generic systems.

The unique identifier is only applicable for components of the same parent generation (two parents of the same type cannot have the same identifier), or from the same parent (two child of the same parent cannot have the same identifier; two child of different parents can have the same identifier). The identifier also only applies to one type of component (i.e. two different components can potentially have the same identifier).

These rules are made such for better scalability and control at the user-level.

## Parenting

To start interacting with parents, you need to add an event, as well as the main system for CUD. The type `T` is a component that you want to add into your application while `U` is the unique identifier type.

```rust
.add_event::<ParentEvent<T, U>>()
.add_systems(Update, cud_parent_component::<T, U>)
```

To add a parent, you just need to call an event in your system.
```rust
mut parent_event_writer: EventWriter<ParentEvent<Building, String>>
...
// to create
parent_event_writer.send(ParentEvent::create("Building".into(), Building));

// to update
parent_event_writer.send(ParentEvent::update("Building".into(), Building));

// to delete
parent_event_writer.send(ParentEvent::delete("Building".into(), Building));
```

## Child-ing

To start interacting with childrens, you need to add an event and the main system. The type `T` is the parent type, `U` is the child type, and `V` is the unique identifier.

```rust
.add_event::<ChildEvent<U, V>>()
.add_systems(Update, cud_child_component::<T, U, V>)
```

> [!TIP]
> You can add multiple events and systems to layer as many generation as you want!
```rust
mut child_event_writer: EventWriter<ChildEvent<Level, String>>
...
// to create a child
child_event_writer.send(ChildEvent::create("Building".into(), "Level".into(), Level));
// to update a child
child_event_writer.send(ChildEvent::update("Building".into(), "Level".into(), Level));
// to delete a child
child_event_writer.send(ChildEvent::delete("Building".into(), "Level".into(), Level));
```
Similar to adding a parent, to add a child to a parent, you just need to write to a bevy event.

## History / Lineage
One challenge that was encountered when using the parenting system with an event based approach is the loss of the event's status.

It is difficult to know whether if the parent or child is successfully added, removed, or updated.

With that, the plugin use the `Resource` feature of bevy to store the `History` of procreation in a `Lineage`.


# Features
- [ ] A query pipeline that allows checking of existing parent and children that are in the application.
- [ ] Able to add multiple components to a parent entity