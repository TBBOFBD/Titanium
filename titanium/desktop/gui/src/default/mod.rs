use crate::GuiInstance;

use druid::{
    widget::{
        prelude::*,
        Controller,
        FillStrat,
        Image,
        Flex,
    },
    WidgetExt,
    Data,
    Lens, ImageBuf,
};

/// The default AppState for the default GUI implementation.
#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {}

struct DragController;

impl<T, W: Widget<T>> Controller<T, W> for DragController {
    fn event(
        &mut self,
        _child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        _data: &mut T,
        _env: &Env,
    ) {
        if let Event::MouseMove(_) = event {
            ctx.window().handle_titlebar(true);
        }
    }
}

fn render() -> impl Widget<AppState> {
    let png = titaniumutils::assets::ICON_PNG;
    let png_data = ImageBuf::from_data(&png).unwrap();

    let img = Image::new(png_data).fill_mode(FillStrat::Contain);
    Flex::column()
        .with_flex_child(img, 1.0)
        .controller(DragController)
}

pub(crate) fn new() -> GuiInstance<AppState> {
    let state = AppState {};

    let main_window = GuiInstance::new(render())
        .window(|window| {
            window
            .show_titlebar(false)
            .window_size((400.0, 400.0))
            .resizable(true)
        })
        .with_initial_state(state);
    
    main_window
}