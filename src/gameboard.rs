use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Shape, ShapeType, Color};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{Stack, Offset, Size, Padding, OutlinedRectangle, Bin};

use serde::{Serialize, Deserialize};

use crate::Sprite;
use crate::CollisionEvent;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct GameboardSize(pub f32, pub f32);

#[derive(Debug, Component)]
pub struct Gameboard(GameGrid, GameboardBackground, Vec<Box<dyn Sprite>>);

impl Gameboard {
    pub fn new(ctx: &mut Context, aspect_ratio: AspectRatio) -> Self {
        let colors = ctx.theme.colors;
        let background = GameboardBackground::new(ctx, 1.0, 8.0, colors.background.secondary, aspect_ratio);
        Gameboard(GameGrid(vec![(Offset::Center, Offset::Center)], aspect_ratio), background, Vec::new())
    }

    pub fn insert_sprite(&mut self, mut sprite: impl Sprite) {
        let (ow, oh) = sprite.offset();
        self.0.0.push((*ow, *oh));
        self.2.push(Box::new(sprite));
    }
}

impl OnEvent for Gameboard {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let GameboardSize(maxw, maxh) = ctx.state().get::<GameboardSize>();
            let children = &self.2;

            children.iter().enumerate().for_each(|(i, a)| {
                let (ax, ay) = a.position((maxw, maxh));
                let (aw, ah) = a.dimensions();

                children.iter().skip(i + 1).for_each(|b| {
                    let (bx, by) = a.position((maxw, maxh));
                    let (bw, bh) = b.dimensions();

                    if  ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by {
                        ctx.trigger_event(CollisionEvent(a.id()));
                        ctx.trigger_event(CollisionEvent(b.id()));
                    }
                });
            });
        }
        true
    }
}

#[derive(Debug)]
pub struct GameboardBackground(Shape, AspectRatio);

impl GameboardBackground {
    pub fn shape(&mut self) -> &mut Shape { &mut self.0 }
    pub fn new(ctx: &mut Context, s: f32, r: f32, color: Color, ratio: AspectRatio) -> Self {
        GameboardBackground(Shape{shape: ShapeType::RoundedRectangle(s, (0.0, 0.0), r), color}, ratio)
    }
}

impl OnEvent for GameboardBackground {}

impl Component for GameboardBackground {
    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {vec![&mut self.0]}
    fn children(&self) -> Vec<&dyn Drawable> {vec![&self.0]}
    fn request_size(&self, _ctx: &mut Context, _children: Vec<SizeRequest>) -> SizeRequest {
        SizeRequest::fill()
    }

    fn build(&mut self, ctx: &mut Context, size: (f32, f32), _children: Vec<SizeRequest>) -> Vec<Area> {
        let new_size = self.1.size(size);
        if let ShapeType::RoundedRectangle(_, s, _) = &mut self.0.shape { *s = new_size; }
        ctx.state().set(&GameboardSize(new_size.0, new_size.1));
        vec![Area { offset: (Offset::Start.get(size.0, new_size.0), Offset::Start.get(size.1, new_size.1)), size }]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AspectRatio {
    OneOne,
    TwoThree,
    FourFive,
    FiveSeven,
    SixteenNine,
}

impl AspectRatio {
    pub fn size(&self, size: (f32, f32)) -> (f32, f32) {
        let max = size.0.max(size.1);
        let min = size.0.min(size.1);
        match self {
            AspectRatio::OneOne => (min, min),
            AspectRatio::TwoThree => (size.0 > size.1).then(|| (max, max * 0.6667)).unwrap_or_else(|| (max * 0.6667, max)),
            AspectRatio::FourFive => (size.0 > size.1).then(|| (max, max * 0.8)).unwrap_or_else(|| (max * 0.8, max)),
            AspectRatio::FiveSeven => (size.0 > size.1).then(|| (max, max * 0.7143)).unwrap_or_else(|| (max * 0.7143, max)),
            AspectRatio::SixteenNine => (size.0 > size.1).then(|| (max, max * 0.5625)).unwrap_or_else(|| (max * 0.5625, max))
        }
    }
}

#[derive(Debug)]
pub struct GameGrid(Vec<(Offset, Offset)>, AspectRatio);

impl Layout for GameGrid {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        let (widths, heights): (Vec<_>, Vec<_>) = children.into_iter().map(|i|
            ((i.min_width(), i.max_width()), (i.min_height(), i.max_height()))
        ).unzip();
        let width = widths.into_iter().fold((f32::MAX, f32::MIN), |(min_w, max_w), (w_min, w_max)| {
            (min_w.min(w_min), max_w.max(w_max))
        });
        let height = heights.into_iter().fold((f32::MAX, f32::MIN), |(min_h, max_h), (h_min, h_max)| {
            (min_h.min(h_min), max_h.max(h_max))
        });
        SizeRequest::new(width.0, height.0, width.1, height.1)
    }

    fn build(&self, _ctx: &mut Context, max_size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        let new_size = self.1.size(max_size);
        children.into_iter().zip(self.0.clone()).map(|(c, offset)| {
            let size = c.get(new_size);
            let x = offset.0.get(new_size.0, size.0);
            let y = offset.1.get(new_size.1, size.1);
            Area{offset: (x, y), size}
        }).collect()
    }
}