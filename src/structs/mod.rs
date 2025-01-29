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
pub struct ParentEvent<T, U> {
    action: Action,
    self_identifier: Identifier<U>,
    component: T,
}

impl<T, U> ParentEvent<T, U>
where
    U: Clone,
{
    pub fn get_action(&self) -> &Action {
        &self.action
    }
    pub fn get_self_identifier(&self) -> &Identifier<U> {
        &self.self_identifier
    }
    pub fn get_component(&self) -> &T {
        &self.component
    }
    pub fn create(self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Create,
            self_identifier: Identifier(self_identifier),
            component,
        }
    }
    pub fn update(self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Update,
            self_identifier: Identifier(self_identifier),
            component,
        }
    }
    pub fn delete(self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Delete,
            self_identifier: Identifier(self_identifier),
            component,
        }
    }
    pub fn clear(self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Clear,
            self_identifier: Identifier(self_identifier),
            component,
        }
    }
    pub fn to_history(&self, result: Result<(), ()>) -> History<U> {
        match self.action {
            Action::Create => History::new_parent_history(Action::Create, self.self_identifier.clone(), result),
            Action::Update => History::new_parent_history(Action::Update, self.self_identifier.clone(), result),
            Action::Delete => History::new_parent_history(Action::Delete, self.self_identifier.clone(), result),
            Action::Clear => History::new_parent_history(Action::Clear, self.self_identifier.clone(), result),
        }
    }
}
/// Event that is used to create, update and delete child entities
#[derive(Event)]
pub struct ChildEvent<T, U> {
    action: Action,
    parent_identifier: Identifier<U>,
    self_identifier: Identifier<U>,
    component: T,
}

impl<T, U> ChildEvent<T, U>
where
    U: Clone,
{
    pub fn get_action(&self) -> &Action {
        &self.action
    }
    pub fn get_self_identifier(&self) -> &Identifier<U> {
        &self.self_identifier
    }
    pub fn get_parent_identifier(&self) -> &Identifier<U> {
        &self.parent_identifier
    }
    pub fn get_component(&self) -> &T {
        &self.component
    }
    pub fn create(parent_identifier: U, self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Create,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Identifier(parent_identifier),
            component,
        }
    }
    pub fn update(parent_identifier: U, self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Update,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Identifier(parent_identifier),
            component,
        }
    }
    pub fn delete(parent_identifier: U, self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Delete,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Identifier(parent_identifier),
            component,
        }
    }
    pub fn clear(parent_identifier: U, self_identifier: U, component: T) -> Self {
        Self {
            action: Action::Clear,
            self_identifier: Identifier(self_identifier),
            parent_identifier: Identifier(parent_identifier),
            component,
        }
    }
    pub fn to_history(&self, result: Result<(), ()>) -> History<U> {
        match self.action {
            Action::Create => History::new_child_history(Action::Create, self.parent_identifier.clone(), self.self_identifier.clone(), result),
            Action::Update => History::new_child_history(Action::Update, self.parent_identifier.clone(), self.self_identifier.clone(), result),
            Action::Delete => History::new_child_history(Action::Delete, self.parent_identifier.clone(), self.self_identifier.clone(), result),
            Action::Clear => History::new_child_history(Action::Clear, self.parent_identifier.clone(), self.self_identifier.clone(), result),
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
