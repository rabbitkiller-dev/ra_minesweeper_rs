use crate::app::App;
use crate::game_box::{GameBox, GameStatus, GridStatus};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew::{html, Callback, Html, Properties};

pub struct MineGrid {}

#[derive(Clone, Properties, PartialEq)]
pub struct MineGridProps {
    pub x: u32,
    pub y: u32,
    pub game_box: Rc<RefCell<GameBox>>,
    pub onaction: Callback<GridAction>,
}

#[derive(Debug)]
pub enum GridAction {
    Click(u32, u32),
    DbClick(u32, u32),
    RightClick(u32, u32),
}

pub enum MineGridMessage {
    Click,
}

impl Component for MineGrid {
    type Message = MineGridMessage;
    type Properties = MineGridProps;

    fn create(ctx: &Context<Self>) -> Self {
        MineGrid {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            MineGridMessage::Click => {
                let action = props.onaction.clone();
                action.emit(GridAction::Click(props.x, props.y));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let game_box = props.game_box.borrow();
        match game_box.status {
            GameStatus::Play => {
                self.view_in_play(ctx)
            },
            GameStatus::Win => todo!(),
            GameStatus::Over => todo!(),
        }
    }
}

impl MineGrid {

    fn view_in_play(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let game_box = props.game_box.borrow();
        // key
        let key = format!("({},{})", props.x, props.y);
        // log::info!("{key}");
        let top = props.y * 25;
        let left = props.x * 25;
        let styles = format!("position: absolute;top: {}px;left: {}px;", top, left);
        let mut class = classes!("grid");
        let label = &game_box.label_map[props.y as usize][props.x as usize];

        match label {
            GridStatus::None => {
            },
            GridStatus::Mine  => {
                class.push("grid_is_mine");
            },
            GridStatus::OpenMine => {
                class.push("grid_is_mine");
            },
            GridStatus::Open(num) => {
            },
        };
        html! {
            <div class={class}
                style={styles}
                onclick={ctx.link().callback(|_| MineGridMessage::Click)}
            >
            if let GridStatus::Open(num) = label {
                { num }
            }
            </div>
        }

    }
}