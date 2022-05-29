use iced::text_input::{self, TextInput};
use iced::{Alignment, Column, Container, Element, Length, Sandbox, Settings, Text};
use rand::Rng;

type YCol = Vec<XRow>;
type XRow = Vec<u8>;
struct GameBox {
    width: u32,
    height: u32,
    mine_max: u32,
}

fn main() {
    let game_box = GameBox {
        width: 8,
        height: 8,
        mine_max: 8,
    };
    let mine_map: YCol = vec![];
    // 初始化
    let mut grids: Vec<u32> = vec![];
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

    print_map(&grids, &game_box);

    for y in 0..game_box.height {
        for x in 0..game_box.width {}
    }
}

fn print_map(mine_map: &Vec<u32>, game_box: &GameBox) {
    for y in 0..game_box.height {
        for x in 0..game_box.width {
            print!("{x} ");
        }
        println!("");
    }
}
fn _print_map(mine_map: &YCol) {
    for y in mine_map {
        for x in y {
            print!("{x} ");
        }
        println!("");
    }
}

fn rand(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}
