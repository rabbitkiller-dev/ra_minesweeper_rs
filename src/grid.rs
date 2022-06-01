
use yew::prelude::*;
use yew::{html, Html, Properties};
use crate::app::App;

pub struct MineGrid {
}


#[derive(Clone, Properties, PartialEq)]
pub struct MineGridProps {
    pub x: u32,
    pub y: u32,
}


pub enum MineGridMessage {
    Click
}

impl Component for MineGrid {
    type Message = MineGridMessage;
    type Properties = MineGridProps;

    fn create(ctx: &Context<Self>) -> Self {
        MineGrid {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MineGridMessage::Click => {
                log::info!("触发点击")
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // key
        let key = format!("({},{})", props.x, props.y);
        log::info!("key: {}", key);
        let top = props.y * 25;
        let left = props.x * 25;
        let styles = format!("position: absolute;top: {}px;left: {}px;", top, left);
        html! {
            <div class="mine_grid" style={styles} onclick={ctx.link().callback(|_| MineGridMessage::Click)}>{ "2" }</div>
        }
    }
}
