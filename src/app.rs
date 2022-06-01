use yew::prelude::*;
use yew::Classes;

use rand::Rng;

use crate::game_box::GameBox;
use crate::grid::MineGrid;

type MineMap = Vec<u32>;

pub struct App {
    game_box: GameBox,
    mine_map: MineMap
}
fn init_mine(game_box: &GameBox) -> MineMap {
        // 初始化
        let mut grids: MineMap = vec![];
        let max = game_box.height * game_box.width;
        for _ in 0..game_box.mine_max {
            grids.push(1);
        }
        for _ in game_box.mine_max..max {
            grids.push(0);
        }
        println!("{:?}", grids);
        let mut i = grids.len() - 1;
        while i > 0 {
            grids.swap(rand(0, i), i);
            i -= 1;
        }
        grids
}
fn rand(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let game_box = GameBox {
            width: 8,
            height: 8,
            mine_max: 8
        };
        let mine_map = init_mine(&game_box);
        App {
            game_box,
            mine_map: mine_map
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let height = self.game_box.height * 26;
        let width = self.game_box.width * 26;
        let styles = format!("height: {height}px;width: {width}px;");
        // const grids: Vec<VNode> = vec![];
        let a = self.mine_map.iter().map(|i| {
            println!("{}", i);
            let mut class = Classes::new();
            class.push("grid");
            if i.clone() == 1 {
                class.push("grid_is_mine");
            }
            html!{
                <div class={class}>{i}</div>
            }
        }).collect::<Html>();
        // let names = vec!["Sam","Bob","Ray"];
        // let b = names.into_iter().map(|name| {
        //     html!{<div key={name.to_string()}>{ format!("Hello, I'am {}!",name) }</div>}
        // }).collect::<Html>();

        html! {
            <div>
                {"asdad"}
                <div class="gamebox" style={styles}>
                {
                    a
                }
                <MineGrid x={0} y={0}></MineGrid>
                <MineGrid x={0} y={1}></MineGrid>
                <MineGrid x={1} y={0}></MineGrid>
                <MineGrid x={1} y={1}></MineGrid>
                </div>
            </div>
        }
    }
}
