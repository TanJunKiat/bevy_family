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
struct Lift;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamilyPlugin::<String>::default())
        .add_event::<CudEvent<Building, String>>()
        .add_event::<CudEvent<(Level, Lift), String>>()
        .add_systems(Update, cud_bundle::<Building, String>)
        .add_systems(Update, cud_bundle::<(Level, Lift), String>)
        .add_plugins(EguiPlugin)
        .add_systems(Update, interaction_panel)
        .add_systems(Update, lineage_panel)
        .run();
}

fn interaction_panel(mut contexts: EguiContexts, mut parent_event_writer: EventWriter<CudEvent<Building, String>>, mut child_event_writer: EventWriter<CudEvent<(Level, Lift), String>>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Parent Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add parent").clicked() {
                    parent_event_writer.send(CudEvent::create_parent("Building".into(), Building));
                }
                if ui.button("Modify parent").clicked() {
                    parent_event_writer.send(CudEvent::update_parent("Building".into(), Building));
                }
                if ui.button("Remove parent").clicked() {
                    parent_event_writer.send(CudEvent::delete_parent("Building".into(), Building));
                }
            });

            ui.separator();

            ui.label("Child Interaction");
            ui.horizontal(|ui| {
                if ui.button("Add child").clicked() {
                    child_event_writer.send(CudEvent::create_child("Building".into(), "Level".into(), (Level, Lift)));
                }
                if ui.button("Create or modify child").clicked() {
                    child_event_writer.send(CudEvent::create_or_modify_child("Building".into(), "Level".into(), (Level, Lift)));
                }
                if ui.button("Modify child").clicked() {
                    child_event_writer.send(CudEvent::update_child("Building".into(), "Level".into(), (Level, Lift)));
                }
                if ui.button("Remove child").clicked() {
                    child_event_writer.send(CudEvent::delete_child("Building".into(), "Level".into(), (Level, Lift)));
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
