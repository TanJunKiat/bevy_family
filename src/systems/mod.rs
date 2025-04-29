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

pub fn cud_bundle<U, V>(
    mut commands: Commands,
    queries: Query<(Entity, &Identifier<V>), With<Identifier<V>>>,
    mut events: EventReader<CudEvent<U, V>>,
    mut lineage: ResMut<Lineage<V>>,
) where
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
                                    let child = commands
                                        .spawn((
                                            event.get_bundle(),
                                            self_identifier.clone(),
                                            BiologicalClock::default(),
                                        ))
                                        .id();
                                    commands.entity(parent_entity).add_child(child);
                                    debug!(
                                        "Child entity {:?} under parent entity {:?} is updated.",
                                        self_identifier, parent_entity
                                    );
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                Action::Delete => {
                                    commands.entity(child_entity).despawn_recursive();
                                    debug!(
                                        "Child entity {:?} under parent entity {:?} is deleted.",
                                        self_identifier, parent_entity
                                    );
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                Action::Clear => {
                                    commands.entity(child_entity).despawn_descendants();
                                    debug!(
                                        "Child entity {:?}'s childrens are cleared.",
                                        self_identifier
                                    );
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                _ => {
                                    warn!(
                                        "Parent {:?} already consist of child entity {:?}.",
                                        parent_identifier, self_identifier
                                    );
                                    lineage.add_history(event.to_history(Err(())));
                                }
                            },
                            // if child not found
                            None => match action {
                                Action::Create | Action::CreateOrModify => {
                                    let child = commands
                                        .spawn((
                                            event.get_bundle(),
                                            self_identifier.clone(),
                                            BiologicalClock::default(),
                                        ))
                                        .id();
                                    commands.entity(parent_entity).add_child(child);
                                    debug!(
                                        "Child entity {:?} created under parent entity {:?}.",
                                        self_identifier, parent_entity,
                                    );
                                    lineage.add_history(event.to_history(Ok(())));
                                }
                                _ => {
                                    warn!(
                                        "Parent entity {:?} does not have child entity {:?}.",
                                        parent_entity, self_identifier
                                    );
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
                            commands.spawn((
                                event.get_bundle(),
                                self_identifier.clone(),
                                BiologicalClock::default(),
                            ));
                            lineage.add_history(event.to_history(Ok(())));
                        }
                        Action::Delete => {
                            commands.entity(entity).despawn_recursive();
                            debug!("Parent entity {:?} deleted.", self_identifier);
                            lineage.add_history(event.to_history(Ok(())));
                        }
                        Action::Clear => {
                            commands.entity(entity).despawn_descendants();
                            debug!(
                                "Parent entity's {:?} childrens cleared.",
                                self_identifier
                            );
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
                            commands.spawn((
                                event.get_bundle(),
                                self_identifier.clone(),
                                BiologicalClock::default(),
                            ));
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
pub fn refresh_by_own_lifetime<T>(
    mut commands: Commands,
    time: Res<Time>,
    mut queries: Query<(Entity, &mut BiologicalClock, &T), (With<BiologicalClock>, With<T>)>,
) where
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
    mut child_queries: Query<
        (&Parent, Entity, &mut BiologicalClock),
        (With<BiologicalClock>, With<U>),
    >,
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
