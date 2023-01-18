use vector3::Vector3;

use super::Team;


#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct CarState {
    pub team: Team,
    pub id: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
    pub ang_vel_x: f32,
    pub ang_vel_y: f32,
    pub ang_vel_z: f32,
    pub is_on_ground: bool,
    pub has_jumped: bool,
    pub has_double_jumped: bool,
    pub has_flipped: bool,
    pub last_rel_dodge_torque_x: f32,
    pub last_rel_dodge_torque_y: f32,
    pub last_rel_dodge_torque_z: f32,
    pub jump_timer: f32,
    pub flip_timer: f32,
    pub is_jumping: bool,
    pub air_time_since_jump: f32,
    pub boost: f32,
    pub is_supersonic: bool,
    pub handbrake_val: f32,
}

impl CarState {
    pub fn new(team: Team) -> Self {
        let mut car_state = Self::default();
        car_state.team = team;
        car_state
    }

    pub fn get_pos(&self) -> Vector3 {
        Vector3::new(self.pos_x.into(), self.pos_y.into(), self.pos_z.into())
    }

    pub fn get_rot(&self) -> Vector3 {
        Vector3::new(self.yaw.into(), self.pitch.into(), self.roll.into())
    }

    pub fn get_vel(&self) -> Vector3 {
        Vector3::new(self.vel_x.into(), self.vel_y.into(), self.vel_z.into())
    }

    pub fn get_ang_vel(&self) -> Vector3 {
        Vector3::new(self.ang_vel_x.into(), self.ang_vel_y.into(), self.ang_vel_z.into())
    }

    pub fn get_last_rel_dodge_torque(&self) -> Vector3 {
        Vector3::new(self.last_rel_dodge_torque_x.into(), self.last_rel_dodge_torque_y.into(), self.last_rel_dodge_torque_z.into())
    }
}

pub struct CarStateBuilder {
    car_state: CarState,
}

impl CarStateBuilder {
    pub fn new(team: Team) -> Self {
        Self {
            car_state: CarState::new(team),
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.car_state.id = id;
        self
    }

    pub fn with_pos(mut self, pos: Vector3) -> Self {
        self.car_state.pos_x = pos.x as f32;
        self.car_state.pos_y = pos.y as f32;
        self.car_state.pos_z = pos.z as f32;
        self
    }

    pub fn with_pos_x(mut self, pos_x: f32) -> Self {
        self.car_state.pos_x = pos_x;
        self
    }

    pub fn with_pos_y(mut self, pos_y: f32) -> Self {
        self.car_state.pos_y = pos_y;
        self
    }

    pub fn with_pos_z(mut self, pos_z: f32) -> Self {
        self.car_state.pos_z = pos_z;
        self
    }

    pub fn with_rot(mut self, rot: Vector3) -> Self {
        self.car_state.yaw = rot.x as f32;
        self.car_state.pitch = rot.y as f32;
        self.car_state.roll = rot.z as f32;
        self
    }

    pub fn with_yaw(mut self, yaw: f32) -> Self {
        self.car_state.yaw = yaw;
        self
    }

    pub fn with_pitch(mut self, pitch: f32) -> Self {
        self.car_state.pitch = pitch;
        self
    }

    pub fn with_roll(mut self, roll: f32) -> Self {
        self.car_state.roll = roll;
        self
    }

    pub fn with_vel(mut self, vel: Vector3) -> Self {
        self.car_state.vel_x = vel.x as f32;
        self.car_state.vel_y = vel.y as f32;
        self.car_state.vel_z = vel.z as f32;
        self
    }

    pub fn with_vel_x(mut self, vel_x: f32) -> Self {
        self.car_state.vel_x = vel_x;
        self
    }

    pub fn with_vel_y(mut self, vel_y: f32) -> Self {
        self.car_state.vel_y = vel_y;
        self
    }

    pub fn with_vel_z(mut self, vel_z: f32) -> Self {
        self.car_state.vel_z = vel_z;
        self
    }

    pub fn with_ang_vel(mut self, ang_vel: Vector3) -> Self {
        self.car_state.ang_vel_x = ang_vel.x as f32;
        self.car_state.ang_vel_y = ang_vel.y as f32;
        self.car_state.ang_vel_z = ang_vel.z as f32;
        self
    }

    pub fn with_ang_vel_x(mut self, ang_vel_x: f32) -> Self {
        self.car_state.ang_vel_x = ang_vel_x;
        self
    }

    pub fn with_ang_vel_y(mut self, ang_vel_y: f32) -> Self {
        self.car_state.ang_vel_y = ang_vel_y;
        self
    }

    pub fn with_ang_vel_z(mut self, ang_vel_z: f32) -> Self {
        self.car_state.ang_vel_z = ang_vel_z;
        self
    }

    pub fn with_is_on_ground(mut self, is_on_ground: bool) -> Self {
        self.car_state.is_on_ground = is_on_ground;
        self
    }

    pub fn with_has_jumped(mut self, has_jumped: bool) -> Self {
        self.car_state.has_jumped = has_jumped;
        self
    }

    pub fn with_has_double_jumped(mut self, has_double_jumped: bool) -> Self {
        self.car_state.has_double_jumped = has_double_jumped;
        self
    }

    pub fn with_has_flipped(mut self, has_flipped: bool) -> Self {
        self.car_state.has_flipped = has_flipped;
        self
    }

    pub fn with_last_rel_dodge_torque(mut self, last_rel_dodge_torque: Vector3) -> Self {
        self.car_state.last_rel_dodge_torque_x = last_rel_dodge_torque.x as f32;
        self.car_state.last_rel_dodge_torque_y = last_rel_dodge_torque.y as f32;
        self.car_state.last_rel_dodge_torque_z = last_rel_dodge_torque.z as f32;
        self
    }

    pub fn with_last_rel_dodge_torque_x(mut self, last_rel_dodge_torque_x: f32) -> Self {
        self.car_state.last_rel_dodge_torque_x = last_rel_dodge_torque_x;
        self
    }

    pub fn with_last_rel_dodge_torque_y(mut self, last_rel_dodge_torque_y: f32) -> Self {
        self.car_state.last_rel_dodge_torque_y = last_rel_dodge_torque_y;
        self
    }

    pub fn with_last_rel_dodge_torque_z(mut self, last_rel_dodge_torque_z: f32) -> Self {
        self.car_state.last_rel_dodge_torque_z = last_rel_dodge_torque_z;
        self
    }

    pub fn with_jump_timer(mut self, jump_timer: f32) -> Self {
        self.car_state.jump_timer = jump_timer;
        self
    }

    pub fn with_flip_timer(mut self, flip_timer: f32) -> Self {
        self.car_state.flip_timer = flip_timer;
        self
    }

    pub fn with_is_jumping(mut self, is_jumping: bool) -> Self {
        self.car_state.is_jumping = is_jumping;
        self
    }

    pub fn with_air_time_since_jump(mut self, air_time_since_jump: f32) -> Self {
        self.car_state.air_time_since_jump = air_time_since_jump;
        self
    }

    pub fn with_boost(mut self, boost: f32) -> Self {
        self.car_state.boost = boost;
        self
    }

    pub fn with_is_supersonic(mut self, is_supersonic: bool) -> Self {
        self.car_state.is_supersonic = is_supersonic;
        self
    }

    pub fn with_handbrake_val(mut self, handbrake_val: f32) -> Self {
        self.car_state.handbrake_val = handbrake_val;
        self
    }

    pub fn build(self) -> CarState {
        self.car_state
    }
}