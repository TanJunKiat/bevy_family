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
pub fn get_entity_by_identifier<T, U>(queries: &Query<(Entity, &Identifier<U>), (With<T>, With<Identifier<U>>)>, identifier: &Identifier<U>) -> Option<Entity>
where
    T: Component,
    U: PartialEq + Send + Sync + 'static,
{
    for (entity, id) in queries.iter() {
        if id == identifier {
            return Some(entity);
        }
    }
    return None;
}
