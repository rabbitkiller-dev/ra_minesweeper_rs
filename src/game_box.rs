use std::vec;

use rand::Rng;
use yew::prelude::*;
use yew::{html, virtual_dom::VNode, Html};

use crate::grid::MineGrid;

#[derive(PartialEq)]
pub enum GameStatus {
    Play,
    Win,
    Over,
}
#[derive(PartialEq)]
pub enum GridStatus {
    None,      // 没打开格子
    Mine,      // 地雷
    OpenMine,  // 打开了地雷
    Open(u32), // 打开的格子
}

#[derive(PartialEq)]
pub struct GameBox {
    pub width: u32,
    pub height: u32,
    pub mine_max: u32,
    pub status: GameStatus,
    pub mine_map: Vec<Vec<u32>>,
    pub flag_map: Vec<Vec<u32>>,
    pub label_map: Vec<Vec<GridStatus>>,
}
impl GameBox {
    pub fn new(width: u32, height: u32, mine_max: u32) -> Self {
        let mut game_box = GameBox {
            width,
            height,
            mine_max,
            status: GameStatus::Play,
            mine_map: vec![],
            flag_map: vec![],
            label_map: vec![],
        };
        game_box.new_game();
        game_box
    }

    pub fn open_grid(&mut self, x: u32, y: u32) {
        if self.is_flag(x, y) {
            return;
        }
        log::info!("打开格子");
        // 点到了地雷, 游戏结束
        if self.is_mine(x, y) {
            self.game_over(x, y);
            return;
        }
        let label = &mut self.label_map[y as usize][x as usize];
        // 格子已经打开
        if let GridStatus::Open(num) = label {
            let mut num = num.clone();
            self.query_around_grid(x, y).into_iter().for_each(|(x, y)| {
                if self.is_flag(x, y) {
                    num -= 1;
                }
            });
            // num等于0时代表周围的雷被玩家用旗子标出来了
            if num == 0 {
                self.query_around_grid(x, y).into_iter().for_each(|(x, y)| {
                    if self.is_flag(x, y) {
                        return
                    }
                    let label = &self.label_map[y as usize][x as usize];
                    // 只打开还没打开过的格子
                    if let GridStatus::None = label {
                        self.open_grid(x, y)
                    }
                });
            }
        }

        let mine = self.query_around_mine(x, y);
        let label = &mut self.label_map[y as usize][x as usize];
        *label = GridStatus::Open(mine);

        // 如果周围有雷，就不能自动打开周围的格子
        if mine > 0 {
            return;
        }

        let grids = self.query_around_grid(x, y);
        grids.into_iter().for_each(|(x, y)| {
            let label = &self.label_map[y as usize][x as usize];
            // 只打开还没打开过的格子
            if let GridStatus::None = label {
                self.open_grid(x, y)
            }
        });

        if self.surplus_grid() == 0 {
            self.game_win();
        }

    }

    pub fn flag_grid(&mut self, x: u32, y: u32) {
        let label = &self.label_map[y as usize][x as usize];
        // 只有空状态时才能标成旗子
        if let GridStatus::None = label {
            if self.flag_map[y as usize][x as usize] == 1 {
                self.flag_map[y as usize][x as usize] = 0
            } else {
                self.flag_map[y as usize][x as usize] = 1;
            }
        }
    }

    pub fn new_game(&mut self) {
        self.status = GameStatus::Play;
        self.mine_map = self.init_mine();
        let mut label_map = vec![];
        let mut flag_map = vec![];
        // 初始化显示地址
        for _ in 0..self.height {
            let mut line: Vec<GridStatus> = vec![];
            let mut flag_line: Vec<u32> = vec![];
            for _ in 0..self.width {
                line.push(GridStatus::None);
                flag_line.push(0);
            }
            label_map.push(line);
            flag_map.push(flag_line);
        }
        self.label_map = label_map;
        self.flag_map = flag_map;
    }

    fn game_over(&mut self, x: u32, y: u32) {
        let label = &mut self.label_map[y as usize][x as usize];
        *label = GridStatus::OpenMine;
        self.status = GameStatus::Over;

        // 初始化显示地址
        for y in 0..self.height {
            for x in 0..self.width {
                let label = &mut self.label_map[y as usize][x as usize];
                match label {
                    GridStatus::None => {
                        let is_mine = self.mine_map[y as usize][x as usize];
                        // 点到了地雷, 游戏结束
                        if is_mine == 1 {
                            self.label_map[y as usize][x as usize] = GridStatus::Mine;
                            continue;
                        }
                        let mine = self.query_around_mine(x, y);
                        self.label_map[y as usize][x as usize] = GridStatus::Open(mine);
                    }
                    GridStatus::Mine => {}
                    GridStatus::OpenMine => {}
                    GridStatus::Open(_) => {}
                }
            }
        }
    }

    fn game_win(&mut self) {
        self.status = GameStatus::Win;
    }

    pub fn query_around_grid(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut grids: Vec<(u32, u32)> = vec![];

        let li: [i32; 3] = [-1, 0, 1];

        for i in li {
            if i == -1 && y == 0 {
                continue;
            }
            let ty = (y as i32 + i) as u32;
            if let Some(line) = self.mine_map.get(ty as usize) {
                for j in li {
                    if j == -1 && x == 0 {
                        continue;
                    }
                    let tx = (x as i32 + j) as u32;
                    if x == tx && y == ty {
                        continue;
                    }
                    if let Some(_) = line.get(tx as usize) {
                        grids.push((tx as u32, ty as u32));
                    }
                }
            }
        }
        log::info!("{:?}", grids);

        grids
    }

    pub fn query_around_mine(&self, x: u32, y: u32) -> u32 {
        let grids = self.query_around_grid(x, y);
        let mut i = 0;
        for (x, y) in grids.into_iter() {
            let is_mine = self.mine_map[y as usize][x as usize];
            if is_mine == 1 {
                i += 1;
            }
        }
        i
    }

    pub fn is_mine(&self, x: u32, y: u32) -> bool {
        let is_mine = self.mine_map[y as usize][x as usize];
        is_mine == 1
    }
    pub fn is_flag(&self, x: u32, y: u32) -> bool {
        let is_flag = self.flag_map[y as usize][x as usize];
        is_flag == 1
    }

    pub fn surplus_grid(&self) -> u32 {
        let mut grid = self.height * self.width - self.mine_max;
        for y in 0..self.height {
            for x in 0..self.width {
                let label = &self.label_map[y as usize][x as usize];
                if let GridStatus::Open(_) = label {
                    grid -= 1;
                }
            }
        }
        grid
    }
    pub fn surplus_mine(&self) -> u32 {
        let mut grid = self.mine_max;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_flag(x, y) {
                    grid -= 1;
                }
            }
        }
        grid
    }
    // 生成地雷图
    fn init_mine(&self) -> Vec<Vec<u32>> {
        // 初始化
        let mut grids: Vec<u32> = vec![];
        let max = self.height * self.width;
        for _ in 0..self.mine_max {
            grids.push(1);
        }
        for _ in self.mine_max..max {
            grids.push(0);
        }
        println!("{:?}", grids);
        let mut i = grids.len() - 1;
        while i > 0 {
            grids.swap(GameBox::rand(0, i), i);
            i -= 1;
        }
        // 变成二维数组
        let mut mine_map: Vec<Vec<u32>> = vec![];
        for y in 0..self.height {
            let mut line: Vec<u32> = vec![];
            for x in 0..self.width {
                line.push(grids.remove(0));
            }
            mine_map.push(line);
        }
        mine_map
    }

    // 生成随机数
    fn rand(start: usize, end: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(start..end)
    }
}
