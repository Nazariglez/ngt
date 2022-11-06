use gotan::draw::Draw;
use notan::math::{vec2, vec3, Mat3, Mat4, Vec2};

pub struct Camera2D {
    win_size: Vec2,
    work_size: Vec2,
    pos: Vec2,
    projection: Mat4,
    ratio: f32,
    transform: Mat3,
    dirty: bool,
}

impl Camera2D {
    pub fn new(width: f32, height: f32) -> Self {
        let size = vec2(width, height);
        let pos = Vec2::splat(0.0);
        let projection = Mat4::IDENTITY;
        let ratio = 1.0;
        let transform = Mat3::IDENTITY;

        Self {
            work_size: size,
            win_size: size,
            pos,
            projection,
            ratio,
            transform,
            dirty: true,
        }
    }

    pub fn apply(&mut self, draw: &mut Draw) {
        self.set_win_size(draw.size().into());

        if self.dirty {
            self.dirty = false;
            self.calculate_projection();
            self.calculate_transform();
        }

        draw.set_projection(Some(self.projection));
        draw.transform().push(self.transform);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let pos = vec2(x, y);
        if self.pos != pos {
            self.dirty = true;
            self.pos = pos;
        }
    }

    pub fn position(&self) -> Vec2 {
        self.pos
    }

    pub fn set_position_to_center(&mut self) {
        self.set_position(self.work_size.x * 0.5, self.work_size. y * 0.5);
    }

    fn calculate_projection(&mut self) {
        let (projection, ratio) = calc_projection(self.win_size, self.work_size);
        self.projection = projection;
        self.ratio = ratio;
    }

    fn calculate_transform(&mut self) {
        let pos = self.pos - self.work_size * 0.5;
        self.transform = Mat3::from_translation(pos * -1.0);
    }

    fn set_win_size(&mut self, size: Vec2) {
        if self.win_size != size {
            self.dirty = true;
            self.win_size = size;
        }
    }
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
