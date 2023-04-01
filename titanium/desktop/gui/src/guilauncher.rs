use crate::{
    GuiInstance,
    libdruid
};

/// A struct that can be used to launch the GUI.
#[derive(Debug)]
pub struct GuiLauncher;
impl GuiLauncher {
    /// Launches the GUI with the given window and initial state.
    pub fn launch<State: libdruid::Data>(main_window: GuiInstance<State>) -> Result<(), libdruid::PlatformError> {
        let launcher = libdruid::AppLauncher::with_window(
            main_window.window
        );
        launcher.launch(
            main_window.initial_state.expect("You must provide an initial state to launch the application")
        )
    }

    /// Launches the GUI with the given window and initial state.
    pub fn custom<State: libdruid::Data>(main_window: GuiInstance<State>) -> (
        libdruid::AppLauncher<State>,
        State
    ) {
        let launcher = libdruid::AppLauncher::with_window(
            main_window.window
        );
        (
            launcher,
            main_window.initial_state.expect("You must provide an initial state to launch the application")
        )
    }
}