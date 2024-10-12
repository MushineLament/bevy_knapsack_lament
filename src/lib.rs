use std::sync::Arc;

use animation::AnimationPlugin;
use bevy::{
    app::{Plugin, Startup},
    asset::Handle,
    prelude::{
        default, AppExtStates, Camera2d, Camera2dBundle, Commands, Component, Image, OnEnter,
        States,
    },
    transform::commands,
};
use curse::CursePlugin;
use item::{ItemId, ItemsPlugin};
use knapsack_system::KnapsackChangePlugin;
use knapsack_ui::KnapsackPlanePlugin;
use state_system::{PlayerStatePlugin, StateTrait};
use texture::TextruePlugin;

pub mod knapsack_ui;

pub mod knapsack_system;

pub mod item;

pub mod texture;

pub mod state_system;
pub mod curse;
pub mod animation;

#[derive(Component, Debug,Clone, Copy)]
pub struct KnapsackSlotID(i8);



#[derive(Debug, States, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    RegisterIdToTextrue,
    RegisterItemdorp,
    PlayerTake,
    End,
}

pub struct Plugins;

impl Plugin for Plugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameState>()
            .add_plugins(KnapsackPlanePlugin)
            .add_plugins(KnapsackChangePlugin)
            .add_plugins(ItemsPlugin)
            .add_plugins(TextruePlugin)
            .add_plugins(PlayerStatePlugin)
            .add_plugins(CursePlugin)
            .add_plugins(AnimationPlugin)
            .add_systems(
                OnEnter(GameState::RegisterIdToTextrue),
                |mut commands: Commands| {
                    commands.spawn(Camera2dBundle::default());
                },
            );
    }
}
