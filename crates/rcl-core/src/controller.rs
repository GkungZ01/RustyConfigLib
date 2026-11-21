use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Controller<T> {
    TickBox,
    Slider {
        min: T,
        max: T,
        step: Option<T>,
        format_value: Option<fn(&T) -> String>,
    },
    Cycle {
        options: Vec<T>,
        format_value: Option<fn(&T) -> String>,
    },
    Dropdown {
        options: Vec<T>,
    },
    ColorPicker {
        allow_alpha: bool,
    },
    TextField,
}

impl<T> Controller<T> {
    pub fn tick_box() -> Self {
        Self::TickBox
    }

    pub fn slider(min: T, max: T, step: Option<T>) -> Self {
        Self::Slider {
            min,
            max,
            step,
            format_value: None,
        }
    }

    pub fn slider_formatted(min: T, max: T, step: Option<T>, formatter: fn(&T) -> String) -> Self {
        Self::Slider {
            min,
            max,
            step,
            format_value: Some(formatter),
        }
    }

    pub fn cycle(options: Vec<T>) -> Self {
        Self::Cycle {
            options,
            format_value: None,
        }
    }
}
