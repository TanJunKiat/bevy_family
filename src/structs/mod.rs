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

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Identifier<T>(pub T);

/// Event that is used to create, update and delete parent entities
#[derive(Event)]
pub struct ParentEvent<T, U> {
    pub action: Action,
    pub self_identifier: Identifier<U>,
    pub component: T,
}

/// Event that is used to create, update and delete child entities
#[derive(Event)]
pub struct ChildEvent<T, U> {
    pub action: Action,
    pub parent_identifier: Identifier<U>,
    pub self_identifier: Identifier<U>,
    pub component: T,
}

/// History of the action that has been performed
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
where T: PartialEq
{
    pub fn add_history(&mut self, history: History<T>) {
        self.histories.push(history);
    }

    pub fn get_histories_by_parent_identifier(&self, parent_identifier: &Identifier<T>) -> Vec<&History<T>> {
        let mut histories = Vec::new();
        for history in &self.histories {
            if &history.parent_identifier == parent_identifier {
                histories.push(history);
            }
        }
        return histories;
    }

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
}

impl<T> History<T> {
    pub fn new_parent_history(
        action: Action,
        parent_identifier: Identifier<T>,
        result: Result<(), ()>,
    ) -> Self {
        Self {
            action,
            parent_identifier,
            child_identifier: None,
            result,
        }
    }

    pub fn new_child_history(
        action: Action,
        parent_identifier: Identifier<T>,
        child_identifier: Identifier<T>,
        result: Result<(), ()>,
    ) -> Self {
        Self {
            action,
            parent_identifier,
            child_identifier: Some(child_identifier),
            result,
        }
    }
}
