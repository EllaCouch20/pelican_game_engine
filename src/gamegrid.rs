use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Align, Image, ShapeType};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};

use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;

use pelican_ui_std::{Padding, Offset, Size};
use crate::pages::{Entity, Coords, BOARD_SIZE, GameState, Sprite};

#[derive(Debug)]
pub struct Grid {
    rows: usize,
    cols: usize,
    spacing: (f32, f32), // (horizontal, vertical)
    offset: Offset,
    padding: Padding,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, spacing: (f32, f32), offset: Offset, padding: Padding) -> Self {
        Grid {
            rows,
            cols,
            spacing,
            offset,
            padding,
        }
    }

    pub fn square(count: usize, spacing: f32) -> Self {
        // let size = (count as f32).sqrt().ceil() as usize;
        Self::new(count, count, (spacing, spacing), Offset::Center, Padding::default())
    }
}

impl Layout for Grid {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let mut col_widths = vec![0.0f32; self.cols];
        let mut row_heights = vec![0.0f32; self.rows];

        for (i, child) in children.iter().enumerate() {
            let col = i % self.cols;
            let row = i / self.cols;
            if row < self.rows {
                col_widths[col] = col_widths[col].max(child.max_width());
                row_heights[row] = row_heights[row].max(child.max_height());
            }
        }

        let total_width = col_widths.iter().sum::<f32>() + self.spacing.0 * (self.cols - 1) as f32;
        let total_height = row_heights.iter().sum::<f32>() + self.spacing.1 * (self.rows - 1) as f32;

        self.padding.adjust_request(SizeRequest::fixed((total_width, total_height)))
    }


    fn build(&self, _ctx: &mut Context, size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let size = self.padding.adjust_size(size);
        let mut col_widths: Vec<f32> = vec![0.0; self.cols];
        let mut row_heights: Vec<f32> = vec![0.0; self.rows];

        for (i, child) in children.iter().enumerate() {
            let col = i % self.cols;
            let row = i / self.cols;
            if row < self.rows {
                col_widths[col] = col_widths[col].max(child.max_width());
                row_heights[row] = row_heights[row].max(child.max_height());
            }
        }

        let mut areas = Vec::with_capacity(children.len());
        let mut y = 0.0;

        for row in 0..self.rows {
            let mut x = 0.0;
            for col in 0..self.cols {
                let idx = row * self.cols + col;
                if idx >= children.len() {
                    continue;
                }

                let w = col_widths[col];
                let h = row_heights[row];
                let child_size = children[idx].get((w, h));
                let offset = self.padding.adjust_offset((x, y));
                areas.push(Area { offset, size: child_size });

                x += w + self.spacing.0;
            }
            y += row_heights[row] + self.spacing.1;
        }

        areas
    }
}


#[derive(Debug, Component)]
pub struct GameGrid(Grid, Vec<Sprite>);
impl OnEvent for GameGrid {}
impl GameGrid {
    pub fn new(ctx: &mut Context, game_state: GameState) -> Self {
        let mut items = Vec::new();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let coord = Coords(x, y);
                if let Some(sprite) = game_state.sprites.get(&coord.to_string()) {
                    items.push(*Box::new(sprite.build(ctx)));
                } else {
                    items.push(*Box::new(Entity::None.build(ctx)));
                }
            }
        }

        GameGrid(Grid::square(BOARD_SIZE, 4.0), items)
    }
}
