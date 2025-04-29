// =========================================================================
/*
 * Copyright (C) 2019 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =========================================================================
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_log::prelude::*;
use bevy_time::prelude::*;

use std::marker::PhantomData;

/// Main Plugin for the Family Plugin
///
/// This plugin is used to initialize the Lineage resource
#[derive(Default)]
pub struct FamilyPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T> Plugin for FamilyPlugin<T>
where
    T: Default + Send + Sync + 'static,
{
    /// Build the plugin
    fn build(&self, app: &mut App) {
        app.init_resource::<Lineage<T>>();
    }
}

pub trait BiologicalTrait {
    fn get_lifetime(&self) -> std::time::Duration;
}

/// Action to interact with the entities.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Create,
    CreateOrModify,
    Update,
    Delete,
    Clear,
}

/// Unique identifier for the entities.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Identifier<T>(pub T);

/// A component that represents the entity's lifetime.
#[derive(Component, Default)]
pub struct BiologicalClock {
    pub lifetime: bevy_time::Stopwatch,
}

/// Event that is used to create, update and delete parent entities
#[derive(Event)]
pub struct CudEvent<U: Bundle, T> {
    action: Action,
    parent_identifier: Option<Identifier<T>>,
    self_identifier: Identifier<T>,
    bundle: U,
}

impl<U, T> CudEvent<U, T>
where
    T: Clone,
    U: Bundle + Clone,
{
    pub fn get_parent_identifier(&self) -> &Option<Identifier<T>> {
        &self.parent_identifier
    }

    pub fn get_self_identifier(&self) -> &Identifier<T> {
        &self.self_identifier
    }

    pub fn get_action(&self) -> &Action {
        &self.action
    }

    pub fn get_bundle(&self) -> impl Bundle {
        self.bundle.clone()
    }

    pub fn to_history(&self, result: Result<(), ()>) -> History<T> {
        match &self.parent_identifier {
            Some(parent_identifier) => History::new_child_history(self.action.clone(), parent_identifier.clone(), self.self_identifier.clone(), result),
            None => History::new_parent_history(self.action.clone(), self.self_identifier.clone(), result),
        }
    }

    pub fn create_parent(self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Create,
            parent_identifier: None,
            self_identifier: Identifier(self_identifier),
            bundle,
        }
    }
    pub fn create_or_modify_parent(self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::CreateOrModify,
            parent_identifier: None,
            self_identifier: Identifier(self_identifier),
            bundle,
        }
    }
    pub fn update_parent(self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Update,
            parent_identifier: None,
            self_identifier: Identifier(self_identifier),
            bundle,
        }
    }
    pub fn delete_parent(self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Delete,
            parent_identifier: None,
            self_identifier: Identifier(self_identifier),
            bundle,
        }
    }
    pub fn clear_parent(self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Clear,
            parent_identifier: None,
            self_identifier: Identifier(self_identifier),
            bundle,
        }
    }

    pub fn create_child(parent_identifier: T, self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Create,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Some(Identifier(parent_identifier)),
            bundle,
        }
    }
    pub fn create_or_modify_child(parent_identifier: T, self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::CreateOrModify,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Some(Identifier(parent_identifier)),
            bundle,
        }
    }
    pub fn update_child(parent_identifier: T, self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Update,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Some(Identifier(parent_identifier)),
            bundle,
        }
    }
    pub fn delete_child(parent_identifier: T, self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Delete,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Some(Identifier(parent_identifier)),
            bundle,
        }
    }
    pub fn clear_child(parent_identifier: T, self_identifier: T, bundle: U) -> Self {
        Self {
            action: Action::Clear,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Some(Identifier(parent_identifier)),
            bundle,
        }
    }
}

/// History of the action that has been performed
#[derive(PartialEq)]
pub struct History<T> {
    pub action: Action,
    pub parent_identifier: Identifier<T>,
    pub child_identifier: Option<Identifier<T>>,
    pub result: Result<(), ()>,
}

/// Lineage of the actions that have been performed
#[derive(Resource, Default)]
pub struct Lineage<T> {
    pub histories: Vec<History<T>>,
}

impl<T> Lineage<T>
where
    T: Clone + PartialEq,
{
    /// Add a history to the lineage.
    pub fn add_history(&mut self, history: History<T>) {
        self.histories.push(history);
    }

    /// Remove a history from the lineage.
    pub fn remove_history(&mut self, history: History<T>) {
        self.histories.retain(|h| h != &history);
    }

    /// Get the history by the parent identifier.
    pub fn get_histories_by_parent_identifier(&self, parent_identifier: &Identifier<T>) -> Vec<&History<T>> {
        let mut histories = Vec::new();
        for history in &self.histories {
            if &history.parent_identifier == parent_identifier {
                histories.push(history);
            }
        }
        return histories;
    }

    /// Get the history by the child identifier.
    pub fn get_histories_by_child_identifier(&self, child_identifier: &Identifier<T>) -> Vec<&History<T>> {
        let mut histories = Vec::new();
        for history in &self.histories {
            if let Some(identifier) = &history.child_identifier {
                if identifier == child_identifier {
                    histories.push(history);
                }
            }
        }
        return histories;
    }

    /// Get the result by the parent identifier.
    pub fn get_result_from_parent_identifier(&self, parent_identifier: &Identifier<T>) -> Result<(), ()> {
        for history in &self.histories {
            if &history.parent_identifier == parent_identifier {
                return history.result;
            }
        }
        return Err(());
    }

    /// Get the result by the child identifier.
    pub fn get_result_from_child_identifier(&self, child_identifier: &Identifier<T>) -> Result<(), ()> {
        for history in &self.histories {
            if let Some(identifier) = &history.child_identifier {
                if identifier == child_identifier {
                    return history.result;
                }
            }
        }
        return Err(());
    }

    /// Clear the history.
    pub fn clear_history(&mut self) {
        self.histories.clear();
    }

    /// Clear the parent histories.
    pub fn clear_parent_history(&mut self, parent_identifier: &Identifier<T>) {
        self.histories.retain(|h| &h.parent_identifier != parent_identifier);
    }

    /// Clear the child histories.
    pub fn clear_child_history(&mut self, child_identifier: &Identifier<T>) {
        self.histories.retain(|h| h.child_identifier != Some(child_identifier.clone()));
    }

    /// Pop the history.
    pub fn pop(&mut self) -> Option<History<T>> {
        return self.histories.pop();
    }
}

impl<T> History<T> {
    /// Create a new parent history.
    pub fn new_parent_history(action: Action, parent_identifier: Identifier<T>, result: Result<(), ()>) -> Self {
        Self {
            action,
            parent_identifier,
            child_identifier: None,
            result,
        }
    }

    /// Create a new child history.
    pub fn new_child_history(action: Action, parent_identifier: Identifier<T>, child_identifier: Identifier<T>, result: Result<(), ()>) -> Self {
        Self {
            action,
            parent_identifier,
            child_identifier: Some(child_identifier),
            result,
        }
    }
}

/// Get the entity by the identifier.
pub fn get_entity_by_identifier<U>(queries: &Query<(Entity, &Identifier<U>), With<Identifier<U>>>, identifier: &Identifier<U>) -> Option<Entity>
where
    U: PartialEq + Send + Sync + 'static,
{
    for (entity, id) in queries.iter() {
        if id == identifier {
            return Some(entity);
        }
    }
    return None;
}

pub fn cud_bundle<U, V>(mut commands: Commands, queries: Query<(Entity, &Identifier<V>), With<Identifier<V>>>, mut events: EventReader<CudEvent<U, V>>, mut lineage: ResMut<Lineage<V>>)
where
    U: Bundle + Clone,
    V: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static,
{
    for event in events.read() {
        let action = event.get_action();
        let self_identifier = event.get_self_identifier();
        match event.get_parent_identifier() {
            // if there is a parent identifier
            Some(parent_identifier) => {
                match get_entity_by_identifier(&queries, parent_identifier) {
                    // if parent is found
                    Some(parent_entity) => {
                        match get_entity_by_identifier(&queries, self_identifier) {
                            // if child is found
                            Some(child_entity) => match action {
                                Action::CreateOrModify | Action::Update => {
                                    commands.entity(child_entity).despawn_recursive();
                                    let child = commands.spawn((event.get_bundle(), self_identifier.clone(), BiologicalClock::default())).id();
                                    commands.entity(parent_entity).add_child(child);
                                    debug!("Child entity {:?} under parent entity {:?} is updated.", self_identifier, parent_entity);
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                Action::Delete => {
                                    commands.entity(child_entity).despawn_recursive();
                                    debug!("Child entity {:?} under parent entity {:?} is deleted.", self_identifier, parent_entity);
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                Action::Clear => {
                                    commands.entity(child_entity).despawn_descendants();
                                    debug!("Child entity {:?}'s childrens are cleared.", self_identifier);
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                _ => {
                                    warn!("Parent {:?} already consist of child entity {:?}.", parent_identifier, self_identifier);
                                    lineage.add_history(event.to_history(Err(())));
                                }
                            },
                            // if child not found
                            None => match action {
                                Action::Create | Action::CreateOrModify => {
                                    let child = commands.spawn((event.get_bundle(), self_identifier.clone(), BiologicalClock::default())).id();
                                    commands.entity(parent_entity).add_child(child);
                                    debug!("Child entity {:?} created under parent entity {:?}.", self_identifier, parent_entity,);
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                _ => {
                                    warn!("Parent entity {:?} does not have child entity {:?}.", parent_entity, self_identifier);
                                    lineage.add_history(event.to_history(Err(())));
                                }
                            },
                        }
                    }
                    // if parent entity not found
                    None => {
                        warn!("Parent entity {:?} does not exist.", parent_identifier);
                        lineage.add_history(event.to_history(Err(())));
                        continue;
                    }
                }
            }
            // if there is no parent identifier, spawn as a parent entity
            None => {
                match get_entity_by_identifier(&queries, self_identifier) {
                    // if identifier is found
                    Some(entity) => match action {
                        Action::CreateOrModify | Action::Update => {
                            debug!("Parent entity {:?} updated.", self_identifier);
                            commands.entity(entity).despawn_recursive();
                            commands.spawn((event.get_bundle(), self_identifier.clone(), BiologicalClock::default()));
                            lineage.add_history(event.to_history(Ok(())));
                        }
                        Action::Delete => {
                            commands.entity(entity).despawn_recursive();
                            debug!("Parent entity {:?} deleted.", self_identifier);
                            lineage.add_history(event.to_history(Ok(())));
                        }
                        Action::Clear => {
                            commands.entity(entity).despawn_descendants();
                            debug!("Parent entity's {:?} childrens cleared.", self_identifier);
                            lineage.add_history(event.to_history(Ok(())));
                        }
                        _ => {
                            warn!("Parent entity {:?} already exists.", self_identifier);
                            lineage.add_history(event.to_history(Err(())));
                        }
                    },
                    // if identifier not found
                    None => match action {
                        Action::Create | Action::CreateOrModify => {
                            debug!("Parent entity {:?} created.", self_identifier);
                            commands.spawn((event.get_bundle(), self_identifier.clone(), BiologicalClock::default()));
                            lineage.add_history(event.to_history(Ok(())));
                        }
                        _ => {
                            warn!("Parent entity {:?} does not exist.", self_identifier);
                            lineage.add_history(event.to_history(Err(())));
                        }
                    },
                }
            }
        }
    }
}

/// Acts like a garbage collector to remove entities that have exceeded their own lifetime
pub fn refresh_by_own_lifetime<T>(mut commands: Commands, time: Res<Time>, mut queries: Query<(Entity, &mut BiologicalClock, &T), (With<BiologicalClock>, With<T>)>)
where
    T: Component + BiologicalTrait,
{
    for (entity, mut bioglical_clock, component) in queries.iter_mut() {
        if component.get_lifetime() < bioglical_clock.lifetime.elapsed() {
            // Dark, but kills of all the children if the parent dies
            commands.entity(entity).despawn_recursive();
            debug!("Entity died");
        } else {
            bioglical_clock.lifetime.tick(time.delta());
        }
    }
}

/// Acts like a garbage collector to remove entities that have exceeded their parent's lifetime
pub fn refresh_by_parent_lifetime<T, U>(
    mut commands: Commands,
    time: Res<Time>,
    parent_queries: Query<&T, With<T>>,
    mut child_queries: Query<(&Parent, Entity, &mut BiologicalClock), (With<BiologicalClock>, With<U>)>,
) where
    T: Component + BiologicalTrait,
    U: Component,
{
    for (parent, child_entity, mut child_bioglical_clock) in child_queries.iter_mut() {
        match parent_queries.get(**parent) {
            Ok(parent_component) => {
                if parent_component.get_lifetime() < child_bioglical_clock.lifetime.elapsed() {
                    // Dark, but kills of all the children if the parent dies
                    commands.entity(child_entity).despawn_recursive();
                    debug!("Entity died");
                } else {
                    child_bioglical_clock.lifetime.tick(time.delta());
                }
            }
            Err(_) => {
                warn!("Parent entity does not exist");
                continue;
            }
        }
    }
}
