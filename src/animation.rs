use std::{any::Any, fmt::Debug, sync::Arc};

use bevy::{
    app::{Plugin, Update},
    log::info,
    prelude::{in_state, Component, IntoSystemConfigs, Query},
};

use crate::{
    item::{Animation, Curse},
    state_system::StateTrait,
    GameState,
};

//如果物品的动画需要更改之类的，这里便是修改的地方
fn item_animation_system<T>(query: Query<&T>)
where
    T: StateTrait + Send + Sync + 'static + Component + Debug,
{
    for x in &query {
        info!("{:?}", x);
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(
                Update,
                ((item_animation_system::<Animation>,).run_if(in_state(GameState::PlayerTake)),),
            );
    }
}
