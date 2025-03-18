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
pub fn cud_parent_component<T, U, V>(
    mut commands: Commands,
    queries: Query<(Entity, &Identifier<V>), (With<T>, With<Identifier<V>>)>,
    mut events: EventReader<ParentEvent<U, V>>,
    mut lineage: ResMut<Lineage<V>>,
) where
    T: Component + Clone,
    U: Bundle + Clone,
    V: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static,
{
    for event in events.read() {
        match event.get_action() {
            Action::Create => match get_entity_by_identifier(&queries, event.get_self_identifier())
            {
                Some(_) => {
                    warn!(
                        "Parent entity {:?} already exists.",
                        event.get_self_identifier()
                    );
                    lineage.add_history(event.to_history(Err(())));
                }
                None => {
                    commands.spawn((
                        event.get_bundle(),
                        event.get_self_identifier().clone(),
                        BiologicalClock::default(),
                    ));
                    debug!("Parent entity {:?} created.", event.get_self_identifier());
                    lineage.add_history(event.to_history(Ok(())));
                }
            },
            Action::CreateOrModify => {
                match get_entity_by_identifier(&queries, event.get_self_identifier()) {
                    Some(entity) => {
                        commands.entity(entity).despawn_recursive();
                        commands.spawn((
                            event.get_bundle(),
                            event.get_self_identifier().clone(),
                            BiologicalClock::default(),
                        ));
                        debug!("Parent entity {:?} updated.", event.get_self_identifier());
                        lineage.add_history(event.to_history(Ok(())));
                    }
                    None => {
                        commands.spawn((
                            event.get_bundle(),
                            event.get_self_identifier().clone(),
                            BiologicalClock::default(),
                        ));
                        debug!("Parent entity {:?} created.", event.get_self_identifier());
                        lineage.add_history(event.to_history(Ok(())));
                    }
                }
            }
            Action::Update => match get_entity_by_identifier(&queries, event.get_self_identifier())
            {
                Some(entity) => {
                    commands.entity(entity).despawn_recursive();
                    commands.spawn((
                        event.get_bundle(),
                        event.get_self_identifier().clone(),
                        BiologicalClock::default(),
                    ));
                    debug!("Parent entity {:?} updated.", event.get_self_identifier());
                    lineage.add_history(event.to_history(Ok(())));
                }
                None => {
                    warn!(
                        "Parent entity {:?} does not exist.",
                        event.get_self_identifier()
                    );
                    lineage.add_history(event.to_history(Err(())));
                }
            },
            Action::Delete => match get_entity_by_identifier(&queries, event.get_self_identifier())
            {
                Some(entity) => {
                    commands.entity(entity).despawn_recursive();
                    debug!("Parent entity {:?} deleted.", event.get_self_identifier());
                    lineage.add_history(event.to_history(Ok(())));
                }
                None => {
                    warn!(
                        "Parent entity {:?} does not exist.",
                        event.get_self_identifier()
                    );
                    lineage.add_history(event.to_history(Err(())));
                }
            },
            Action::Clear => {
                match get_entity_by_identifier(&queries, event.get_self_identifier()) {
                    Some(entity) => {
                        commands.entity(entity).despawn_descendants();
                        debug!(
                            "Parent entity's {:?} childrens cleared.",
                            event.get_self_identifier()
                        );
                        lineage.add_history(event.to_history(Ok(())));
                    }
                    None => {
                        warn!(
                            "Parent entity {:?} does not exist.",
                            event.get_self_identifier()
                        );
                        lineage.add_history(event.to_history(Err(())));
                    }
                }
            }
        }
    }
}

/// Main system to handle the creation, updating and deletion of child entities
pub fn cud_child_component<T, U, V, W>(
    mut commands: Commands,
    mut events: EventReader<ChildEvent<V, W>>,
    parent_queries: Query<(Entity, &Identifier<W>), (With<T>, With<Identifier<W>>)>,
    child_queries: Query<(Entity, &Identifier<W>), (With<U>, With<Identifier<W>>)>,
    mut lineage: ResMut<Lineage<W>>,
) where
    T: Component,
    U: Component + Clone,
    V: Bundle + Clone,
    W: Clone + std::fmt::Debug + PartialEq + Send + Sync + 'static,
{
    for event in events.read() {
        let parent_entity =
            match get_entity_by_identifier(&parent_queries, event.get_parent_identifier()) {
                Some(entity) => entity,
                None => {
                    warn!(
                        "Parent entity {:?} does not exist.",
                        event.get_parent_identifier()
                    );
                    lineage.add_history(event.to_history(Err(())));
                    continue;
                }
            };

        match event.get_action() {
            Action::Create => {
                match get_entity_by_identifier(&child_queries, event.get_self_identifier()) {
                    Some(_) => {
                        warn!(
                            "Parent {:?} already consist of child entity {:?}.",
                            event.get_parent_identifier(),
                            event.get_self_identifier()
                        );
                        lineage.add_history(event.to_history(Err(())));
                    }
                    None => {
                        let child = commands
                            .spawn((
                                event.get_bundle(),
                                event.get_self_identifier().clone(),
                                BiologicalClock::default(),
                            ))
                            .id();
                        commands.entity(parent_entity).add_child(child);
                        debug!(
                            "Child entity {:?} created under parent entity {:?}.",
                            event.get_self_identifier(),
                            event.get_parent_identifier()
                        );
                        lineage.add_history(event.to_history(Ok(())));
                    }
                }
            }
            Action::CreateOrModify => {
                match get_entity_by_identifier(&child_queries, event.get_self_identifier()) {
                    Some(entity) => {
                        commands.entity(entity).despawn_recursive();
                        let child = commands
                            .spawn((
                                event.get_bundle(),
                                event.get_self_identifier().clone(),
                                BiologicalClock::default(),
                            ))
                            .id();
                        commands.entity(parent_entity).add_child(child);
                        debug!(
                            "Child entity {:?} under parent entity {:?} is updated.",
                            event.get_self_identifier(),
                            event.get_parent_identifier()
                        );
                        lineage.add_history(event.to_history(Ok(())));
                    }
                    None => {
                        let child = commands
                            .spawn((
                                event.get_bundle(),
                                event.get_self_identifier().clone(),
                                BiologicalClock::default(),
                            ))
                            .id();
                        commands.entity(parent_entity).add_child(child);
                        debug!(
                            "Child entity {:?} created under parent entity {:?}.",
                            event.get_self_identifier(),
                            event.get_parent_identifier()
                        );
                        lineage.add_history(event.to_history(Ok(())));
                    }
                }
            }
            Action::Update => {
                match get_entity_by_identifier(&child_queries, event.get_self_identifier()) {
                    Some(entity) => {
                        commands.entity(entity).despawn_recursive();
                        let child = commands
                            .spawn((
                                event.get_bundle(),
                                event.get_self_identifier().clone(),
                                BiologicalClock::default(),
                            ))
                            .id();
                        commands.entity(parent_entity).add_child(child);
                        debug!(
                            "Child entity {:?} under parent entity {:?} is updated.",
                            event.get_self_identifier(),
                            event.get_parent_identifier()
                        );
                        lineage.add_history(event.to_history(Ok(())));
                    }
                    None => {
                        warn!(
                            "Parent entity {:?} does not have child entity {:?}.",
                            event.get_parent_identifier(),
                            event.get_self_identifier()
                        );
                        lineage.add_history(event.to_history(Err(())));
                    }
                }
            }
            Action::Delete => {
                match get_entity_by_identifier(&child_queries, event.get_self_identifier()) {
                    Some(entity) => {
                        commands.entity(entity).despawn_recursive();
                        debug!(
                            "Child entity {:?} under parent entity {:?} is deleted.",
                            event.get_self_identifier(),
                            event.get_parent_identifier()
                        );
                        lineage.add_history(event.to_history(Ok(())));
                    }
                    None => {
                        warn!(
                            "Parent entity {:?} does not have child entity {:?}.",
                            event.get_parent_identifier(),
                            event.get_self_identifier()
                        );
                        lineage.add_history(event.to_history(Err(())));
                    }
                }
            }
            Action::Clear => {
                match get_entity_by_identifier(&child_queries, event.get_self_identifier()) {
                    Some(entity) => {
                        commands.entity(entity).despawn_descendants();
                        debug!(
                            "Child entity {:?}'s childrens are cleared.",
                            event.get_self_identifier()
                        );
                        lineage.add_history(event.to_history(Ok(())));
                    }
                    None => {
                        warn!(
                            "Parent entity {:?} does not have child entity {:?}.",
                            event.get_parent_identifier(),
                            event.get_self_identifier()
                        );
                        lineage.add_history(event.to_history(Err(())));
                    }
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
