use glam::{Mat3A, Vec3A};

use super::Team;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct CarState {
    pub team: Team,
    pub id: u32,
    pub pos: Vec3A,
    pub rot: Mat3A,
    pub vel: Vec3A,
    pub ang_vel: Vec3A,
    pub is_on_ground: bool,
    pub has_jumped: bool,
    pub has_double_jumped: bool,
    pub has_flipped: bool,
    pub last_rel_dodge_torque: Vec3A,
    pub jump_timer: f32,
    pub flip_timer: f32,
    pub is_jumping: bool,
    pub air_time_since_jump: f32,
    pub boost: f32,
    pub time_spent_boosting: f32,
    pub is_supersonic: bool,
    pub supersonic_time: f32,
    pub handbrake_val: f32,
    pub is_auto_flipping: bool,
    pub auto_flip_timer: f32,
    pub auto_flip_torque_scale: f32,
    pub has_contact: bool,
    pub contact_normal: Vec3A,
}

impl CarState {
    pub fn new(team: Team) -> Self {
        let mut car_state = Self::default();
        car_state.team = team;
        car_state
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

    pub fn with_pos(mut self, pos: Vec3A) -> Self {
        self.car_state.pos = pos;
        self
    }
    pub fn with_vel(mut self, vel: Vec3A) -> Self {
        self.car_state.vel = vel;
        self
    }

    pub fn with_ang_vel(mut self, ang_vel: Vec3A) -> Self {
        self.car_state.ang_vel = ang_vel;
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

    pub fn with_last_rel_dodge_torque(mut self, last_rel_dodge_torque: Vec3A) -> Self {
        self.car_state.last_rel_dodge_torque = last_rel_dodge_torque;
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
