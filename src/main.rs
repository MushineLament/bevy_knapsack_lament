use std::time::Instant;

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, Children, DespawnRecursiveExt},
    log::info,
    math::Vec2,
    render::texture::ImagePlugin,
    sprite::{Sprite, SpriteBundle},
    time::{Stopwatch, Time, Timer},
    transform::{self, commands, components::Transform},
    utils::HashMap,
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rust_deep_go::Plugins;

/// 注意，这背包系统能优化的性能并不大
/// 按着我想法设计的背包，按理来说可操作性更大，
/// 但是，代码堆积量十分大，属于空间换时间的一种方式
/// 并且，这样设计的背包系统，需要你对bevy熟悉。
/// 
/// 里面的代码核心就是Item<T>的设计，仿制他的设计思路
/// 后面就是堆量了

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1280.0, 800.0)
                        .with_scale_factor_override(1.0),
                    ..Default::default()
                }),
                ..Default::default()
            }), // .set(ImagePlugin::default_nearest())
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(Plugins)
        .run();
}
