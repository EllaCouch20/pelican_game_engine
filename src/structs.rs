use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Align, Image, ShapeType};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{Stack, Offset, Size, Padding, OutlinedRectangle};

#[derive(Debug, Component)]
pub struct Gameboard(Stack, OutlinedRectangle, Vec<Sprite>);

impl Gameboard {
    pub fn new(ctx: &mut Context) -> Self {
        let colors = ctx.theme.colors;
        let background = OutlinedRectangle::new(colors.background.primary, colors.outline.secondary, 8.0, 1.0);
        let width = Size::custom(move |_: Vec<(f32, f32)>| (0.0, f32::MAX));
        let height = Size::custom(move |_: Vec<(f32, f32)>| (0.0, f32::MAX));
        Gameboard(Stack(Offset::Center, Offset::Center, width, height, Padding::new(10.0)), background, Vec::new())
    }

    pub fn insert_sprite(&mut self, sprite: Sprite) {
        self.2.push(sprite);
    }
}

impl OnEvent for Gameboard {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let children = &self.2;

            children.iter().enumerate().for_each(|(i, a)| {
                let (ax, ay) = a.position();
                let (aw, ah) = a.size();

                children.iter().skip(i + 1).for_each(|b| {
                    let (bx, by) = b.position();
                    let (bw, bh) = b.size();

                    if  ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by {
                        ctx.trigger_event(CollisionEvent(a.3));
                        ctx.trigger_event(CollisionEvent(b.3));
                    }
                });
            });
        }
        true
    }
}

#[derive(Component)]
pub struct Sprite(Stack, Image, #[skip] Box<dyn FnMut(&mut Context, &mut dyn Event)>, #[skip] uuid::Uuid);

impl Sprite {
    pub fn new(ctx: &mut Context, path: &str, pos: (f32, f32), size: (f32, f32), on_event: impl FnMut(&mut Context, &mut dyn Event) + 'static) -> Self {
        let image = ctx.assets.add_image(image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap().into());
        let sprite = Image{shape: ShapeType::Rectangle(0.0, (size.0, size.1)), image, color: None}; 
        Sprite(Stack(Offset::Static(pos.0), Offset::Static(pos.1), Size::Static(size.0), Size::Static(size.1), Padding::default()), sprite, Box::new(on_event), uuid::Uuid::new_v4())
    }

    pub fn size(&self) -> (f32, f32) {
        let w = if let Size::Static(a) = self.0.2 {a} else {0.0};
        let h = if let Size::Static(a) = self.0.3 {a} else {0.0};
        (w, h)
    }

    pub fn position(&self) -> (f32, f32) {
        let x = if let Offset::Static(a) = self.0.0 {a} else {0.0};
        let y = if let Offset::Static(a) = self.0.1 {a} else {0.0};
        (x, y)
    }
}

impl OnEvent for Sprite {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        // println!("EVENT TRIGGERED {:?}", event);
        (self.2)(ctx, event);
        true
    }
}

impl std::fmt::Debug for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sprite")
    }
}

#[derive(Debug, Clone)]
pub struct CollisionEvent(pub uuid::Uuid);

impl Event for CollisionEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}