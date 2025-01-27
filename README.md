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

## Parenting

To start interacting with parents, you need to add an event, as well as the main system for CUD. The type `T` is a component that you want to add into your application while `U` is the unique identifier type.

```rust
.add_event::<ParentEvent<T, U>>()
.add_systems(Update, cud_parent_component::<T, U>)
```

## Child-ing

To start interacting with childrens, you need to add an event and the main system. The type `T` is the parent type, `U` is the child type, and `V` is the unique identifier.

```rust
.add_event::<ChildEvent<U, V>>()
.add_systems(Update, cud_child_component::<T, U, V>)
```

> [!TIP]
> You can add multiple events and systems to layer as many generation as you want!

## History / Lineage
One challenge that was encountered when using the parenting system with an event based approach is the loss of the event's status.

It is difficult to know whether if the parent or child is successfully added, removed, or updated.

With that, the plugin use the `Resource` feature of bevy to store the `History` of procreation in a `Lineage`.