use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Shape, ShapeType, Color, Image};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{Stack, Offset, Size, Padding, OutlinedRectangle, Bin};

#[derive(Debug, Component)]
pub struct Sprite(Stack, Image, #[skip] String, #[skip] (Offset, Offset), #[skip] (f32, f32));
impl OnEvent for Sprite {}

impl Sprite {
    pub fn new(ctx: &mut Context, id: &str, path: &str, size: (f32, f32), offsets: (Offset, Offset)) -> Self {
        let bytes = ctx.assets.add_image(image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap().into());
        let image = Image{shape: ShapeType::Rectangle(0.0, (size.0, size.1)), image: bytes, color: None}; 
        Sprite(Stack::default(), image, id.to_string(), offsets, (0.0, 0.0))
    }

    pub fn dimensions(&mut self) -> &mut (f32, f32) { 
        match &mut self.1.shape {
            ShapeType::Ellipse(_, size) => size,
            ShapeType::Rectangle(_, size) => size,
            ShapeType::RoundedRectangle(_, size, _) => size,
        }
    }

    // fn offset(&mut self) -> (&mut Offset, &mut Offset) {(&mut self.2.0, &mut self.2.1)}
    pub fn id(&self) -> &String { &self.2 }

    pub fn offset(&self) -> &(Offset, Offset) { &self.3 }

    pub fn position(&mut self) -> &mut (f32, f32) { &mut self.4 }
}