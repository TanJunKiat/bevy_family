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
        .add_event::<CudEvent<Building, String>>()
        .add_event::<CudEvent<Level, String>>()
        .add_event::<CudEvent<Room, String>>()
        .add_systems(Update, cud_bundle::<Building, String>)
        .add_systems(Update, cud_bundle::<Level, String>)
        .add_systems(Update, cud_bundle::<Room, String>)
        .add_plugins(EguiPlugin)
        .add_systems(Update, interaction_panel)
        .add_systems(Update, lineage_panel)
        .run();
}

fn interaction_panel(
    mut contexts: EguiContexts,
    mut building_event_writer: EventWriter<CudEvent<Building, String>>,
    mut level_event_writer: EventWriter<CudEvent<Level, String>>,
    mut room_event_writer: EventWriter<CudEvent<Room, String>>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Building Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add building").clicked() {
                    building_event_writer.send(CudEvent::create_parent("Building".into(), Building));
                }
                if ui.button("Modify building").clicked() {
                    building_event_writer.send(CudEvent::update_parent("Building".into(), Building));
                }
                if ui.button("Remove building").clicked() {
                    building_event_writer.send(CudEvent::delete_parent("Building".into(), Building));
                }
            });

            ui.separator();

            ui.label("Level Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add level").clicked() {
                    level_event_writer.send(CudEvent::create_child(
                        "Building".into(),
                        "Level".into(),
                        Level,
                    ));
                }
                if ui.button("Modify level").clicked() {
                    level_event_writer.send(CudEvent::update_child(
                        "Building".into(),
                        "Level".into(),
                        Level,
                    ));
                }
                if ui.button("Remove level").clicked() {
                    level_event_writer.send(CudEvent::delete_child(
                        "Building".into(),
                        "Level".into(),
                        Level,
                    ));
                }
            });

            ui.separator();

            ui.label("Room Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add room").clicked() {
                    room_event_writer.send(CudEvent::create_child("Level".into(), "Room".into(), Room));
                }
                if ui.button("Modify room").clicked() {
                    room_event_writer.send(CudEvent::update_child("Level".into(), "Room".into(), Room));
                }
                if ui.button("Remove room").clicked() {
                    room_event_writer.send(CudEvent::delete_child("Level".into(), "Room".into(), Room));
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
