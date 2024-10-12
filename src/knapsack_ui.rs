use bevy::{
    app::{Plugin, Startup},
    color::Color,
    log::info,
    prelude::{BuildChildren, ButtonBundle, Commands, Component, NodeBundle, Parent, Query, With},
    transform::commands,
    ui::{
        AlignContent, AlignItems, BackgroundColor, BorderColor, BorderRadius, FlexWrap,
        JustifyContent, Style, UiRect, Val,
    },
};

use crate::{knapsack_system::ItemInKapsackMark, KnapsackSlotID};

/*
    我设计的背包结构大概是这样

    玩家（entity）
     >背包组件
         >卡槽组件（组件数量取决于背包大小）
            >Item（物品）

    因此，实际上我们可以通过获取item的parent的entity_slot，通过entity_slot获取knapsack_entity
    依次向上传递，就可以获取玩家的entity，赋予玩家状态
    但是，bevy目前好像还不支持通过entity获取他的parent
*/

fn component_spawn(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(800.0),
                align_content: AlignContent::Start,
                justify_content: JustifyContent::FlexStart,

                column_gap: Val::Px(10.0),

                flex_wrap: FlexWrap::Wrap,
                row_gap: Val::Px(10.0),
                ..Default::default()
            },
            border_color: BorderColor(Color::srgb(1.0, 1.0, 1.0)),
            border_radius: BorderRadius::all(Val::Px(10.0)),

            ..Default::default()
        })
        .with_children(|parent| {
            let mut mub = 0;
            for x in -4..=-2 {
                mub += x;
                for y in -4..=-2 {
                    mub += y;
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                // left: Val::Percent(1.0 / x as f32),
                                // top: Val::Percent(1.0 / y as f32),
                                width: Val::Px(80.0),
                                height: Val::Px(80.0),

                                border: UiRect::all(Val::Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            border_color: BorderColor(Color::srgb(1.0, 1.0, 1.0)),
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..Default::default()
                        })
                        .insert(KnapsackSlotID(mub));
                }
            }
        });
}

pub struct KnapsackPlanePlugin;

impl Plugin for KnapsackPlanePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, component_spawn);
    }
}
