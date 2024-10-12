use std::{any::Any, fmt::Debug, sync::Arc};

use bevy::{
    app::{Plugin, Update},
    log::info,
    prelude::{in_state, Component, IntoSystemConfigs, Query, RemovedComponents, With},
};

use crate::{item::Curse, knapsack_system::ItemInKapsackMark, state_system::StateTrait, GameState};

pub trait CurveTrait {}

#[derive(Debug, Component)]
pub struct SpeedBuff;

impl CurveTrait for SpeedBuff {}

#[derive(Debug, Component)]
pub struct PowerBuff;

impl CurveTrait for PowerBuff {}

//药水的buff就按照这样进行设置
fn speed_buff_system<T>(query: Query<&T>)
where
    T: StateTrait + Send + Sync + 'static + Component + Debug,
{
    for x in &query {
        info!("{:?}", x);
    }
}

//物品没有了导致curse（药水效果），导致触发了下面的代码
fn remove_speed_buff_system<T>(mut removals: RemovedComponents<T>)
where
    T: StateTrait + Send + Sync + 'static + Component + Debug,
{
    for entity in removals.read() {
        // do something with the entity
        eprintln!("Entity {:?} had the component removed.", entity);
    }
}

pub struct CursePlugin;

impl Plugin for CursePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .add_systems(OnEnter(GameState::PlayerTake), take_itemdorp::<TestMark>)
            // .add_systems(Update, item_insert_knapsack)
            .add_systems(
                Update,
                (
                    (speed_buff_system::<Curse<SpeedBuff>>,) //虽然chain（）会降低点性能，但是这里一旦并发，会导致错误
                        .run_if(in_state(GameState::PlayerTake)),
                    remove_speed_buff_system::<Curse<SpeedBuff>>,
                ),
            );
    }
}
