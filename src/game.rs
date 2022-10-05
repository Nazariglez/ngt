use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {}

pub fn initialize() -> Result<(), String> {
    let win_config = WindowConfig::default()
        .size(800, 600)
        .vsync(true)
        .title("{{ project-name }} - Notan Game");

    notan::init_with(|| State {})
        .add_config(DrawConfig)
        .add_config(win_config)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, _state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }
}

fn draw(gfx: &mut Graphics, _state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.rect((100.0, 100.0), (600.0, 400.0));

    gfx.render(&draw);
}
