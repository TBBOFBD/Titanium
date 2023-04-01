use crate::{libdruid, default};

/// A struct that can be used to create a GUI instance.
pub struct GuiInstance<State> {
    pub(crate) window: libdruid::WindowDesc<State>,
    pub(crate) initial_state: Option<State>,
    pub(crate) env: Option<Box<dyn FnOnce(&mut druid::Env, &State)>>
}

impl std::fmt::Debug for GuiInstance<()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuiInstance")
            .finish()
    }
}

impl<State: libdruid::Data> GuiInstance<State> {
    /// Creates a new GUI instance with the given root widget.
    pub fn new<W>(root: W) -> Self
    where
        W: libdruid::Widget<State> + 'static,
    {
        let window = libdruid::WindowDesc::new(root)
            .title(titaniumcommon::__internals__::DEFAULT_APP_NAME)
            .resizable(false)
            .with_min_size((50.0, 50.0));
        Self {
            window,
            initial_state: None,
            env: None
        }
    }

    /// Sets the settings for the window.
    pub fn window(mut self, f: impl FnOnce(libdruid::WindowDesc<State>) -> libdruid::WindowDesc<State>) -> Self {
        self.window = f(self.window);
        self
    }

    /// Sets the initial state of the GUI.
    pub fn with_initial_state(mut self, state: State) -> Self {
        self.initial_state = Some(state);
        self
    }

    /// Configures the environment variables of the GUI.
    pub fn configure_env(mut self, f: impl Fn(&mut druid::Env, &State) + 'static) -> Self {
        self.env = Some(Box::new(f));
        self
    }
}

impl Default for GuiInstance<default::AppState> {
    fn default() -> Self {
        crate::default::new()
    }
}