use bevy::color::palettes::css::{DARK_GREEN, LIME, ORANGE, ORANGE_RED, RED, SEA_GREEN, YELLOW};
use bevy::prelude::*;
use bevy::text::LineHeight;
use rand::seq::SliceRandom;
use std::ops::Range;
const TILE_SIZE: f32 = 30.0;
const TILE_GAP: f32 = 6.0;
const BOARD_PADDING: f32 = 15.0;
// 常量区
const BACKGROUND_COLOR: Color = Color::Srgba(DARK_GREEN);
const TILE_BORDER_COLOR: Color = Color::Srgba(SEA_GREEN);
const HOVERED_TILE_BORDER_COLOR: Color = Color::Srgba(LIME);
const MILD_DANGER_COLOR: Color = Color::Srgba(YELLOW);
const SOME_DANGER_COLOR: Color = Color::Srgba(ORANGE);
const VERY_DANGER_COLOR: Color = Color::Srgba(ORANGE_RED);
const MOST_DANGER_COLOR: Color = Color::Srgba(RED);

// 格子方块，是否有地雷，是否被扫清，是否被标记
#[derive(Default, Clone, Copy)]
struct Tile {
    mined: bool,
    revealed: bool,
    flagged: bool,
}

// 雷区
struct MineField {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl MineField {

    // 宽度
    fn xs(&self) -> Range<i32> {
        0..self.width
    }
    // 雷区高度
    fn ys(&self) -> Range<i32> {
        0..self.height
    }
    // 二维坐标转一维坐标
    fn get(&self, p: IVec2) -> Option<usize> {
        if self.xs().contains(&p.x) && self.ys().contains(&p.y) {
            Some((p.y * self.width + p.x) as usize)
        } else {
            None
        }
    }

    fn new(width: i32, height: i32, mines: usize) -> Self {
        let mut tiles = vec![Tile::default(); (width * height) as usize];
        // 把前mines个位置设置为有地雷
        tiles
            .iter_mut()
            .take(mines)
            .for_each(|tile| tile.mined = true);
        // 打乱顺序
        tiles.shuffle(&mut rand::rng());

        Self {
            width,
            height,
            tiles,
        }
    }

    // 获取相邻元素的真实坐标
    fn get_adjacent(&self, target: IVec2) -> impl Iterator<Item = IVec2> + '_ {
        [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ]
            .map(|d| target + IVec2::from(d))
            .into_iter()
            .filter(|adjacent| self.get(*adjacent).is_some())
    }
    // 统计相邻的地雷数量
    fn count_adjacent_mines(&self, target: IVec2) -> usize {
        self.get_adjacent(target)
            .filter(|&adjacent| self[adjacent].mined)
            .count()
    }

    // 迭代器，返回二维坐标+地块
    fn iter(&self) -> impl Iterator<Item = (IVec2, Tile)> + '_ {
        self.ys()
            .flat_map(|y| self.xs().map(move |x| IVec2::new(x, y)))
            .map(|tile| (tile, self[tile]))
    }

    // 统计插旗地块数量
    fn flag_count(&self) -> usize {
        self.tiles.iter().filter(|tile| tile.flagged).count()
    }
    // 统计地雷地块数量
    fn mine_count(&self) -> usize {
        self.tiles.iter().filter(|tile| tile.mined).count()
    }
    // 地雷区是否被完全清理，&&优先级高于||，即：无地雷的地块需要被揭开，有地雷的地块需要被插旗，复杂度较高
    fn is_cleared(&self) -> bool {
        self.tiles
            .iter()
            .all(|tile| tile.revealed && !tile.mined || tile.mined && tile.flagged)
    }
}

// 对minefield类型实现二维索引
impl std::ops::Index<IVec2> for MineField {
    type Output = Tile;

    fn index(&self, tile: IVec2) -> &Self::Output {
        let index = self.get(tile).unwrap();
        &self.tiles[index]
    }
}


fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
