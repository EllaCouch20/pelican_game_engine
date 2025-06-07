use pelican_ui::events::{Event, OnEvent, TickEvent};
use pelican_ui::drawable::{Drawable, Component, Align, Image, ShapeType};
use pelican_ui::layout::{Area, SizeRequest, Layout};
use pelican_ui::{Context, Component};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};

use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;

use crate::gamegrid::GameGrid;

use pelican_ui_std::{
    Padding, Offset, Size,
    RoundedRectangle,
    Row, Bin, Column, Page, Stack,
    Header, Content, AppPage,
};

pub const BOARD_SIZE: usize = 9;
pub const SQUARE_SIZE: f32 = 40.0;

#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {
    Player,
    None
}

impl Entity {
    pub fn build(&self, ctx: &mut Context) -> Sprite {
        match self {
            Entity::Player => Ship::new(ctx),
            Entity::None => Sprite::empty(ctx)
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GameState {
    pub sprites: HashMap<String, Entity>, 
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut sprites = HashMap::new();
        sprites.insert(Coords(4, 8).to_string(), Entity::Player);

        GameState {
            sprites,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Coords(pub usize, pub usize);

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

impl FromStr for Coords {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Input must be in format `x,y`".into());
        }

        let x = parts[0].trim().parse::<usize>().map_err(|e| format!("Invalid x: {}", e))?;
        let y = parts[1].trim().parse::<usize>().map_err(|e| format!("Invalid y: {}", e))?;

        Ok(Coords(x, y))
    }
}

#[derive(Debug, Component, AppPage)]
pub struct MyGame(Stack, Page, #[skip] bool);

impl MyGame {
    pub fn new(ctx: &mut Context) -> Self {
        let game_state = GameState::new(ctx);
        ctx.state().set(&game_state);
        println!("game_state {:?}", game_state);

        let content = Content::new(Offset::Center, vec![Box::new(GameGrid::new(ctx, game_state))]);
        let header = Header::stack(ctx, None, "My Game", None);

        MyGame(Stack::default(), Page::new(header, content, None), false)
    }
}

impl OnEvent for MyGame {}

pub struct Ship;
impl Ship {
    pub fn new(ctx: &mut Context) -> Sprite {
        Sprite::new(ctx, "ship.png", SQUARE_SIZE)
    }
}

#[derive(Debug, Component)]
pub struct Sprite(Stack, Option<Image>, Option<Bin<Stack, RoundedRectangle>>); // image, background
impl OnEvent for Sprite {}
impl Sprite {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(ctx: &mut Context, path: &str, size: f32) -> Sprite {
        let img = image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap();
        Sprite(Stack::center(), Some(Image{shape: ShapeType::Rectangle(0.0, (size, size)), image: ctx.assets.add_image(img.into()), color: None}), None)
    }

    pub fn empty(ctx: &mut Context) -> Sprite {
        let color = ctx.theme.colors.background.secondary;
        Sprite(Stack::center(), None, Some(Bin(
            Stack(Offset::Center, Offset::Center, Size::Static(SQUARE_SIZE), Size::Static(SQUARE_SIZE), Padding::default()),
            RoundedRectangle::new(0.0, 4.0, color)
        )))
    }
}
