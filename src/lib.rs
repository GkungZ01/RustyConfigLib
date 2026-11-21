//! RustyConfigLib - Configuration & GUI library and mod for Horizon
pub use rcl_core as core;
pub use rcl_gui as gui;
pub use rcl_macros as macros;

pub use rcl_core::*;

use horizon_mod_api as mod_api;

#[derive(Default)]
pub struct RustyConfigLibMod;

#[cfg(not(test))]
#[mod_api::horizon_mod]
impl mod_api::Hooks for RustyConfigLibMod {
    fn register(
        &self,
        _registrar: &mut dyn mod_api::Registrar,
        _context: &mod_api::RegistrationContext,
    ) {
        // Library initialization hooks
    }
}
