#[derive(Debug, Clone)]
pub enum Controller<T> {
    TickBox,
    Slider { min: T, max: T, step: Option<T> },
    Dropdown { options: Vec<T> },
    ColorPicker { allow_alpha: bool },
    TextField,
}
