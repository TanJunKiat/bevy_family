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

/// Get the entity by the identifier.
pub fn get_entity_by_identifier<T, U>(queries: &Query<(Entity, &mut T, &Identifier<U>), (With<T>, With<Identifier<U>>)>, identifier: &Identifier<U>) -> Option<Entity>
where
    T: Component,
    U: PartialEq + Send + Sync + 'static,
{
    for (entity, _, id) in queries.iter() {
        if id == identifier {
            return Some(entity);
        }
    }
    return None;
}

/// Get the component by the identifier.
pub fn get_component_by_identifier<T, U>(queries: &Query<(Entity, &mut T, &Identifier<U>), (With<T>, With<Identifier<U>>)>, identifier: &Identifier<U>) -> Option<T>
where
    T: Component + Clone,
    U: PartialEq + Send + Sync + 'static,
{
    for (_, component, id) in queries.iter() {
        if id == identifier {
            return Some(component.clone());
        }
    }
    return None;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, Clone)]
    struct ExperiencePoints;
    
    fn runtime_system(
        queries: Query<(Entity, &mut ExperiencePoints, &Identifier<String>), (With<ExperiencePoints>, With<Identifier<String>>)>,
    ) {
        let identifier = Identifier("player".to_string());
        let entity = get_entity_by_identifier::<ExperiencePoints, String>(&queries, &identifier);
        assert!(entity.is_some());

        let component = get_component_by_identifier::<ExperiencePoints, String>(&queries, &identifier);
        assert!(component.is_some());
    }

    #[test]
    fn get_entity_by_identifier_test() {
        let mut app = App::new();
        app.world_mut().spawn((ExperiencePoints,Identifier("player".to_string())));
        app.add_systems(Update, runtime_system);
        app.update();
    }
}