use glam::Vec3A;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct BallState {
    pub pos: Vec3A,
    pub vel: Vec3A,
    pub ang_vel: Vec3A,
}

impl BallState {
    pub fn new() -> Self {
        Self::default()
    }
}
