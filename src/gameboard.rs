use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};

use pelican_ui_std::{Stack, Offset, Size, Padding, OutlinedRectangle};

use crate::Sprite;
use crate::CollisionEvent;

#[derive(Debug, Component)]
pub struct Gameboard(Stack, OutlinedRectangle, Vec<Box<dyn Sprite>>);

impl Gameboard {
    pub fn new(ctx: &mut Context) -> Self {
        let colors = ctx.theme.colors;
        let background = OutlinedRectangle::new(colors.background.primary, colors.outline.secondary, 8.0, 1.0);
        let width = Size::custom(move |_: Vec<(f32, f32)>| (0.0, f32::MAX));
        let height = Size::custom(move |_: Vec<(f32, f32)>| (0.0, f32::MAX));
        Gameboard(Stack(Offset::Center, Offset::Center, width, height, Padding::new(10.0)), background, Vec::new())
    }

    pub fn insert_sprite(&mut self, sprite: impl Sprite) {
        self.2.push(Box::new(sprite));
    }
}

impl OnEvent for Gameboard {
    fn on_event(&mut self, ctx: &mut Context, event: &mut dyn Event) -> bool {
        if let Some(TickEvent) = event.downcast_ref::<TickEvent>() {
            let (maxw, maxh) = self.1.size();
            let children = &self.2;

            children.iter().enumerate().for_each(|(i, a)| {
                let (ax, ay) = a.position((maxw, maxh));
                let (aw, ah) = a.size(ctx);

                children.iter().skip(i + 1).for_each(|b| {
                    let (bx, by) = a.position((maxw, maxh));
                    let (bw, bh) = b.size(ctx);

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