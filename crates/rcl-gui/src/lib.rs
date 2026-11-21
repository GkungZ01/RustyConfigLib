pub trait GuiRenderer {
    fn render_button(&mut self, label: &str, x: f32, y: f32, width: f32, height: f32);
}

pub struct Screen;
