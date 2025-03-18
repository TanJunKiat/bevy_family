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
struct Parent1;

#[derive(Component, Clone)]
struct Parent2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FamilyPlugin::<String>::default())
        .add_event::<ParentEvent<Parent1, String>>()
        .add_event::<ParentEvent<Parent2, String>>()
        .add_systems(Update, cud_parent_component::<Parent1, Parent1, String>)
        .add_systems(Update, cud_parent_component::<Parent2, Parent2, String>)
        .add_plugins(EguiPlugin)
        .add_systems(Update, interaction_panel)
        .add_systems(Update, lineage_panel)
        .run();
}

fn interaction_panel(
    mut contexts: EguiContexts,
    mut parent_1_event_writer: EventWriter<ParentEvent<Parent1, String>>,
    mut parent_2_event_writer: EventWriter<ParentEvent<Parent2, String>>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Parent 1");
            ui.horizontal(|ui| {
                if ui.button("Add parent").clicked() {
                    parent_1_event_writer.send(ParentEvent::create("Parent1_name".into(), Parent1));
                }
                if ui.button("Modify parent").clicked() {
                    parent_1_event_writer.send(ParentEvent::update("Parent1_name".into(), Parent1));
                }
                if ui.button("Remove parent").clicked() {
                    parent_1_event_writer.send(ParentEvent::delete("Parent1_name".into(), Parent1));
                }
            });

            ui.separator();

            ui.label("Parent 2");
            ui.horizontal(|ui| {
                if ui.button("Add parent").clicked() {
                    parent_2_event_writer.send(ParentEvent::create("Parent2_name".into(), Parent2));
                }
                if ui.button("Modify parent").clicked() {
                    parent_2_event_writer.send(ParentEvent::update("Parent2_name".into(), Parent2));
                }
                if ui.button("Remove parent").clicked() {
                    parent_2_event_writer.send(ParentEvent::delete("Parent2_name".into(), Parent2));
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
