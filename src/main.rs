use nimbus::{
    components::{color::Color, rect::Rect, timer::Timer},
    math::{UVec2, Vec2, Vec3},
    window::WindowDescriptor,
    Engine, Nimbus,
};

pub struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

impl State {}

impl Nimbus for State {
    fn init(&mut self, engine: &mut nimbus::Engine) {}

    fn update(&mut self, engine: &mut nimbus::Engine, delta: f32) {}

    fn render(&mut self, renderer: &mut nimbus::renderer::Renderer, delta: f32) {}
}

fn main() {
    let app = Engine::new(WindowDescriptor {
        // render_resolution: Some(UVec2 { x: 320, y: 180 }),
        ..Default::default()
    });

    let game = State::default();

    app.run(game);
}
