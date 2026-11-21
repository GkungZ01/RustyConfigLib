pub mod binding;
pub mod category;
pub mod controller;
pub mod error;
pub mod group;
pub mod option;
pub mod storage;

pub use binding::{Binding, DirectBinding};
pub use category::ConfigCategory;
pub use controller::Controller;
pub use error::RclError;
pub use group::OptionGroup;
pub use option::OptionInstance;

/// Base trait for configuration models in RustyConfigLib
pub trait ConfigModel: serde::Serialize + serde::de::DeserializeOwned + Default {
    fn id() -> &'static str;
}
