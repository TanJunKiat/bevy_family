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

mod enums;
mod functions;
mod structs;
mod systems;
mod traits;

pub use enums::*;
pub use functions::*;
pub use structs::*;
pub use systems::*;
pub use traits::*;

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
