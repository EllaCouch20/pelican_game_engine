use pelican_ui::{Context, Plugins, Plugin, Service, Services, ServiceList, maverick_start, start, Application, PelicanEngine, MaverickOS, HardwareContext};
use pelican_ui::drawable::Drawable;
use pelican_ui_std::{Interface, NavigateEvent};
use std::collections::BTreeMap;
use pelican_ui::include_assets;
use std::fs::{DirEntry, File};

mod pages;
use crate::pages::MyGame;

pub struct MyApp;
impl Services for MyApp {
    fn services() -> ServiceList {
        BTreeMap::new()
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

        let game = MyGame::new(ctx);
        Box::new(Interface::new(ctx, game, None))
    }
}

start!(MyApp);
