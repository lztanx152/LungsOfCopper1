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
    // 按照坐标选取，
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



}



fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
