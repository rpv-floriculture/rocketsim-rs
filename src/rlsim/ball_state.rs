use vector3::Vector3;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct BallState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
    pub ang_vel_x: f32,
    pub ang_vel_y: f32,
    pub ang_vel_z: f32,
}

impl BallState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_pos(&self) -> Vector3 {
        Vector3::new(self.pos_x.into(), self.pos_y.into(), self.pos_z.into())
    }

    pub fn get_vel(&self) -> Vector3 {
        Vector3::new(self.vel_x.into(), self.vel_y.into(), self.vel_z.into())
    }

    pub fn get_ang_vel(&self) -> Vector3 {
        Vector3::new(self.ang_vel_x.into(), self.ang_vel_y.into(), self.ang_vel_z.into())
    }
}