use std::{any::Any, fmt::Debug, sync::Arc};

use bevy::{
    app::{Plugin, Update},
    asset::Handle,
    log::info,
    prelude::{
        any_component_removed, in_state, BuildChildren, Changed, Children, Commands, Component,
        Entity, Image, ImageBundle, IntoSystemConfigs, Mut, NextState, OnEnter, Parent, Query, Res,
        ResMut, With, Without, World,
    },
    transform::commands,
    ui::UiImage,
};

use crate::{
    // items::ItemDorpResource,
    curse::{CurveTrait, SpeedBuff},
    item::{Animation, Base, Curse, Item, ItemDorpResource, ItemId},
    state_system::StateTrait,
    GameState,
    KnapsackSlotID,
};

//玩家捡起掉落物品
fn take_itemdorp<T: StateTrait + Send + Sync + 'static + Debug>(
    mut commands: Commands,
    empty_slot: Query<Entity, (With<KnapsackSlotID>, Without<Children>)>, //仅获取没有物品的背包slot
    mut knapsack_item: Query<(&mut Item<T>, Entity), With<ItemInKapsackMark>>, //获取处于背包的卡槽上的item

    mut itemdorp: ResMut<ItemDorpResource>,
    mut state: ResMut<NextState<GameState>>,
) {
    //如果掉落物品为空，或者说玩家可捡起的掉落物品为0，则提前返回。
    //如果玩家的背包为空，则不会执行捡起物品的操作
    if itemdorp.item.is_empty() || empty_slot.is_empty() {
        return;
    }

    //啊，对了，这里没加验证是哪个玩家捡起了物品
    for (mut knapsack_item, _knapsack_item_entity) in &mut knapsack_item {
        //下面这段，感觉还有优化空间，但是玩家捡起的物品一般不超过上百个物品，因此优化这部分获取的性能提升较为有限
        //如果是多人，可以尝试继续优化
        itemdorp.item.retain_mut(|item_dorp| {
            let mut retu = true;
            if item_dorp.id == knapsack_item.id {
                knapsack_item.count += item_dorp.count;
                retu = false;
            }
            retu
        });
    }

    // 这里应该不会出现，玩家的背包被塞满了，但是pop还会执行，导致掉落物bug消失
    for slot_entity in &empty_slot {
        if let Some(item_dorp) = itemdorp.item.pop() {
            let image = item_dorp.image.clone();

            let (component, state) = item_dorp.to_component();

            let mut entity = commands.spawn(component);

            //插入item包含的状态
            for state in state {
                entity.insert(state);
            }

            entity
                .insert(ImageBundle {
                    image: UiImage::new(image),
                    ..Default::default()
                })
                .insert(ItemInKapsackMark);
            let id = entity.id();
            commands.entity(id).set_parent(slot_entity);
        } else {
            break;
        }
    }

    state.set(GameState::End);
}

#[derive(Debug, Component)]
pub struct ItemInKapsackMark;

//检测是否有item插入了玩家的背包，但是还需要query changed的item，或者直接从commands.add 上查找entity下的item属性
fn item_insert_knapsack(
    mut commands: Commands,
    mut query: Query<(&Children, Entity), (With<KnapsackSlotID>, Changed<Children>)>,
) {
    for (children, entity) in &query {
        if children.is_empty() {
            // info!("此处运行:{:?}", entity);
        } else {
            for child in children {
                // info!("此处运行:{:?},{:?}", entity, child);
            }
        }
    }
}

pub struct KnapsackChangePlugin;

impl Plugin for KnapsackChangePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .add_systems(OnEnter(GameState::PlayerTake), take_itemdorp::<TestMark>)
            // .add_systems(Update, item_insert_knapsack)
            .add_systems(
                Update,
                (((
                    take_itemdorp::<Base>,
                    take_itemdorp::<Curse<SpeedBuff>>,
                    take_itemdorp::<Animation>,
                )
                    .chain()) //这里一旦并发，会导致错误
                .run_if(in_state(GameState::PlayerTake)),),
            );
    }
}
