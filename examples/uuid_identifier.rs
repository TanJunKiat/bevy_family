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
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_family::*;
use uuid::Uuid;
#[derive(Component, Clone)]
struct Building;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamilyPlugin::<Uuid>::default())
        .add_event::<ParentEvent<Building, Uuid>>()
        .add_systems(Update, cud_parent_component::<Building, Uuid>)
        .add_plugins(EguiPlugin)
        .add_systems(Update, interaction_panel)
        .add_systems(Update, lineage_panel)
        .run();
}

#[derive(Resource)]
struct UuidResource {
    uuid: Uuid,
    uuid_history: Vec<Uuid>,
}

impl Default for UuidResource {
    fn default() -> Self {
        UuidResource {
            uuid: Uuid::new_v4(),
            uuid_history: Vec::new(),
        }
    }
}

fn interaction_panel(
    mut contexts: EguiContexts,
    mut parent_event_writer: EventWriter<ParentEvent<Building, Uuid>>,
    mut uuid_resource: Local<UuidResource>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("UUID");
                ui.label(format!("{:?}", uuid_resource.uuid));
            });
            ui.horizontal(|ui| {
                if ui.button("New uuid").clicked() {
                    let current_uuid = uuid_resource.uuid.clone();
                    uuid_resource.uuid_history.push(current_uuid);
                    uuid_resource.uuid = Uuid::new_v4();
                }
                if ui.button("previous uuid").clicked() {
                    if let Some(uuid) = uuid_resource.uuid_history.pop() {
                        uuid_resource.uuid = uuid;
                    }
                }
            });

            ui.separator();

            ui.label("Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add parent").clicked() {
                    parent_event_writer.send(ParentEvent::create(uuid_resource.uuid, Building));
                }
                if ui.button("Modify parent").clicked() {
                    parent_event_writer.send(ParentEvent::update(uuid_resource.uuid, Building));
                }
                if ui.button("Remove parent").clicked() {
                    parent_event_writer.send(ParentEvent::delete(uuid_resource.uuid, Building));
                }
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn lineage_panel(mut contexts: EguiContexts, lineage: Res<Lineage<Uuid>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Lineage");
            for history in lineage.histories.iter() {
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}", history.action));
                    ui.label(format!("{:?}", history.parent_identifier));
                    ui.label(format!("{:?}", history.child_identifier));
                    ui.label(format!("{:?}", history.result));
                });
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
