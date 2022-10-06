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
struct State {}

fn window_config() -> WindowConfig {
    WindowConfig::default()
        .size(WORK_SIZE.x as _, WORK_SIZE.y as _)
        .min_size((WORK_SIZE.x * 0.5) as _, (WORK_SIZE.y * 0.5) as _)
        .resizable(true)
        .vsync(true)
        .title("{{ project-name }} - Notan Game")
}

pub fn initialize() -> Result<(), String> {
    notan::init_with(|| State {})
        .add_config(DrawConfig)
        .add_config(window_config())
        .add_plugin(FpsLimit::new(LIMIT_MAX_FPS))
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
    let (width, height) = gfx.size();
    let win_size = vec2(width as f32, height as f32);

    // get the projection that will fit and center our content in the screen
    let (projection, _) = calc_projection(win_size, WORK_SIZE);

    let mut draw = gfx.create_draw();
    draw.set_projection(Some(projection));
    draw.clear(Color::BLACK);
    draw.rect((100.0, 100.0), (600.0, 400.0));

    gfx.render(&draw);
}

// This returns a projection that keeps the aspect ratio while scaling
// and fitting the content in our window
// It also returns the ratio in case we need it to calculate positions
// or manually scale something
fn calc_projection(win_size: Vec2, work_size: Vec2) -> (Mat4, f32) {
    let ratio = (win_size.x / work_size.x).min(win_size.y / work_size.y);

    let projection = Mat4::orthographic_rh_gl(0.0, win_size.x, win_size.y, 0.0, -1.0, 1.0);
    let scale = Mat4::from_scale(vec3(ratio, ratio, 1.0));
    let position = vec3(
        (win_size.x - work_size.x * ratio) * 0.5,
        (win_size.y - work_size.y * ratio) * 0.5,
        1.0,
    );
    let translation = Mat4::from_translation(position);
    (projection * translation * scale, ratio)
}
