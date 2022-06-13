use yew::prelude::*;
use yew::Classes;

use crate::game_box::{GameBox, GameStatus};
use crate::grid::{MineGrid, GridAction};
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    game_box: Rc<RefCell<GameBox>>,
}

pub enum Msg {
   Action(GridAction),
   ReStart
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let game_box = GameBox::new(12, 15, 25);
        
        App {
            game_box: Rc::new(RefCell::new(game_box))
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Action(action) => {
                log::info!("格子动作: {:?}", action);
                match action {
                    GridAction::Click(x, y) => {
                        let game_box = self.game_box.clone();
                        let mut game_box = game_box.borrow_mut();
                        if game_box.is_flag(x, y) {
                            return false;
                        }
                        game_box.open_grid(x, y);
                    }
                    GridAction::RightClick(x, y) => {
                        let game_box = self.game_box.clone();
                        let mut game_box = game_box.borrow_mut();
                        game_box.flag_grid(x, y)
                    },
                }
            }
            Msg::ReStart => {
                let game_box = self.game_box.clone();
                let mut game_box = game_box.borrow_mut();
                game_box.new_game();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_box = self.game_box.borrow();
        let height = game_box.height * 26;
        let width = game_box.width * 26;
        let styles = format!("height: {height}px;width: {width}px;");
        // const grids: Vec<VNode> = vec![];

        // let a = self.mine_map.iter().map(|i| {
        //     println!("{}", i);
        //     let mut class = Classes::new();
        //     class.push("grid");
        //     if i.clone() == 1 {
        //         class.push("grid_is_mine");
        //     }
        //     html!{
        //         <div class={class}>{i}</div>
        //     }
        // }).collect::<Html>();

        let mut grids = vec![];
        for y in 0..game_box.height {
            for x in 0..game_box.width {
                // log::info!("({x}, {y})");
                let vnode = html!{
                    <MineGrid x={x} y={y} onaction={ctx.link().callback(move |action| Msg::Action(action))} game_box={self.game_box.clone()}></MineGrid>
                };
                grids.push(vnode);
            }
        }

        html! {
            <>
                if let GameStatus::Play = game_box.status {
                    <p>{ "游戏进行中: 请找出所有不是地雷的格子" }</p>
                    <p>{ format!("还有{}个格子", game_box.surplus_grid()) }</p>
                    <p>{ format!("还有{}个地雷", game_box.surplus_mine()) }</p>
                }
                if let GameStatus::Win = game_box.status {
                    <p>{ "游戏结束: 胜利" }</p>
                }
                if let GameStatus::Over = game_box.status {
                    <p>{ "游戏结束: 炸雷了" }</p>
                }
                <button onclick={ctx.link().callback(move |_| Msg::ReStart)} >{ "重开" }</button>
                <div class="gamebox" style={styles}>
                {
                    grids
                }
                </div>
            </>
        }
    }
}
