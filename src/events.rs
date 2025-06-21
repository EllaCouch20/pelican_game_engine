use pelican_ui::Context;
use pelican_ui::events::Event;

#[derive(Debug, Clone)]
pub struct CollisionEvent(pub u32);

impl Event for CollisionEvent {
    fn pass(self: Box<Self>, _ctx: &mut Context, children: Vec<((f32, f32), (f32, f32))>) -> Vec<Option<Box<dyn Event>>> {
        children.into_iter().map(|_| Some(self.clone() as Box<dyn Event>)).collect()
    }
}
