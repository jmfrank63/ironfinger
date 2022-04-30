use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Screen};
use druid::widget::Label;

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}

fn main() -> Result<(), PlatformError> {

    let rect = Screen::get_display_rect();

    println!("{:?}", rect);
    let main_window = WindowDesc::new(build_ui())
        .title("Typewriter")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window).launch(())?;
    Ok(())
}
