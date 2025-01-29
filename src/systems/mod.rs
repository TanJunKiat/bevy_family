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
use super::*;

/// Main system to handle the creation, updating and deletion of parent entities
pub fn cud_parent_component<T, U>(
    mut commands: Commands,
    mut queries: Query<(Entity, &mut T, &Identifier<U>), (With<T>, With<Identifier<U>>)>,
    mut events: EventReader<ParentEvent<T, U>>,
    mut lineage: ResMut<Lineage<U>>,
) where
    T: Component + Clone,
    U: Clone + PartialEq + Send + Sync + 'static,
{
    for event in events.read() {
        match event.action {
            Action::Create => match get_entity_by_identifier(&queries, &event.self_identifier) {
                Some(_) => {
                    warn!("Parent entity already exists");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Err(()));
                    lineage.add_history(history);
                }
                None => {
                    commands.spawn((event.component.clone(), event.self_identifier.clone(), BiologicalClock::default()));
                    info!("Parent entity created");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Ok(()));
                    lineage.add_history(history);
                }
            },
            Action::Update => match get_entity_by_identifier(&queries, &event.self_identifier) {
                Some(entity) => {
                    let (entity, mut component, _) = queries.get_mut(entity).unwrap();
                    commands.entity(entity).insert(BiologicalClock::default());
                    *component = event.component.clone();
                    info!("Parent entity updated");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Ok(()));
                    lineage.add_history(history);
                }
                None => {
                    warn!("Parent entity does not exist");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Err(()));
                    lineage.add_history(history);
                }
            },
            Action::Delete => match get_entity_by_identifier(&queries, &event.self_identifier) {
                Some(entity) => {
                    commands.entity(entity).despawn_recursive();
                    info!("Parent entity deleted");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Ok(()));
                    lineage.add_history(history);
                }
                None => {
                    warn!("Parent entity does not exist");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Err(()));
                    lineage.add_history(history);
                }
            },
            Action::Clear => match get_entity_by_identifier(&queries, &event.self_identifier) {
                Some(entity) => {
                    commands.entity(entity).despawn_descendants();
                    info!("Parent entity's childrens cleared");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Ok(()));
                    lineage.add_history(history);
                }
                None => {
                    warn!("Parent entity does not exist");
                    let history = History::new_parent_history(event.action.clone(), event.self_identifier.clone(), Err(()));
                    lineage.add_history(history);
                }
            },
        }
    }
}

/// Main system to handle the creation, updating and deletion of child entities
pub fn cud_child_component<T, U, V>(
    mut commands: Commands,
    mut events: EventReader<ChildEvent<U, V>>,
    parent_queries: Query<(Entity, &mut T, &Identifier<V>), (With<T>, With<Identifier<V>>)>,
    mut child_queries: Query<(Entity, &mut U, &Identifier<V>), (With<U>, With<Identifier<V>>)>,
    mut lineage: ResMut<Lineage<V>>,
) where
    T: Component,
    U: Component + Clone,
    V: Clone + PartialEq + Send + Sync + 'static,
{
    for event in events.read() {
        let parent_entity = match get_entity_by_identifier(&parent_queries, &event.parent_identifier) {
            Some(entity) => entity,
            None => {
                warn!("Parent entity does not exist");
                let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Err(()));
                lineage.add_history(history);
                continue;
            }
        };

        match event.action {
            Action::Create => match get_entity_by_identifier(&child_queries, &event.self_identifier) {
                Some(_) => {
                    warn!("Child entity already exists");
                    let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Err(()));
                    lineage.add_history(history);
                }
                None => {
                    let child_id = commands.spawn((event.component.clone(), event.self_identifier.clone(), BiologicalClock::default())).id();
                    commands.entity(parent_entity).add_child(child_id);
                    info!("Child entity created");
                    let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Ok(()));
                    lineage.add_history(history);
                }
            },
            Action::Update => {
                match get_entity_by_identifier(&child_queries, &event.self_identifier) {
                    Some(entity) => {
                        let (entity, mut component, _) = child_queries.get_mut(entity).unwrap();
                        // refresh the biological clock
                        commands.entity(entity).insert(BiologicalClock::default());
                        *component = event.component.clone();
                        info!("Child entity updated");
                        let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Ok(()));
                        lineage.add_history(history);
                    }
                    None => {
                        warn!("Child entity does not exist");
                        let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Err(()));
                        lineage.add_history(history);
                    }
                }
            }
            Action::Delete => match get_entity_by_identifier(&child_queries, &event.self_identifier) {
                Some(entity) => {
                    commands.entity(entity).despawn_recursive();
                    info!("Child entity deleted");
                    let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Ok(()));
                    lineage.add_history(history);
                }
                None => {
                    warn!("Child entity does not exist");
                    let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Err(()));
                    lineage.add_history(history);
                }
            },
            Action::Clear => match get_entity_by_identifier(&child_queries, &event.self_identifier) {
                Some(entity) => {
                    commands.entity(entity).despawn_descendants();
                    info!("Child entity's childrens cleared");
                    let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Ok(()));
                    lineage.add_history(history);
                }
                None => {
                    warn!("Child entity does not exist");
                    let history = History::new_child_history(event.action.clone(), event.parent_identifier.clone(), event.self_identifier.clone(), Err(()));
                    lineage.add_history(history);
                }
            },
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
            info!("Entity died");
        } else {
            bioglical_clock.lifetime.tick(time.delta());
        }
    }
}

/// Acts like a garbage collector to remove entities that have exceeded their parent's lifetime
pub fn refresh_by_parent_lifetime<T, U>(mut commands: Commands, time: Res<Time>,
    parent_queries: Query<&T, With<T>>,
    mut child_queries: Query<(&Parent, Entity, &mut BiologicalClock), (With<BiologicalClock>, With<U>)>)
where
    T: Component + BiologicalTrait,
    U: Component,
{
    for (parent, child_entity, mut child_bioglical_clock) in child_queries.iter_mut() {
        match parent_queries.get(**parent) {
            Ok(parent_component) => {
                if parent_component.get_lifetime() < child_bioglical_clock.lifetime.elapsed() {
                    // Dark, but kills of all the children if the parent dies
                    commands.entity(child_entity).despawn_recursive();
                    info!("Entity died");
                } else {
                    child_bioglical_clock.lifetime.tick(time.delta());
                }
            },
            Err(_) => {
                warn!("Parent entity does not exist");
                continue;
            }
        }
    }
}
