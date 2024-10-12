use std::sync::Arc;

use bevy::{
    app::Plugin,
    asset::{AssetServer, Handle},
    prelude::{Image, NextState, OnEnter, Res, ResMut, Resource},
    utils::HashMap,
};

use crate::{item::ItemId, GameState};

#[derive(Debug, Resource, Default)]
pub struct GameTextrue {
    pub item: HashMap<Arc<ItemId>, Handle<Image>>,
}

fn register_textrue(
    mut res: ResMut<GameTextrue>,
    asset: Res<AssetServer>,
    mut state: ResMut<NextState<GameState>>,
) {
    for id in 0..10 {
        let arc = Arc::new(ItemId(id));
        let path = format!("{}.png",id.to_string());
        let test = asset.load::<Image>(path);
        res.item.insert(arc, test);
    }
    state.set(GameState::RegisterItemdorp);
}

pub struct TextruePlugin;

impl Plugin for TextruePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GameTextrue>()
            .add_systems(OnEnter(GameState::RegisterIdToTextrue), register_textrue);
    }
}
