use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Shape, ShapeType, Color, Image};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{Stack, Offset, Size, Padding, OutlinedRectangle, Bin};

use crate::GameboardSize;

#[derive(Debug, Component)]
pub struct Sprite(Stack, Image, #[skip] String, #[skip] (Offset, Offset), #[skip] (f32, f32));
impl OnEvent for Sprite {}

impl Sprite {
    pub fn new(ctx: &mut Context, id: &str, path: &'static str, size: (f32, f32), offsets: (Offset, Offset)) -> Self {
        let image = ctx.theme.brand.illustrations.get(path).unwrap();
        let image = Image{shape: ShapeType::Rectangle(0.0, (size.0, size.1)), image, color: None}; 
        Sprite(Stack::default(), image, id.to_string(), offsets, (0.0, 0.0))
    }

    pub fn dimensions(&mut self) -> &mut (f32, f32) { 
        match &mut self.1.shape {
            ShapeType::Ellipse(_, size) => size,
            ShapeType::Rectangle(_, size) => size,
            ShapeType::RoundedRectangle(_, size, _) => size,
        }
    }

    pub fn id(&self) -> &String { &self.2 }

    // This is not the total position, just the original offsets provided
    pub fn offset(&self) -> &(Offset, Offset) { &self.3 }

    // this is the total position
    pub fn position(&mut self, ctx: &mut Context) -> (f32, f32) {
        let max = ctx.state().get_or_default::<GameboardSize>().get();
        let pos = self.adjustments().clone();
        let dims = self.dimensions().clone();
        let x = self.offset().0.get(dims.0, max.0).abs() + pos.0;
        let y = self.offset().1.get(dims.1, max.1).abs() + pos.1;
        (x, y)
    }

    // This is not the total position, just the adjustments to be made to the offsets
    pub fn adjustments(&mut self) -> &mut (f32, f32) { &mut self.4 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteAction {
    Hurt,
    Die,
    Shoot,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SpriteState {
    #[default]
    Idle,
    MovingLeft,
    MovingRight,
    MovingUp,
    MovingDown,
}