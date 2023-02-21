use glam::Vec3;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct BallState {
    pub pos: Vec3,
    pub vel: Vec3,
    pub ang_vel: Vec3,
}

impl BallState {
    pub fn new() -> Self {
        Self::default()
    }
}
