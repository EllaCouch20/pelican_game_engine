use pelican_ui::{Context, Plugins, Plugin, Service, Services, ServiceList, maverick_start, start, Application, PelicanEngine, MaverickOS, HardwareContext};
use pelican_ui::drawable::Drawable;
use pelican_ui::events::Event;
use std::collections::BTreeMap;
use pelican_ui::include_assets;

mod structs;
use crate::structs::{Gameboard, Sprite};

pub struct MyApp;
impl Services for MyApp {
    fn services() -> ServiceList {
        ServiceList(BTreeMap::new())
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![]
    }
}

impl Application for MyApp {
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        ctx.assets.include_assets(include_assets!("./assets"));

        let sprite_a = Sprite::new(ctx, "ship.png", (50.0, 50.0), (50.0, 50.0), |ctx: &mut Context, event: &mut dyn Event| {});
        let sprite_b = Sprite::new(ctx, "ship.png", (250.0, 250.0), (50.0, 50.0), |ctx: &mut Context, event: &mut dyn Event| {});

        let mut game = Gameboard::new(ctx);
        game.insert_sprite(sprite_a);
        game.insert_sprite(sprite_b);
        Box::new(game)
    }
}

start!(MyApp);
