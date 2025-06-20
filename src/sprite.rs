use pelican_ui::Context;
use pelican_ui::drawable::{Drawable, Image, ShapeType};
use pelican_ui_std::Offset;

pub trait Sprite: Drawable {
    fn size(&self, ctx: &mut Context) -> (f32, f32);
    fn position(&self, max_size: (f32, f32)) -> (f32, f32);
    fn offset(&self) -> (&mut Offset, &mut Offset);
    fn id(&self) -> uuid::Uuid;
}

pub struct SpriteImage(pub Image);

impl SpriteImage {
    pub fn new(ctx: &mut Context, path: &str, size: (f32, f32)) -> Self {
        let bytes = ctx.assets.add_image(image::load_from_memory(&ctx.assets.load_file(path).unwrap()).unwrap().into());
        let image = Image{shape: ShapeType::Rectangle(0.0, (size.0, size.1)), image: bytes, color: None}; 
        SpriteImage(image)
    }
}

// pub fn size(&self, ctx: &mut Context) -> (f32, f32) {
//     let sw = self.size_request(ctx);
//     (sw.max_width(), sw.max_height())
// }

// pub fn position(&self, max_size: (f32, f32)) -> (f32, f32) {
//     let (minw, minh, maxw, maxh) = self.request_size(ctx);
//     let x = self.0.0.get(max_size.0, maxw);
//     let y = self.0.1.get(max_size.1, maxh);
//     (x, y)
// }

// pub fn offset(&self) -> (&mut Offset, &mut Offset) {
//     (&mut self.0.0, &mut self.0.1)
// }