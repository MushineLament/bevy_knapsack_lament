use std::{fmt::Debug, sync::Arc};

use bevy::{
    app::Plugin,
    asset::Handle,
    prelude::{Component, Image, NextState, OnEnter, Res, ResMut, Resource},
    utils::hashbrown::hash_map::IntoIter,
};

use crate::{
    curse::{CurveTrait, PowerBuff, SpeedBuff},
    state_system::StateTrait,
    texture::GameTextrue,
    GameState,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ItemId(pub i32);

#[derive(Component, Debug, PartialEq, Eq)]
pub struct Item<T: Sized + Send + Sync + StateTrait + 'static> {
    pub id: Arc<ItemId>,
    pub count: i32,
    pub image: Handle<Image>,
    pub state: Vec<T>,
    //..
}

/// 赋予玩家（药水？）状态
/// 现在我怎么确保vec<T>里面的状态不重复呢？
///
/// 泛型T不是被保存成一段段的u64之类的吗？
/// 那我为什么不能通过这段u64翻译成泛型，进而避免存储不定类泛型呢？
///
/// 我打算弄成这样  Curse<T: Component + Debug + CurveTrait>{
///     持续时间 ：..
///     药水效果等级 ： ..
///     药水类型 ； T
/// }
///
/// 假设我可以用过u64存储T的值，等到添加组件的时候，我把这个T给还原成对应的药水组件mark，岂不美哉？
///
///
/// 那我Item也可以用啊，但是Item只是划分物品的类型，量要求不大
impl<T: Sized + Send + Sync + StateTrait + 'static> Item<T> {
    pub fn to_component(self) -> (Self, Vec<T>) {
        let Self {
            id,
            count,
            image,
            state,
        } = self;
        let item = Self {
            id,
            count,
            image,
            state: vec![],
        };
        (item, state)
    }
}

//这里仅仅只是获取凋落物的引用，实际上游戏掉落物的情况要看情况进行处理。
#[derive(Debug, Resource, Default)]
pub struct ItemDorpResource {
    pub item: Vec<Item<Curse<SpeedBuff>>>, //这里只是模拟玩家能捡到的物品，使用场景实际上会更加复杂，更加麻烦。
}

fn set_itemdorp(
    mut res: ResMut<ItemDorpResource>,
    textrue: Res<GameTextrue>,
    mut state: ResMut<NextState<GameState>>,
) {
    for id in 0..10 {
        let arc_id = Arc::new(ItemId(id));
        res.item.push(Item {
            id: arc_id.clone(),
            count: 1,
            image: textrue
                .item
                .get(&arc_id)
                .unwrap_or(&Handle::<Image>::default())
                .clone(),
            state: vec![Curse(vec![SpeedBuff])],
        });
    }
    state.set(GameState::PlayerTake);
}

///单纯的物品
#[derive(Component, Debug)]
pub struct Base;

/// 赋予玩家（药水？）状态
/// 现在我怎么确保vec<T>里面的状态不重复呢？
///
/// 泛型T不是被保存成一段段的u64之类的吗？
/// 那我为什么不能通过这段u64翻译成泛型，进而避免存储不定类泛型呢？
///
/// 我打算弄成这样  Curse<T: Component + Debug + CurveTrait>{
///     持续时间 ：..
///     药水效果等级 ： ..
///     药水类型 ； T
/// }
///
/// 假设我可以用过u64存储T的值，等到添加组件的时候，我把这个T给还原成对应的药水组件mark，岂不美哉？
#[derive(Component, Debug)]
pub struct Curse<T: Component + Debug + CurveTrait>(pub Vec<T>);
#[derive(Component, Debug)]
pub struct Animation {
    delay: u64, //延迟几秒播放动画
    data: std::vec::IntoIter<(i32, Handle<Image>)>,
}

impl StateTrait for Base {}
impl<T: Component + Debug + CurveTrait> StateTrait for Curse<T> {}
impl StateTrait for Animation {}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ItemDorpResource>()
            .add_systems(OnEnter(GameState::RegisterItemdorp), set_itemdorp);
    }
}
