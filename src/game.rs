use notan::draw::*;
use notan::extra::FpsLimit;
use notan::math::{vec2, vec3, Mat4, Vec2};
use notan::prelude::*;

// This is the size of our content, no matter the size
// of the window our content will always keep this aspect ratio
const WORK_SIZE: Vec2 = vec2(800.0, 600.0);

// In case vsync is disabled by user's machine
// limit the fps to avoid consume too much resources
const LIMIT_MAX_FPS: u8 = 255;

#[derive(AppState)]
struct State {
    camera: Camera2D,
}

fn window_config() -> WindowConfig {
    WindowConfig::default()
        .size(WORK_SIZE.x as _, WORK_SIZE.y as _)
        .min_size((WORK_SIZE.x * 0.5) as _, (WORK_SIZE.y * 0.5) as _)
        .resizable(true)
        .vsync(true)
        .title("{{ project-name }} - Notan Game")
}

pub fn initialize() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .add_config(window_config())
        .add_plugin(FpsLimit::new(LIMIT_MAX_FPS))
        .update(update)
        .draw(draw)
        .build()
}

fn setup() -> State {
    let mut camera = Camera2D::new(WORK_SIZE.x, WORK_SIZE.y);
    camera.set_position_to_center();

    State {
        camera,
    }
}

fn update(app: &mut App, _state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }
}
fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    state.camera.apply(&mut draw);

    draw.clear(Color::BLACK);
    draw.rect((100.0, 100.0), (600.0, 400.0));

    gfx.render(&draw);
}