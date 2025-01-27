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

#[derive(Component, Clone)]
struct Building;

#[derive(Component, Clone)]
struct Level;

#[derive(Component, Clone)]
struct Room;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamilyPlugin::<String>::default())
        .add_event::<ParentEvent<Building, String>>()
        .add_event::<ChildEvent<Level, String>>()
        .add_event::<ChildEvent<Room, String>>()
        .add_systems(Update, cud_parent_component::<Building, String>)
        .add_systems(Update, cud_child_component::<Building, Level, String>)
        .add_systems(Update, cud_child_component::<Level, Room, String>)
        .add_plugins(EguiPlugin)
        .add_systems(Update, interaction_panel)
        .add_systems(Update, lineage_panel)
        .run();
}

fn interaction_panel(
    mut contexts: EguiContexts,
    mut building_event_writer: EventWriter<ParentEvent<Building, String>>,
    mut level_event_writer: EventWriter<ChildEvent<Level, String>>,
    mut room_event_writer: EventWriter<ChildEvent<Room, String>>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Building Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add building").clicked() {
                    building_event_writer.send(ParentEvent {
                        action: Action::Create,
                        self_identifier: Identifier("Building".to_string()),
                        component: Building,
                    });
                }
                if ui.button("Modify building").clicked() {
                    building_event_writer.send(ParentEvent {
                        action: Action::Update,
                        self_identifier: Identifier("Building".to_string()),
                        component: Building,
                    });
                }
                if ui.button("Remove building").clicked() {
                    building_event_writer.send(ParentEvent {
                        action: Action::Delete,
                        self_identifier: Identifier("Building".to_string()),
                        component: Building,
                    });
                }
            });

            ui.separator();

            ui.label("Level Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add level").clicked() {
                    level_event_writer.send(ChildEvent {
                        action: Action::Create,
                        parent_identifier: Identifier("Building".to_string()),
                        self_identifier: Identifier("Level".to_string()),
                        component: Level,
                    });
                }
                if ui.button("Modify level").clicked() {
                    level_event_writer.send(ChildEvent {
                        action: Action::Update,
                        parent_identifier: Identifier("Building".to_string()),
                        self_identifier: Identifier("Level".to_string()),
                        component: Level,
                    });
                }
                if ui.button("Remove level").clicked() {
                    level_event_writer.send(ChildEvent {
                        action: Action::Delete,
                        parent_identifier: Identifier("Building".to_string()),
                        self_identifier: Identifier("Level".to_string()),
                        component: Level,
                    });
                }
            });

            ui.separator();

            ui.label("Room Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add room").clicked() {
                    room_event_writer.send(ChildEvent {
                        action: Action::Create,
                        parent_identifier: Identifier("Level".to_string()),
                        self_identifier: Identifier("Room".to_string()),
                        component: Room,
                    });
                }
                if ui.button("Modify room").clicked() {
                    room_event_writer.send(ChildEvent {
                        action: Action::Update,
                        parent_identifier: Identifier("Level".to_string()),
                        self_identifier: Identifier("Room".to_string()),
                        component: Room,
                    });
                }
                if ui.button("Remove room").clicked() {
                    room_event_writer.send(ChildEvent {
                        action: Action::Delete,
                        parent_identifier: Identifier("Level".to_string()),
                        self_identifier: Identifier("Room".to_string()),
                        component: Room,
                    });
                }
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn lineage_panel(mut contexts: EguiContexts, lineage: Res<Lineage<String>>) {
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
