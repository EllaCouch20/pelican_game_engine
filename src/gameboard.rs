use pelican_ui::events::{Event, Key, KeyboardEvent, KeyboardState, NamedKey, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Shape, ShapeType, Color};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{Stack, Offset, Size, Padding, OutlinedRectangle, Bin};

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use indexmap::IndexMap;

use crate::Sprite;
use crate::CollisionEvent;

/// Aspect Ratio enumerator containing a few standard ratios. Is used with the background structure

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum AspectRatio {
    OneOne,
    TwoThree,
    FourFive,
    FiveSeven,
    #[default]
    SixteenNine,
}

impl AspectRatio {
    pub fn size(&self, screen_size: (f32, f32)) -> (f32, f32) {
        let (screen_w, screen_h) = screen_size;
        let mut aspect = match self {
            AspectRatio::OneOne => 1.0,
            AspectRatio::TwoThree => 2.0 / 3.0,
            AspectRatio::FourFive => 4.0 / 5.0,
            AspectRatio::FiveSeven => 5.0 / 7.0,
            AspectRatio::SixteenNine => 16.0 / 9.0,
        };

        match screen_w > screen_h {
            true if screen_h < screen_w * aspect => (screen_h * aspect, screen_h),
            false if screen_w < screen_h * aspect => (screen_w, screen_w * aspect),
            true => (screen_w, screen_w * aspect),
            false => (screen_h * aspect, screen_h),
        }
    }
}

/// Closure that will run when winit detects an event
type OnGameEvent = Box<dyn FnMut(&mut Gameboard, &mut Context, &mut dyn Event) -> bool>;

/// Gameboard structure that contains the background and all sprites. Triggers a provided OnGameEvent when an event happens
#[derive(Component)]
pub struct Gameboard(pub GameLayout, pub GameboardBackground, pub IndexMap<String, Sprite>, #[skip] Option<OnGameEvent>);

impl Gameboard {
    pub fn new(ctx: &mut Context, aspect_ratio: AspectRatio, on_event: OnGameEvent) -> Self {
        let colors = ctx.theme.colors;
        let background = GameboardBackground::new(ctx, 1.0, 8.0, colors.background.secondary, aspect_ratio);
        Gameboard(GameLayout::new(IndexMap::from([("background".to_string(), (Offset::Start, Offset::Start))]), aspect_ratio), background, IndexMap::new(), Some(on_event))
    }

    pub fn insert_sprite(&mut self, ctx: &mut Context, sprite: Sprite) {
        println!("Adding new sprite {:?}", sprite);
        self.0.0.insert(sprite.id().to_string(), *sprite.offset());
        self.2.insert(sprite.id().to_string(), sprite);
    }
}

impl OnEvent for Gameboard {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        let mut callback = self.3.take().expect("callback should be set");
        let result = callback(self, ctx, event);
        self.3 = Some(callback);
        result
    }
}

impl std::fmt::Debug for Gameboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gameboard")
    }
}

/// Game Layout takes the remaining children ignoring those tagged with
/// #[skip] and positions them on the screen using their corresponding offsets

#[derive(Debug, Default)]
pub struct GameLayout(pub IndexMap<String, (Offset, Offset)>, AspectRatio);

impl GameLayout {
    pub fn new(offsets: IndexMap<String, (Offset, Offset)>, ratio: AspectRatio) -> Self {
        GameLayout(offsets, ratio)
    }

    pub fn size(&self, ctx: &mut Context) -> (f32, f32) {
        ctx.state().get_or_default::<GameboardSize>().get()
    }
}

impl Layout for GameLayout {
    fn request_size(&self, ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
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
        children.into_iter().zip(self.0.clone().values()).map(|(c, offset)| {
            let size = c.get(new_size);
            let x = offset.0.get(new_size.0, size.0);
            let y = offset.1.get(new_size.1, size.1);
            Area{offset: (x, y), size}
        }).collect()
    }
}

/// Background that will keep aspect ratio

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
        ctx.state().set(GameboardSize(new_size.0, new_size.1));
        vec![Area { offset: (Offset::Start.get(size.0, new_size.0), Offset::Start.get(size.1, new_size.1)), size }]
    }
}

///Keeps track of gameboard size
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct GameboardSize(pub f32, pub f32);

impl GameboardSize{pub fn get(&self) -> (f32, f32) {(self.0, self.1)}}